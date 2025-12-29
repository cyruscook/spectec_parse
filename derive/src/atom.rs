use crate::utils::{get_attr, syn_throw, syn_try};
use quote::{ToTokens, quote};
use syn::Expr;
use synstructure::Structure;

pub(crate) fn spectec_atom_derive(s: Structure) -> proc_macro2::TokenStream {
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
                let item_attr = if let Some(attr) = get_attr("spectec_atom", v.ast().attrs) {
                    attr
                } else {
                    syn_throw!(syn::Error::new_spanned(
                        variant_name,
                        "Must have a spectec_atom attribute"
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
                    _ => syn_throw!(syn::Error::new_spanned(
                        &v.ast().ident,
                        "Atoms cannot have any fields"
                    )),
                }
            }

            quote! {
                gen impl crate::decode::Decode for @Self {
                    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
                        match item {
                            crate::sexpr::SExprItem::Atom(name) => {
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
                            crate::sexpr::SExprItem::Atom(name) => {
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
            syn_throw!(syn::Error::new_spanned(&s.ast().ident, "Unsupported data"))
        }
    };

    s.gen_impl(quote! {
        #decode
    })
}
