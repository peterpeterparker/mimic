use crate::{
    helper::{quote_one, to_path},
    node::Args,
};
use darling::FromMeta;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use syn::Path;
use quote::quote;

///
/// TypeSanitizer
///

#[derive(Clone, Debug, FromMeta)]
pub struct TypeSanitizer {
    pub path: Path,

    #[darling(default)]
    pub args: Args,
}

impl Schemable for TypeSanitizer {
    fn schema(&self) -> TokenStream {
        let path = quote_one(&self.path, to_path);
        let args = &self.args.schema();

        let q = quote! {
            ::mimic::orm::schema::node::TypeSanitizer {
                path: #path,
                args: #args,
            }
        };

        //  panic!("{q}");

        q
    }
}

///
/// TypeValidator
///

#[derive(Clone, Debug, FromMeta)]
pub struct TypeValidator {
    pub path: Path,

    #[darling(default)]
    pub args: Args,
}

impl Schemable for TypeValidator {
    fn schema(&self) -> TokenStream {
        let path = quote_one(&self.path, to_path);
        let args = &self.args.schema();

        let q = quote! {
            ::mimic::orm::schema::node::TypeValidator {
                path: #path,
                args: #args,
            }
        };

        //  panic!("{q}");

        q
    }
}
