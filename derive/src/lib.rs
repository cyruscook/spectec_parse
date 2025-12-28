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

pub(crate) fn spectec_item_derive(s: Structure) -> proc_macro2::TokenStream {
    let decode = match s.ast().data {
        syn::Data::Enum(_) => {
            let mut decoders = quote!();

            for v in s.variants() {
                if let Some(disc) = v.ast().discriminant {
                    syn_throw!(syn::Error::new_spanned(
                        &disc.1,
                        "Must not have a discriminant"
                    ));
                }
                let variant_name = v.ast().ident;
                let item_attrs = v
                    .ast()
                    .attrs
                    .iter()
                    .filter(|attr| attr.meta.path().is_ident("spectec_item"))
                    .collect::<Vec<_>>();
                let item_attr = if let [item_attr] = item_attrs.as_slice() {
                    item_attr
                } else {
                    syn_throw!(syn::Error::new_spanned(
                        variant_name,
                        "Must be exactly one spectec_item attribute"
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
                match v.ast().fields {
                    syn::Fields::Unit => {
                        (quote!(
                            #item_name => Self::#variant_name,
                        ))
                        .to_tokens(&mut decoders);
                    }
                    syn::Fields::Named(named) => {
                        let field_names = named
                            .named
                            .iter()
                            .map(|f| f.ident.as_ref().unwrap())
                            .collect::<Vec<_>>();
                        let field_decodes = named
                            .named
                            .iter()
                            .map(|f| {
                                let ty = &f.ty;
                                quote! {
                                    #ty::decode(r)?
                                }
                            })
                            .collect::<Vec<_>>();
                        (quote!(
                            #item_name => {
                                Ok(Self::#variant_name {
                                    #(
                                        #field_names: #field_decodes,
                                    )*
                                })
                            },
                        ))
                        .to_tokens(&mut decoders);
                    }
                    syn::Fields::Unnamed(_) => todo!(),
                }
            }

            quote! {
                gen impl Decode for @Self
                    {
                    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError> {
                        match item {
                            crate::sexpr::SExprItem::Node(name, items) => {
                                Ok(match name.as_str() {
                                    #decoders
                                    _ => {return Err(crate::decode::DecodeError::UnrecognisedSymbol(name))}
                                })
                            },
                            _ => Err(DecodeError::UnexpectedItem(item)),
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

decl_derive!([SpecTecItem, attributes(spectec_item)] => spectec_item_derive);
