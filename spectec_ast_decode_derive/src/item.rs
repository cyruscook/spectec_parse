use crate::utils::{get_attr, syn_throw, syn_try};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use synstructure::Structure;

fn process_atom(
    s_name: &syn::Ident,
    atom_decoders: &mut TokenStream,
    atom_takes_any_name: &mut Vec<TokenStream>,
    item_attr: &syn::Attribute,
    variant_name: &syn::Ident,
    variant_fields: &syn::Fields,
) -> Result<(), syn::Error> {
    let item_name: Option<syn::Expr> =
        item_attr.parse_args_with(|parser: syn::parse::ParseStream| {
            Ok(if parser.is_empty() {
                None
            } else {
                syn::custom_keyword!(name);
                parser.parse::<name>()?;
                parser.parse::<syn::Token![=]>()?;
                Some(parser.parse::<syn::Expr>()?)
            })
        })?;
    match variant_fields {
        syn::Fields::Unit => {
            if let Some(item_name) = item_name {
                (quote!(
                    #item_name => return Ok(#s_name::#variant_name),
                ))
                .to_tokens(atom_decoders);
                Ok(())
            } else {
                Err(syn::Error::new_spanned(
                    variant_fields,
                    "Unit atom variants must have a name specified",
                ))
            }
        }
        syn::Fields::Unnamed(unnamed) => {
            if let Some(item_name) = item_name {
                Err(syn::Error::new_spanned(
                    item_name,
                    "Atom variants with unnamed fields must not have a name",
                ))
            } else if unnamed.unnamed.len() != 1 {
                Err(syn::Error::new_spanned(
                    unnamed,
                    "Atom variant with unnamed fields must only have one",
                ))
            } else {
                let ftype = &unnamed.unnamed.get(0).unwrap().ty;
                atom_takes_any_name.push(quote!(
                    if let Ok(out) = <#ftype as decode::Decode>::decode(&mut std::iter::once(&item).peekable()) {
                        return Ok(#s_name::#variant_name(out));
                    }
                ));
                Ok(())
            }
        }
        syn::Fields::Named(named) => Err(syn::Error::new_spanned(
            named,
            "Atoms cannot have named fields",
        )),
    }
}
fn process_node(
    s_name: &syn::Ident,
    node_decoders: &mut TokenStream,
    item_attr: &syn::Attribute,
    variant_name: &syn::Ident,
    variant_fields: &syn::Fields,
) -> Result<(), syn::Error> {
    let variant_name_str = variant_name.to_token_stream().to_string();
    let item_name: syn::Expr = item_attr.parse_args_with(|parser: syn::parse::ParseStream| {
        syn::custom_keyword!(name);
        parser.parse::<name>()?;
        parser.parse::<syn::Token![=]>()?;
        parser.parse::<syn::Expr>()
    })?;
    match variant_fields {
        syn::Fields::Unit => {
            (quote!(
                #item_name => {
                    // There should be no items for a unit variant
                    if let Some(i) = items.into_iter().next() {
                        return Err(decode::Error::unparsed_sexpr::<#s_name>(
                            i,
                        ).with_variant(#variant_name_str));
                    }
                    return Ok(#s_name::#variant_name)
                },
            ))
            .to_tokens(node_decoders);
            Ok(())
        }
        syn::Fields::Named(named) => {
            let mut field_parses = quote!();
            for f in &named.named {
                let fname = f.ident.as_ref().unwrap();
                let fname_str = fname.to_token_stream().to_string();
                let ftype = &f.ty;
                (quote! (
                    let #fname = <#ftype as decode::Decode>::decode(&mut items).map_err(|e|
                        decode::Error::wrapped::<#s_name>(e).with_variant(#variant_name_str).with_field(#fname_str)
                    )?;
                ))
                .to_tokens(&mut field_parses);
            }
            let field_names = named.named.iter().map(|f| f.ident.as_ref().unwrap());
            (quote!(
                #item_name => {
                    let mut items = items.into_iter().peekable();
                    #field_parses
                    let out = #s_name::#variant_name {
                        #(
                            #field_names,
                        )*
                    };
                    // We should have consumed all the items
                    if let Some(i) = items.next() {
                        return Err(decode::Error::unparsed_sexpr::<#s_name>(
                            i,
                        ).with_variant(#variant_name_str));
                    }
                    return Ok(out)
                },
            ))
            .to_tokens(node_decoders);
            Ok(())
        }
        syn::Fields::Unnamed(unnamed) => {
            let mut field_parses = quote!();
            for f in &unnamed.unnamed {
                let ftype = &f.ty;
                (quote! (
                    <#ftype as decode::Decode>::decode(&mut items).map_err(|e|
                        decode::Error::wrapped::<#s_name>(e).with_variant(#variant_name_str)
                    )?,
                ))
                .to_tokens(&mut field_parses);
            }
            (quote!(
                #item_name => {
                    let mut items = items.into_iter().peekable();
                    let out = #s_name::#variant_name (
                        #field_parses
                    );
                    // We should have consumed all the items
                    if let Some(i) = items.next() {
                        return Err(decode::Error::unparsed_sexpr::<#s_name>(
                            i,
                        ).with_variant(#variant_name_str));
                    }
                    return Ok(out)
                },
            ))
            .to_tokens(node_decoders);
            Ok(())
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn sexpr_decode_derive(s: Structure) -> proc_macro2::TokenStream {
    let s_name = s.ast().ident.clone();
    let decode = match s.ast().data {
        syn::Data::Enum(_) => {
            let mut atom_takes_any_name = Vec::new();
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
                if let Some(item_attr) = syn_try!(get_attr("sexpr_atom", v.ast().attrs)) {
                    syn_try!(process_atom(
                        &s_name,
                        &mut atom_decoders,
                        &mut atom_takes_any_name,
                        item_attr,
                        variant_name,
                        v.ast().fields,
                    ));
                } else if let Some(item_attr) = syn_try!(get_attr("sexpr_node", v.ast().attrs)) {
                    syn_try!(process_node(
                        &s_name,
                        &mut node_decoders,
                        item_attr,
                        variant_name,
                        v.ast().fields,
                    ));
                } else {
                    syn_throw!(syn::Error::new_spanned(
                        variant_name,
                        "Must have either a sexpr_atom or sexpr_item attribute"
                    ));
                }
            }

            let atom_decoders = if atom_takes_any_name.is_empty() {
                quote!(
                    #atom_decoders
                    name => return Err(decode::Error::unrecognised_atom_symbol::<#s_name, _>(name)),
                )
            } else {
                let any_decoders = atom_takes_any_name.iter();
                quote!(
                    #atom_decoders
                    name => {
                        let item = sexpr_parse::SExprItem::Atom(name.to_owned());
                        #( #any_decoders )*
                        return Err(decode::Error::unrecognised_atom_symbol::<#s_name, _>(name));
                    }
                )
            };
            let node_decoders = quote!(
                #node_decoders
                _ => return Err(decode::Error::unrecognised_node_symbol::<#s_name, _>(name)),
            );

            quote! {
                gen impl decode::Decode for @Self {
                    fn decode<'a, I: Iterator<Item = &'a sexpr_parse::SExprItem>>(
                        items: &mut std::iter::Peekable<I>,
                    ) -> decode::Result<#s_name> {
                        match items.next() {
                            Some(sexpr_parse::SExprItem::Atom(name)) => match name.as_str() {
                                #atom_decoders
                            },
                            Some(sexpr_parse::SExprItem::Node(name, items)) => match name.as_str() {
                                #node_decoders
                            },
                            Some(item) => return Err(decode::Error::cannot_decode_sexpr::<#s_name>(item)),
                            None => return Err(decode::Error::required_missing_sexpr::<#s_name>()),
                        }
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
