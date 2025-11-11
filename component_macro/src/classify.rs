use crate::utils::struct_has_field;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn impl_classify_attrs(input: TokenStream2) -> syn::Result<TokenStream2> {
    let ast: DeriveInput = syn::parse2(input)?;
    let name = &ast.ident;

    let cls = if struct_has_field(&ast, "class") {
        quote! {
            fn get_class(&self) -> &Option<Vec<String>> {
                &self.class
            }
            fn add_class(&mut self, class: &str) {
                if let Some(cls) = &mut self.class {
                    cls.push(class.to_string());
                } else {
                    self.class = Some(vec![class.to_string()]);
                };
            }
            fn delete_class(&mut self, class: &str) {
                if let Some(cls) = &mut self.class
                    && cls.contains(&class.to_string()) {
                    let ix = cls.iter().position(|x| x == &class.as_ref()).unwrap();
                    cls.remove(ix);
                };
            }
        }
    } else {
        quote! {
            fn get_class(&self) -> &Option<Vec<String>> {
                &None
            }
            fn add_class(&mut self, class: &str) {
            }
            fn delete_class(&mut self, class: &str) {
            }
        }
    };

    let hor = if struct_has_field(&ast, "horizontal") {
        quote! {
            fn is_horizontal(&self) -> bool {
                if let Some(h) = self.horizontal {
                    return h;
                };
                false
            }
        }
    } else {
        quote! {
            fn is_horizontal(&self) -> bool {
                false
            }
        }
    };

    Ok(quote! {
        impl Classify for #name {
            #cls
            #hor
        }
    })
}

pub fn impl_classify_component(input: TokenStream2) -> syn::Result<TokenStream2> {
    let ast: DeriveInput = syn::parse2(input)?;
    let name = &ast.ident;

    let g = if struct_has_field(&ast, "attrs") {
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
