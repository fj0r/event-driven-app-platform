mod classify;
mod props;
mod utils;

use classify::{impl_classify_attrs, impl_classify_component, impl_classify_variant};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use props::{impl_component_props, impl_component_props_variant};
use syn::{Data, DeriveInput, parse_macro_input};

fn into_ts(result: syn::Result<TokenStream2>) -> TokenStream {
    match result {
        Ok(output_stream2) => output_stream2.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_derive(ComponentProps)]
pub fn component_props(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ast.data {
        Data::Struct(_) => into_ts(impl_component_props(&ast)),
        Data::Enum(_) => into_ts(impl_component_props_variant(&ast)),
        _ => syn::Error::new(
            ast.ident.span(),
            "ComponentProps only supports structs and enums",
        )
        .to_compile_error()
        .into(),
    }
}

#[proc_macro_derive(ClassifyAttrs)]
pub fn classify_attrs(input: TokenStream) -> TokenStream {
    let input_stream2: TokenStream2 = input.into();

    match impl_classify_attrs(input_stream2) {
        Ok(output_stream2) => output_stream2.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_derive(ClassifyComponent)]
pub fn classify_component(input: TokenStream) -> TokenStream {
    let input_stream2: TokenStream2 = input.into();

    match impl_classify_component(input_stream2) {
        Ok(output_stream2) => output_stream2.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_derive(ClassifyVariant)]
pub fn classify_variant(input: TokenStream) -> TokenStream {
    let input_stream2: TokenStream2 = input.into();

    match impl_classify_variant(input_stream2) {
        Ok(output_stream2) => output_stream2.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[cfg(test)]
mod test_macro {
    use super::*;
    use quote::quote;
    use syn::DeriveInput;
    use syn::parse2;

    #[test]
    fn test_struct_hello() {
        let input = quote! {
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
            #[cfg_attr(feature = "dioxus", derive(Props))]
            #[cfg_attr(feature = "schema", derive(JsonSchema))]
            pub struct Placeholder {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub id: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub attrs: Option<ClassAttr>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub children: Option<Vec<JsonComponent>>,
            }
        };

        let _input = quote! {
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
            #[cfg_attr(feature = "dioxus", derive(Props))]
            #[cfg_attr(feature = "schema", derive(JsonSchema))]
            pub struct ClassAttr {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub class: Option<Vec<String>>,
            }
        };

        let _input = quote! {
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            #[cfg_attr(feature = "schema", derive(JsonSchema))]
            #[serde(tag = "type")]
            pub enum JsonComponent {
                case(Case),
                placeholder(Placeholder),
            }
        };

        let _input = quote! {
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            #[cfg_attr(feature = "schema", derive(JsonSchema))]
            #[serde(untagged)]
            pub enum BindVariant {
                Source {
                    source: String,
                },
                Target {
                    target: String,
                },
                Event {
                    event: String,
                },
                Field {
                    field: String,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    payload: Option<Value>,
                    #[cfg(feature = "dioxus")]
                    #[allow(dead_code)]
                    #[serde(skip)]
                    signal: Option<Signal<Value>>,
                },
                Submit {
                    submit: bool,
                    #[cfg(feature = "dioxus")]
                    #[allow(dead_code)]
                    #[serde(skip)]
                    signal: Option<Signal<Value>>,
                },
                Default {},
            }
        };

        let ast = syn::parse2::<DeriveInput>(input).unwrap();
        let output = impl_component_props(&ast).expect("Macro expansion failed");

        let _ = std::fs::write("../data/out.ast", format!("{:#?}", ast));
        let _ = std::fs::write("../data/out.rs", format!("{:#}", output.to_string()));

        assert!(true);
    }

    #[test]
    fn test_attribute_rename() {
        use syn::ItemFn;
        let input_args = quote! { value=1 };

        let input_item = quote! {
            #[xxx]
            fn original_function() {
                println!("{:?}", 123);
            }
        };

        let expected_fn: ItemFn = parse2(input_item).expect("Failed to parse expected output");

        let _ = std::fs::write("../data/itemfn.ast", format!("{:#?}", expected_fn));
    }
}
