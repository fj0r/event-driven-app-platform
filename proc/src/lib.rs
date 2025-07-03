use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Hello)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let name = ident.to_string();
    quote! {
        impl #ident {
            pub fn hello(&self) -> String {
                format!("hello from {}", #name)
            }
        }
    }.into()
}
