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

pub(crate) fn get_attr<'a>(
    name: &str,
    attrs: &'a [syn::Attribute],
) -> Result<Option<&'a syn::Attribute>, syn::Error> {
    let mut matching_attrs = attrs.iter().filter(|attr| attr.meta.path().is_ident(name));
    match matching_attrs.next() {
        None => Ok(None),
        Some(first) => {
            // Check this is the only matching one
            if let Some(second) = matching_attrs.next() {
                Err(syn::Error::new_spanned(
                    second,
                    format!("Cannot have more than one attribute with name '{name}'"),
                ))
            } else {
                Ok(Some(first))
            }
        }
    }
}
