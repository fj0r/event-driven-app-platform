use crate::utils::{get_ident_from_type, struct_has_field};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

pub fn impl_brick_props(ast: &DeriveInput) -> syn::Result<TokenStream2> {
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
            fn get_children(&self) -> Option<&Vec<Brick>> {
                self.children.as_ref()
            }
            fn borrow_children_mut(&mut self) -> Option<&mut Vec<Brick>> {
                self.children.as_mut()
            }
            fn set_children(&mut self, brick: Vec<Brick>) {
                self.children = Some(brick);
            }
        }
    } else {
        quote! {
            fn get_children(&self) -> Option<&Vec<Brick>> {
                None
            }
            fn borrow_children_mut(&mut self) -> Option<&mut Vec<Brick>> {
                None
            }
            fn set_children(&mut self, brick: Vec<Brick>) {
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
        impl BrickProps for #name {
            #id
            #typ
            #att
            #bind
            #render
            #child
        }
    })
}

pub fn impl_brick_props_variant(ast: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &ast.ident;
    let mut r = Vec::new();
    if let syn::Data::Enum(d) = &ast.data {
        for i in &d.variants {
            r.push(&i.ident);
        }
    }
    Ok(quote! {
        impl BrickProps for #name {
            fn get_id(&self) -> &Option<String> {
                match self {
                    #(#name::#r(c) => c.get_id()),*
                }
            }

            fn get_children(&self) -> Option<&Vec<Brick>> {
                match self {
                    #(#name::#r(c) => c.get_children()),*
                }
            }

            fn borrow_children_mut(&mut self) -> Option<&mut Vec<Brick>> {
                match self {
                    #(#name::#r(c) => c.borrow_children_mut()),*
                }
            }

            fn set_children(&mut self, brick: Vec<Brick>) {
                match self {
                    #(#name::#r(c) => { c.set_children(brick) }),*
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

pub fn impl_brick_wrap_variant(ast: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &ast.ident;
    let mut r = Vec::new();
    if let syn::Data::Enum(d) = &ast.data {
        for i in &d.variants {
            let v = &i.ident;
            let ty = match &i.fields {
                syn::Fields::Unnamed(f) => {
                    let x = f
                        .unnamed
                        .iter()
                        .map(|x| get_ident_from_type(&x.ty))
                        .filter(|x| x.is_some())
                        .map(|x| x.unwrap())
                        .collect::<Vec<_>>();
                    x.get(0).cloned()
                }
                _ => None,
            };
            if let Some(ty) = ty {
                r.push(quote! {
                    impl Wrap for #ty {
                        type Target = #name;
                        fn wrap(self) -> Self::Target {
                            Self::Target::#v(self)
                        }
                    }
                })
            }
        }
    }
    Ok(quote! {
        #(#r)*
    })
}
