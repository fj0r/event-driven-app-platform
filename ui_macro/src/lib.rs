use std::collections::HashMap;

use anyhow::anyhow;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse_file;
mod configlist;
mod walk;
use walk::{CompInfo, walk};
mod utils;

#[proc_macro]
pub fn gen_dispatch(input: TokenStream) -> TokenStream {
    quote! {}.into()
}

fn gen_match(ast: &syn::File, entry: &str) -> syn::Result<TokenStream2> {
    let info = walk(&ast);
    let ty = Ident::new(entry, Span::call_site());
    let CompInfo::Enum { fields } = info
        .get(entry)
        .ok_or(syn::Error::new(Span::call_site(), "no fields"))?
    else {
        return Err(syn::Error::new(Span::call_site(), "no enum"));
    };
    let f = fields.iter().map(|x| {
        let var = Ident::new(&x.name, Span::call_site());
        let var_ = Ident::new(&format!("{}_", &x.name), Span::call_site());
        let has_child = info
            .get(&x.r#type)
            .ok_or(syn::Error::new(Span::call_site(), "get child failed"));
        let Ok(CompInfo::Struct { name: _, has_child }) = has_child else {
            // return Err(syn::Error::new(Span::call_site(), "no child"));
            panic!("no child")
        };
        let children = if *has_child {
            quote! {
                {children}
            }
        } else {
            quote! {}
        };
        let id = if x.has_id {
            let id = Ident::new(&format!("{}_id", &x.name).to_uppercase(), Span::call_site());
            let fmt = format!("{}-{{}}", &x.name);
            quote! {
                static #id: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
                let mut tc = #id.lock().unwrap();
                *tc += 1;
                let id = format!(#fmt , *tc) ;
            }
        } else {
            quote! {}
        };

        quote! {
            #ty::#var(c) => {
                #id
                rsx!(#var_ {
                    id: id,
                    component: c,
                    #children
                })
            }
        }
    });

    Ok(quote! {
        match component {
            #(#f),*
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        let d = include_str!("../../component/src/lib.rs");
        let ast = parse_file(&d).unwrap();
        let _ = std::fs::write("../data/component_def.ast", format!("{:#?}", ast));
        let info = walk(&ast);
        let _ = std::fs::write("../data/info.rs", format!("{:#?}", info));
        let output = gen_match(&ast, "JsonComponent").unwrap();
        let _ = std::fs::write("../data/dispatch_def.rs", format!("{}", output.to_string()));
    }
}
