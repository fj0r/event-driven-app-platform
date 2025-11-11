use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn impl_classify_struct(input: TokenStream2) -> syn::Result<TokenStream2> {
    let ast: DeriveInput = syn::parse2(input)?;
    let name = &ast.ident;

    let g = quote! {
        impl Classify for #name {
            fn get_class(&self) -> &Option<Vec<String>> {
                self.attrs.get_class()
            }
            fn add_class(&mut self, class: &str) {
                self.attrs.add_class(class);
            }
            fn delete_class(&mut self, class: &str) {
                self.attrs.delete_class(class);
            }
            fn is_horizontal(&self) -> bool {
                self.attrs.is_horizontal()
            }
        }
    };

    Ok(g)
}
