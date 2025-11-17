use crate::utils::{get_ident_from_type, struct_has_field};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

pub fn impl_brick_props(ast: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &ast.ident;

    let id = if struct_has_field(ast, "id") {
        quote! { &self.id }
    } else {
        quote! { &None }
    };

    let mut child_ref = quote! { None };
    let mut child_mut = quote! { None };
    let mut set_child = quote! {};

    if struct_has_field(&ast, "children") {
        child_ref = quote! { self.children.as_ref() };
        child_mut = quote! { self.children.as_mut() };
        set_child = quote! { self.children = Some(brick); };
    };

    let item = if struct_has_field(&ast, "item") {
        quote! { self.children.as_ref() }
    } else {
        quote! { None }
    };

    let mut attrs_ref = quote! { None };
    let mut attrs_mut = quote! { None };
    if struct_has_field(&ast, "attrs") {
        attrs_ref = quote! { Some(&self.attrs) };
        attrs_mut = quote! { Some(&mut self.attrs) };
    };

    let mut get_bind = quote! { None };
    let mut set_bind = quote! {};
    if struct_has_field(&ast, "bind") {
        get_bind = quote! { self.bind.as_ref() };
        set_bind = quote! { self.bind = bind; }
    };

    let render = if struct_has_field(&ast, "render") {
        quote! { self.render.as_ref() }
    } else {
        quote! { None }
    };

    Ok(quote! {
        impl BrickProps for #name {
            fn get_id(&self) -> &Option<String> {
                #id
            }
            fn get_type(&self) -> &str {
                stringify!(#name)
            }
            fn borrow_children(&self) -> Option<&Vec<Brick>> {
                #child_ref
            }
            fn borrow_children_mut(&mut self) -> Option<&mut Vec<Brick>> {
                #child_mut
            }
            fn set_children(&mut self, brick: Vec<Brick>) {
                #set_child
            }
            fn get_item(&self) -> Option<&Vec<Brick>> {
                #item
            }
            fn borrow_attrs(&self) -> Option<&dyn Classify> {
                #attrs_ref
            }
            fn borrow_attrs_mut(&mut self) -> Option<&mut dyn Classify> {
                #attrs_mut
            }
            fn get_bind(&self) -> Option<&HashMap<String, Bind>> {
                #get_bind
            }
            fn set_bind(&mut self, bind: Option<HashMap<String, Bind>>) {
                #set_bind
            }
            fn get_render(&self) -> Option<&Render> {
                #render
            }
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

            fn borrow_children(&self) -> Option<&Vec<Brick>> {
                match self {
                    #(#name::#r(c) => c.borrow_children()),*
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

            fn get_item(&self) -> Option<&Vec<Brick>> {
                match self {
                    #(#name::#r(c) => c.get_item()),*
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

            fn borrow_attrs(&self) -> Option<&dyn Classify> {
                match self {
                    #(#name::#r(c) => { c.borrow_attrs() }),*
                }
            }

            fn borrow_attrs_mut(&mut self) -> Option<&mut dyn Classify> {
                match self {
                    #(#name::#r(c) => { c.borrow_attrs_mut() }),*
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
