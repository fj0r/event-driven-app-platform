use crate::utils::struct_has_field;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

pub fn impl_classify_attrs(ast: &DeriveInput) -> syn::Result<TokenStream2> {
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

pub fn impl_classify_brick(ast: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &ast.ident;

    let g = if struct_has_field(&ast, "attrs") {
        quote! {
            impl Classify for #name {
                fn get_class(&self) -> &Option<Vec<String>> {
                    self.borrow_attrs().unwrap().get_class()
                }
                fn add_class(&mut self, class: &str) {
                    self.borrow_attrs_mut().unwrap().add_class(class);
                }
                fn delete_class(&mut self, class: &str) {
                    self.borrow_attrs_mut().unwrap().delete_class(class);
                }
                fn is_horizontal(&self) -> bool {
                    self.borrow_attrs().unwrap().is_horizontal()
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

pub fn impl_classify_variant(ast: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &ast.ident;
    let mut r = Vec::new();
    if let syn::Data::Enum(d) = &ast.data {
        for i in &d.variants {
            r.push(&i.ident);
        }
    }
    Ok(quote! {
        impl Classify for #name {
            fn get_class(&self) -> &Option<Vec<String>> {
                match self {
                    #(#name::#r(c) => c.borrow_attrs().get_class(),)*
                    _ => &None
                }
            }
            fn add_class(&mut self, class: &str) {
                match self {
                    #(#name::#r(c) => { c.borrow_attrs_mut().add_class(class) })*
                    _ => {}
                }
            }
            fn delete_class(&mut self, class: &str) {
                match self {
                    #(#name::#r(c) => { c.borrow_attrs_mut().delete_class(class) })*
                    _ => {}
                }
            }
            fn is_horizontal(&self) -> bool {
                match self {
                    #(#name::#r(c) => c.borrow_attr().is_horizontal(),)*
                    _ => false
                }
            }
        }
    })
}
