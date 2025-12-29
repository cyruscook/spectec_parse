extern crate proc_macro;

use quote::{ToTokens, quote};
use syn::Expr;
use synstructure::{Structure, decl_derive};

macro_rules! syn_throw {
    ($err:expr) => {
        return syn::Error::to_compile_error(&$err)
    };
}

macro_rules! syn_try {
    ($expr:expr) => {
        match $expr {
            Ok(expr) => expr,
            Err(err) => syn_throw!(err),
        }
    };
}

fn get_attr<'a>(name: &str, attrs: &'a [syn::Attribute]) -> Option<&'a syn::Attribute> {
    let mut matching_attrs = attrs.iter().filter(|attr| attr.meta.path().is_ident(name));
    match matching_attrs.next() {
        None => None,
        Some(first) => {
            // Check this is the only matching one
            if matching_attrs.next().is_none() {
                Some(first)
            } else {
                None
            }
        }
    }
}

pub(crate) fn spectec_item_derive(s: Structure) -> proc_macro2::TokenStream {
    let decode = match s.ast().data {
        syn::Data::Enum(_) => {
            let mut allowed_names = quote!();
            let mut decoders = quote!();

            for v in s.variants() {
                if let Some(disc) = v.ast().discriminant {
                    syn_throw!(syn::Error::new_spanned(
                        &disc.1,
                        "Must not have a discriminant"
                    ));
                }
                let variant_name = v.ast().ident;
                let item_attr = if let Some(attr) = get_attr("spectec_item", v.ast().attrs) {
                    attr
                } else {
                    syn_throw!(syn::Error::new_spanned(
                        variant_name,
                        "Must have a spectec_item attribute"
                    ));
                };
                let item_name: Expr = syn_try!(item_attr.parse_args_with(
                    |parser: syn::parse::ParseStream| {
                        syn::custom_keyword!(name);
                        parser.parse::<name>()?;
                        parser.parse::<syn::Token![=]>()?;
                        parser.parse::<Expr>()
                    }
                ));
                (quote!(
                    #item_name => true,
                ))
                .to_tokens(&mut allowed_names);
                match v.ast().fields {
                    syn::Fields::Unit => {
                        (quote!(
                            #item_name => Self::#variant_name,
                        ))
                        .to_tokens(&mut decoders);
                    }
                    syn::Fields::Named(named) => {
                        let mut field_parses = quote!();
                        for f in &named.named {
                            let fname = f.ident.as_ref().unwrap();
                            let ftype = &f.ty;
                            let is_vec =
                                if let Some(item_attr) = get_attr("spectec_field", &f.attrs) {
                                    syn_try!(item_attr.parse_args_with(
                                        |parser: syn::parse::ParseStream| {
                                            syn::custom_keyword!(vec);
                                            parser.parse::<vec>()?;
                                            parser.parse::<syn::Token![=]>()?;
                                            parser.parse::<Expr>()
                                        }
                                    )) == syn_try!(syn::parse_str::<Expr>("true"))
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
                        .to_tokens(&mut decoders);
                    }
                    syn::Fields::Unnamed(_) => todo!(),
                }
            }

            quote! {
                gen impl crate::decode::Decode for @Self {
                    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
                        match item {
                            crate::sexpr::SExprItem::Node(name, _items) => {
                                match name.as_str() {
                                    #allowed_names
                                    _ => false,
                                }
                            },
                            _ => false,
                        }
                    }
                    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, crate::decode::DecodeError> {
                        match item {
                            crate::sexpr::SExprItem::Node(name, items) => {
                                Ok(match name.as_str() {
                                    #decoders
                                    _ => {return Err(crate::decode::DecodeError::UnrecognisedSymbol(name))}
                                })
                            },
                            _ => Err(crate::decode::DecodeError::UnexpectedItem(item)),
                        }
                    }
                }
            }
        }
        _ => {
            syn_throw!(syn::Error::new_spanned(&s.ast().ident, "Unsupported data"));
        }
    };

    s.gen_impl(quote! {
        #decode
    })
}

decl_derive!([SpecTecItem, attributes(spectec_item, spectec_field)] => spectec_item_derive);
