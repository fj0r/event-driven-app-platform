use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn impl_classify_component(input: TokenStream2) -> syn::Result<TokenStream2> {
    let ast: DeriveInput = syn::parse2(input)?;
    let name = &ast.ident;

    let mut has_attr = false;
    if let syn::Data::Struct(d) = ast.data {
        for i in d.fields {
            if let Some(id) = i.ident
                && id.to_string() == "attrs"
            {
                has_attr = true;
                break;
            }
        }
    };
    let g = if has_attr {
        quote! {
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
        }
    } else {
        quote! {
            impl Classify for #name {
                fn get_class(&self) -> &Option<Vec<String>> {
                    &None
                }
                fn add_class(&mut self, class: &str) {
                }
                fn delete_class(&mut self, class: &str) {
                }
                fn is_horizontal(&self) -> bool {
                    false
                }
            }
        }
    };

    Ok(g)
}
