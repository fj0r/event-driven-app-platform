use crate::utils::struct_has_field;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

pub fn impl_component_props(ast: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &ast.ident;

    let id = if struct_has_field(ast, "id") {
        quote! {
            fn get_id(&self) -> &Option<String> {
                &self.id
            }
        }
    } else {
        quote! {
            fn get_id(&self) -> &Option<String> {
                &None
            }
        }
    };

    let typ = quote! {
        fn get_type(&self) -> &str {
            stringify!(#name)
        }
    };

    let child = if struct_has_field(&ast, "children") {
        quote! {
            fn get_children(&mut self) -> Option<&mut Vec<JsonComponent>> {
                self.children.as_mut()
            }
            fn set_children(&mut self, component: Vec<JsonComponent>) {
                self.children = Some(component);
            }
        }
    } else {
        quote! {
            fn get_children(&mut self) -> Option<&mut Vec<JsonComponent>> {
                None
            }
            fn set_children(&mut self, component: Vec<JsonComponent>) {
            }
        }
    };

    let att = if struct_has_field(&ast, "attrs") {
        quote! {
            fn get_attrs(&self) -> Option<&dyn Classify> {
                Some(&self.attrs)
            }
        }
    } else {
        quote! {
            fn get_attrs(&self) -> Option<&dyn Classify> {
                None
            }
        }
    };

    let bind = if struct_has_field(&ast, "bind") {
        quote! {
            fn get_bind(&self) -> Option<&HashMap<String, Bind>> {
                self.bind.as_ref()
            }
            fn set_bind(&mut self, bind: Option<HashMap<String, Bind>>) {
                self.bind = bind;
            }
        }
    } else {
        quote! {
            fn get_bind(&self) -> Option<&HashMap<String, Bind>> {
                None
            }
            fn set_bind(&mut self, bind: Option<HashMap<String, Bind>>) {
            }
        }
    };

    let render = if struct_has_field(&ast, "render") {
        quote! {
            fn get_render(&self) -> Option<&Render> {
                self.render.as_ref()
            }
        }
    } else {
        quote! {
            fn get_render(&self) -> Option<&Render> {
                None
            }
        }
    };

    Ok(quote! {
        impl ComponentProps for #name {
            #id
            #typ
            #att
            #bind
            #render
            #child
        }
    })
}

pub fn impl_component_props_variant(ast: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &ast.ident;
    let mut r = Vec::new();
    if let syn::Data::Enum(d) = &ast.data {
        for i in &d.variants {
            r.push(&i.ident);
        }
    }
    Ok(quote! {
        impl ComponentProps for #name {
            fn get_id(&self) -> &Option<String> {
                match self {
                    #(#name::#r(c) => c.get_id()),*
                }
            }

            fn get_children(&mut self) -> Option<&mut Vec<JsonComponent>> {
                match self {
                    #(#name::#r(c) => c.get_children()),*
                }
            }

            fn set_children(&mut self, component: Vec<JsonComponent>) {
                match self {
                    #(#name::#r(c) => { c.set_children(component) }),*
                }
            }

            fn get_bind(&self) -> Option<&HashMap<String, Bind>> {
                match self {
                    #(#name::#r(c) => { c.get_bind() }),*
                }
            }

            fn set_bind(&mut self, bind: Option<HashMap<String, Bind>>) {
                match self {
                    #(#name::#r(c) => { c.set_bind(bind) }),*
                }
            }

            fn get_render(&self) -> Option<&Render> {
                match self {
                    #(#name::#r(c) => { c.get_render() }),*
                }
            }

            fn get_type(&self) -> &str {
                match self {
                    #(#name::#r(c) => { stringify!(#name::#r) }),*
                }
            }

            fn get_attrs(&self) -> Option<&dyn Classify> {
                match self {
                    #(#name::#r(c) => { c.get_attrs() }),*
                }
            }
        }
    })
}
