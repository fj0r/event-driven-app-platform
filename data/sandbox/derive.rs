//! ```cargo
//! [dependencies]
//! colored = "3.0.0"
//! proc-macro2 = { version = "1.0.103", features = ["span-locations"] }
//! quote = "1.0.42"
//! serde = "1.0.228"
//! serde_json = "1.0.145"
//! syn = { version = "2.0.109", features = ["extra-traits", "full"] }
//! anyhow = "*"
//! ```
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

mod my_awesome_lib {
    pub trait Hello {
        fn hello(&self);
    }
}

fn generate_hello_impl(input: TokenStream) -> Result<TokenStream> {
    let ast: DeriveInput = syn::parse2(input)?;
    let name = &ast.ident;

    match ast.data {
        Data::Struct(_) => {}
        Data::Enum(_) | Data::Union(_) => {
            return Err(anyhow::anyhow!(Error::new(
                name.span(),
                "The Hello derive macro only works on structs."
            )));
        }
    }

    let gen = quote! {
        impl crate::my_awesome_lib::Hello for #name {
            fn hello(&self) {
                println!("Hello, my name is {}!", stringify!(#name));
            }
        }
    };

    Ok(gen)
}

fn main() -> Result<()> {
    let input = quote! {
        struct User { id: u32, name: String }
    };

    println!(
        "--- Input AST (Debug Format) ---\n{:#?}\n",
        syn::parse2::<DeriveInput>(input.clone())?
    );

    let generated_output = generate_hello_impl(input)?;

    println!(
        "--- Generated TokenStream (Code) ---\n{}\n",
        generated_output.to_string()
    );

    println!(
        "--- Generated TokenStream (Debug Format) ---\n{:#?}",
        generated_output
    );

    Ok(())
}
