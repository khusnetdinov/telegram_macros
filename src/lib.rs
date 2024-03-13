use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(DerefInner)]
pub fn deref_inner_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let gen = quote! {
        impl std::ops::Deref for #ident {
            type Target = Inner;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(FromInner)]
pub fn from_inner_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let gen = quote! {
        impl From<Inner> for #ident {
            fn from(inner: Inner) -> Self {
                Self { inner }
            }
        }
    };

    gen.into()
}
