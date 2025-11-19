use crate::utils::struct_has_field;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

pub fn impl_classify_attrs(ast: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &ast.ident;

    let get_selector = if struct_has_field(ast, "selector") {
        quote! { &self.selector }
    } else {
        quote! { &None }
    };

    let mut get_class = quote! { &None };
    let mut add_class = quote! {};
    let mut delete_class = quote! {};
    if struct_has_field(ast, "class") {
        get_class = quote! { &self.class };
        add_class = quote! {
            if let Some(cls) = &mut self.class {
                cls.push(class.to_string());
            } else {
                self.class = Some(vec![class.to_string()]);
            };
        };
        delete_class = quote! {
            if let Some(cls) = &mut self.class
                && cls.contains(&class.to_string()) {
                let ix = cls.iter().position(|x| x == &class.as_ref()).unwrap();
                cls.remove(ix);
            };
        };
    };

    let hor = if struct_has_field(ast, "horizontal") {
        quote! {
            if let Some(h) = self.horizontal {
                return h;
            };
            false
        }
    } else {
        quote! { false }
    };

    Ok(quote! {
        impl Classify for #name {
            fn get_selector(&self) -> &Option<String> {
                #get_selector
            }
            fn get_class(&self) -> &Option<Vec<String>> {
                #get_class
            }
            fn add_class(&mut self, class: &str) {
                #add_class
            }
            fn delete_class(&mut self, class: &str) {
                #delete_class
            }
            fn is_horizontal(&self) -> bool {
                #hor
            }
        }
    })
}

pub fn impl_classify_brick(ast: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &ast.ident;

    let g = if struct_has_field(ast, "attrs") {
        quote! {
            impl Classify for #name {
                fn get_class(&self) -> &Option<Vec<String>> {
                    self.borrow_attrs().unwrap().get_class()
                }
                fn get_selector(&self) -> &Option<String> {
                    self.borrow_attrs().unwrap().get_selector()
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
                fn get_selector(&self) -> &Option<String> {
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
                    #(#name::#r(c) => c.borrow_attrs().unwrap().get_class(),)*
                    _ => &None
                }
            }
            fn get_selector(&self) -> &Option<String> {
                match self {
                    #(#name::#r(c) => c.borrow_attrs().unwrap().get_selector(),)*
                    _ => &None
                }
            }
            fn add_class(&mut self, class: &str) {
                match self {
                    #(#name::#r(c) => { c.borrow_attrs_mut().unwrap().add_class(class) })*
                    _ => {}
                }
            }
            fn delete_class(&mut self, class: &str) {
                match self {
                    #(#name::#r(c) => { c.borrow_attrs_mut().unwrap().delete_class(class) })*
                    _ => {}
                }
            }
            fn is_horizontal(&self) -> bool {
                match self {
                    #(#name::#r(c) => c.borrow_attrs().unwrap().is_horizontal(),)*
                    _ => false
                }
            }
        }
    })
}
