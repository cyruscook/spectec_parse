use crate::utils::{get_attr, syn_throw, syn_try};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use synstructure::Structure;

fn process_atom(
    atom_checkers: &mut TokenStream,
    atom_decoders: &mut TokenStream,
    item_attr: &syn::Attribute,
    variant_name: &syn::Ident,
    variant_fields: &syn::Fields,
) -> Result<(), syn::Error> {
    let item_name: syn::Expr = item_attr.parse_args_with(|parser: syn::parse::ParseStream| {
        syn::custom_keyword!(name);
        parser.parse::<name>()?;
        parser.parse::<syn::Token![=]>()?;
        parser.parse::<syn::Expr>()
    })?;
    (quote!(
        #item_name => true,
    ))
    .to_tokens(atom_checkers);
    match variant_fields {
        syn::Fields::Unit => {
            (quote!(
                #item_name => Self::#variant_name,
            ))
            .to_tokens(atom_decoders);
            Ok(())
        }
        _ => Err(syn::Error::new_spanned(
            variant_name,
            "Atoms cannot have any fields",
        )),
    }
}
fn process_node(
    node_checkers: &mut TokenStream,
    node_decoders: &mut TokenStream,
    item_attr: &syn::Attribute,
    variant_name: &syn::Ident,
    variant_fields: &syn::Fields,
) -> Result<(), syn::Error> {
    let item_name: syn::Expr = item_attr.parse_args_with(|parser: syn::parse::ParseStream| {
        syn::custom_keyword!(name);
        parser.parse::<name>()?;
        parser.parse::<syn::Token![=]>()?;
        parser.parse::<syn::Expr>()
    })?;
    (quote!(
        #item_name => true,
    ))
    .to_tokens(node_checkers);
    match variant_fields {
        syn::Fields::Unit => {
            (quote!(
                #item_name => {
                    // There should be no items for a unit variant
                    if let Some(i) = items.into_iter().next() {
                        return Err(crate::decode::DecodeError::UnexpectedItem(
                            i,
                        ));
                    }
                    Self::#variant_name
                },
            ))
            .to_tokens(node_decoders);
            Ok(())
        }
        syn::Fields::Named(named) => {
            let mut field_parses = quote!();
            for f in &named.named {
                let fname = f.ident.as_ref().unwrap();
                let ftype = &f.ty;
                let is_vec = if let Some(item_attr) = get_attr("spectec_field", &f.attrs)? {
                    item_attr.parse_args_with(|parser: syn::parse::ParseStream| {
                        syn::custom_keyword!(vec);
                        parser.parse::<vec>()?;
                        parser.parse::<syn::Token![=]>()?;
                        parser.parse::<syn::Expr>()
                    })? == syn::parse_str::<syn::Expr>("true")?
                } else {
                    false
                };
                if is_vec {
                    (quote! (
                        let #fname = crate::decode::decode_iter::<#ftype, _, _>(&mut items)?;
                    ))
                    .to_tokens(&mut field_parses);
                } else {
                    (quote! (
                        let #fname = <#ftype as crate::decode::Decode>::decode(items.next().unwrap())?;
                    ))
                    .to_tokens(&mut field_parses);
                }
            }
            let field_names = named.named.iter().map(|f| f.ident.as_ref().unwrap());
            (quote!(
                #item_name => {
                    let mut items = items.into_iter().peekable();
                    #field_parses
                    // We should have consumed all the items
                    if let Some(i) = items.next() {
                        return Err(crate::decode::DecodeError::UnexpectedItem(
                            i,
                        ));
                    }
                    Self::#variant_name {
                        #(
                            #field_names,
                        )*
                    }
                },
            ))
            .to_tokens(node_decoders);
            Ok(())
        }
        syn::Fields::Unnamed(_) => todo!("Unnamed fields not supported yet"),
    }
}

pub(crate) fn spectec_item_derive(s: Structure) -> proc_macro2::TokenStream {
    let decode = match s.ast().data {
        syn::Data::Enum(_) => {
            let mut atom_checkers = quote!();
            let mut node_checkers = quote!();
            let mut atom_decoders = quote!();
            let mut node_decoders = quote!();

            for v in s.variants() {
                if let Some(disc) = v.ast().discriminant {
                    syn_throw!(syn::Error::new_spanned(
                        &disc.1,
                        "Must not have a discriminant"
                    ));
                }
                let variant_name = v.ast().ident;
                if let Some(item_attr) = syn_try!(get_attr("spectec_atom", v.ast().attrs)) {
                    syn_try!(process_atom(
                        &mut atom_checkers,
                        &mut atom_decoders,
                        item_attr,
                        &variant_name,
                        &v.ast().fields,
                    ));
                } else if let Some(item_attr) = syn_try!(get_attr("spectec_node", v.ast().attrs)) {
                    syn_try!(process_node(
                        &mut node_checkers,
                        &mut node_decoders,
                        item_attr,
                        &variant_name,
                        &v.ast().fields,
                    ));
                } else {
                    syn_throw!(syn::Error::new_spanned(
                        variant_name,
                        "Must have either a spectec_atom or spectec_item attribute"
                    ));
                };
            }

            quote! {
                gen impl crate::decode::Decode for @Self {
                    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
                        match item {
                            crate::sexpr::SExprItem::Atom(name) => match name.as_str() {
                                #atom_checkers
                                _ => false,
                            },
                            crate::sexpr::SExprItem::Node(name, _) => match name.as_str() {
                                #node_checkers
                                _ => false,
                            },
                            _ => false,
                        }
                    }
                    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, crate::decode::DecodeError> {
                        Ok(match item {
                            crate::sexpr::SExprItem::Atom(name) => match name.as_str() {
                                #atom_decoders
                                _ => return Err(crate::decode::DecodeError::UnrecognisedSymbol(name)),
                            },
                            crate::sexpr::SExprItem::Node(name, items) => match name.as_str() {
                                #node_decoders
                                _ => return Err(crate::decode::DecodeError::UnrecognisedSymbol(name)),
                            },
                            _ => return Err(crate::decode::DecodeError::UnexpectedItem(item)),
                        })
                    }
                }
            }
        }
        _ => {
            syn_throw!(syn::Error::new_spanned(&s.ast().ident, "Unsupported data"))
        }
    };

    s.gen_impl(decode)
}
