use crate::utils::{check_spectec_field_attr, get_attr, syn_throw, syn_try};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use synstructure::Structure;

fn process_atom(
    s_name: &syn::Ident,
    atom_checkers: &mut TokenStream,
    atom_decoders: &mut TokenStream,
    atom_takes_any_name: &mut Vec<(TokenStream, TokenStream)>,
    item_attr: &syn::Attribute,
    variant_name: &syn::Ident,
    variant_fields: &syn::Fields,
) -> Result<(), syn::Error> {
    let variant_name_str = variant_name.to_token_stream().to_string();
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
    if let Some(item_name) = &item_name {
        quote!(
            #item_name => true,
        )
        .to_tokens(atom_checkers);
    }
    match variant_fields {
        syn::Fields::Unit => {
            if let Some(item_name) = item_name {
                (quote!(
                    #item_name => #s_name::#variant_name,
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
                atom_takes_any_name.push((
                    quote!(
                        #ftype::can_decode(&item) ||
                    ),
                    quote!(
                        if #ftype::can_decode(&item) {
                            #ftype::decode(item)
                                .map(#s_name::#variant_name)
                                .map_err(|e| {
                                    e.with_context(concat!("decoding variant ", #variant_name_str, " field 0"))
                                })
                        } else
                    ),
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
    node_checkers: &mut TokenStream,
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
                    items.len() == 0
                },
            ))
            .to_tokens(node_checkers);

            (quote!(
                #item_name => {
                    // There should be no items for a unit variant
                    if let Some(i) = items.into_iter().next() {
                        return Err(crate::decode::DecodeError::UnexpectedItem(
                            i,
                        ).with_context(concat!("decoding variant ", #variant_name_str, " unit type should have no items")));
                    }
                    #s_name::#variant_name
                },
            ))
            .to_tokens(node_decoders);
            Ok(())
        }
        syn::Fields::Named(named) => {
            let mut field_checks: Vec<TokenStream> = Vec::new();
            let mut field_parses = quote!();
            for f in &named.named {
                let fname = f.ident.as_ref().unwrap();
                let fname_str = fname.to_token_stream().to_string();
                let ftype = &f.ty;
                let (is_vec, is_option) = check_spectec_field_attr(&f.attrs)?;
                if is_vec {
                    field_checks.push(quote!(
                        crate::decode::can_decode_iter::<#ftype, _, _>(&mut items)
                    ));
                    (quote! (
                        let #fname = crate::decode::decode_iter::<#ftype, _, _>(&mut items).map_err(|e| {
                            e.with_context(concat!("decoding variant ", #variant_name_str, " field ", #fname_str))
                        })?;
                    ))
                    .to_tokens(&mut field_parses);
                } else if is_option {
                    field_checks.push(quote!(
                        crate::decode::can_decode_option::<#ftype, _, _>(&mut items)
                    ));
                    (quote! (
                        let #fname = crate::decode::decode_option::<#ftype, _, _>(&mut items).map_err(|e| {
                            e.with_context(concat!("decoding variant ", #variant_name_str, " field ", #fname_str))
                        })?;
                    ))
                    .to_tokens(&mut field_parses);
                } else {
                    field_checks.push(quote!(
                        items.next()
                            .map(<#ftype as crate::decode::Decode>::can_decode)
                        == Some(true)
                    ));
                    (quote! (
                        let #fname = items.next()
                            .ok_or_else(|| crate::decode::DecodeError::MissingItem)
                            .map(<#ftype as crate::decode::Decode>::decode)
                            .flatten()
                            .map_err(|e| e.with_context(concat!("decoding variant ", #variant_name_str, " field ", #fname_str)))?;
                    ))
                    .to_tokens(&mut field_parses);
                }
            }
            let field_names = named.named.iter().map(|f| f.ident.as_ref().unwrap());
            (quote!(
                #item_name => {
                    let mut items = items.iter().peekable();
                    (
                        #(
                            #field_checks
                            &&
                        )*
                        // There should be no more items
                        items.next().is_none()
                    )
                },
            ))
            .to_tokens(node_checkers);
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
                        return Err(crate::decode::DecodeError::UnexpectedItem(
                            i,
                        ).with_context(format!(concat!("decoding variant ", #variant_name_str, " item not consumed by fields ({:?})"), out)));
                    }
                    out
                },
            ))
            .to_tokens(node_decoders);
            Ok(())
        }
        syn::Fields::Unnamed(unnamed) => {
            let mut field_checks: Vec<TokenStream> = Vec::new();
            let mut field_parses = quote!();
            for f in &unnamed.unnamed {
                let ftype = &f.ty;
                let (is_vec, is_option) = check_spectec_field_attr(&f.attrs)?;
                if is_vec {
                    field_checks.push(quote!(
                        crate::decode::can_decode_iter::<#ftype, _, _>(&mut items)
                    ));
                    (quote! (
                        crate::decode::decode_iter::<#ftype, _, _>(&mut items).map_err(|e| {
                            e.with_context(concat!("decoding variant ", #variant_name_str))
                        })?,
                    ))
                    .to_tokens(&mut field_parses);
                } else if is_option {
                    field_checks.push(quote!(
                        crate::decode::can_decode_option::<#ftype, _, _>(&mut items)
                    ));
                    (quote! (
                        crate::decode::decode_option::<#ftype, _, _>(&mut items).map_err(|e| {
                            e.with_context(concat!("decoding variant ", #variant_name_str))
                        })?,
                    ))
                    .to_tokens(&mut field_parses);
                } else {
                    field_checks.push(quote!(
                        items.next()
                            .map(<#ftype as crate::decode::Decode>::can_decode)
                        == Some(true)
                    ));
                    (quote! (
                        items.next()
                            .ok_or_else(|| crate::decode::DecodeError::MissingItem)
                            .map(<#ftype as crate::decode::Decode>::decode)
                            .flatten()
                            .map_err(|e| e.with_context(concat!("decoding variant ", #variant_name_str)))?,
                    ))
                    .to_tokens(&mut field_parses);
                }
            }
            (quote!(
                #item_name => {
                    let mut items = items.iter().peekable();
                    (
                        #(
                            #field_checks
                            &&
                        )*
                        // There should be no more items
                        items.next().is_none()
                    )
                },
            ))
            .to_tokens(node_checkers);
            (quote!(
                #item_name => {
                    let mut items = items.into_iter().peekable();
                    let out = #s_name::#variant_name (
                        #field_parses
                    );
                    // We should have consumed all the items
                    if let Some(i) = items.next() {
                        return Err(crate::decode::DecodeError::UnexpectedItem(
                            i,
                        ).with_context(format!(concat!("decoding variant ", #variant_name_str, " item not consumed by fields ({:?})"), out)));
                    }
                    out
                },
            ))
            .to_tokens(node_decoders);
            Ok(())
        }
    }
}

pub(crate) fn spectec_item_derive(s: Structure) -> proc_macro2::TokenStream {
    let s_name = s.ast().ident.clone();
    let decode = match s.ast().data {
        syn::Data::Enum(_) => {
            let mut atom_checkers = quote!();
            let mut node_checkers = quote!();
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
                if let Some(item_attr) = syn_try!(get_attr("spectec_atom", v.ast().attrs)) {
                    syn_try!(process_atom(
                        &s_name,
                        &mut atom_checkers,
                        &mut atom_decoders,
                        &mut atom_takes_any_name,
                        item_attr,
                        variant_name,
                        v.ast().fields,
                    ));
                } else if let Some(item_attr) = syn_try!(get_attr("spectec_node", v.ast().attrs)) {
                    syn_try!(process_node(
                        &s_name,
                        &mut node_checkers,
                        &mut node_decoders,
                        item_attr,
                        variant_name,
                        v.ast().fields,
                    ));
                } else {
                    syn_throw!(syn::Error::new_spanned(
                        variant_name,
                        "Must have either a spectec_atom or spectec_item attribute"
                    ));
                };
            }

            let atom_checkers = if !atom_takes_any_name.is_empty() {
                let any_checkers = atom_takes_any_name.iter().map(|q| &q.0);
                quote!(
                    #atom_checkers
                    _ => #( #any_checkers )* false
                )
            } else {
                quote!(
                    #atom_checkers
                    _ => false,
                )
            };
            let node_checkers = quote!(
                #node_checkers
                _ => false,
            );

            let atom_decoders = if !atom_takes_any_name.is_empty() {
                let any_decoders = atom_takes_any_name.iter().map(|q| &q.1);
                quote!(
                    #atom_decoders
                    _ => {
                        let item = crate::sexpr::SExprItem::Atom(name.clone());
                        #( #any_decoders )* { Err(crate::decode::DecodeError::UnrecognisedSymbol(name)) }?
                    }
                )
            } else {
                quote!(
                    #atom_decoders
                    _ => return Err(crate::decode::DecodeError::UnrecognisedSymbol(name)),
                )
            };
            let node_decoders = quote!(
                #node_decoders
                _ => return Err(crate::decode::DecodeError::UnrecognisedSymbol(name)),
            );

            quote! {
                gen impl crate::decode::Decode for @Self {
                    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
                        match item {
                            crate::sexpr::SExprItem::Atom(name) => match name.as_str() {
                                #atom_checkers
                            },
                            crate::sexpr::SExprItem::Node(name, items) => match name.as_str() {
                                #node_checkers
                            },
                            _ => false,
                        }
                    }
                    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, crate::decode::DecodeError> {
                        fn impl_decode(item: crate::sexpr::SExprItem) -> Result<#s_name, crate::decode::DecodeError> {
                            Ok(match item {
                                crate::sexpr::SExprItem::Atom(name) => match name.as_str() {
                                    #atom_decoders
                                },
                                crate::sexpr::SExprItem::Node(name, items) => match name.as_str() {
                                    #node_decoders
                                },
                                _ => return Err(crate::decode::DecodeError::UnexpectedItem(item)),
                            })
                        }

                        impl_decode(item)
                            .map_err(|e| e.with_context(format!("while decoding {}", std::any::type_name::<Self>())))
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
