macro_rules! syn_throw {
    ($err:expr) => {
        return syn::Error::to_compile_error(&$err)
    };
}
pub(crate) use syn_throw;

macro_rules! syn_try {
    ($expr:expr) => {
        match $expr {
            Ok(expr) => expr,
            Err(err) => syn_throw!(err),
        }
    };
}
pub(crate) use syn_try;

pub(crate) fn get_attr<'a>(name: &str, attrs: &'a [syn::Attribute]) -> Option<&'a syn::Attribute> {
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
