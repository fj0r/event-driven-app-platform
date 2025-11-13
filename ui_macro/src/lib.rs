use anyhow::anyhow;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_file;
mod configlist;
mod walk;
use walk::walk;
mod utils;

#[proc_macro]
pub fn gen_dispatch(input: TokenStream) -> TokenStream {
    quote! {}.into()
}

fn gen_(ast: &syn::File) -> syn::Result<TokenStream2> {
    let info = walk(&ast);
    Ok(quote! {#ast})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        let d = include_str!("../../component/src/lib.rs");
        let ast = parse_file(&d).unwrap();
        let _ = std::fs::write("../data/component_def.ast", format!("{:#?}", ast));
        let output = walk(&ast);
        let _ = std::fs::write("../data/dispatch_def.rs", format!("{:#?}", output));
    }
}
