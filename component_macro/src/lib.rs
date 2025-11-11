use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};
mod classify;
use classify::impl_classify_component;

#[proc_macro_derive(classify)]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let input_stream2: TokenStream2 = input.into();

    match impl_classify_component(input_stream2) {
        Ok(output_stream2) => output_stream2.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[cfg(test)]
mod test_macro {
    use super::*;
    use quote::quote;
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

        let input2 = quote! {
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
            #[cfg_attr(feature = "dioxus", derive(Props))]
            #[cfg_attr(feature = "schema", derive(JsonSchema))]
            pub struct ClassAttr {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub class: Option<Vec<String>>,
            }
        };

        let input3 = quote! {
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            #[cfg_attr(feature = "schema", derive(JsonSchema))]
            #[serde(tag = "type")]
            pub enum JsonComponent {
                case(Case),
                placeholder(Placeholder),
            }
        };

        let input4 = quote! {
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

        let output = impl_classify_component(input.clone()).expect("Macro expansion failed");

        let ast = syn::parse2::<DeriveInput>(input).unwrap();
        let _ = std::fs::write("../data/out.ast", format!("{:#?}", ast));
        let _ = std::fs::write("../data/out.rs", format!("{:#}", output.to_string()));

        assert!(true);
    }
}
