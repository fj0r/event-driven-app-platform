use crate::configlist::ConfigList;
use crate::utils::{get_ident_from_type, struct_has_field};
use quote::ToTokens;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub r#type: String,
    pub has_id: bool,
}

#[derive(Debug)]
pub enum CompInfo {
    Struct { name: String, has_child: bool },
    Enum { fields: Vec<Field> },
}

pub fn walk(ast: &syn::File) -> HashMap<String, CompInfo> {
    ast.items.iter().fold(HashMap::new(), |mut acc, x| {
        match x {
            syn::Item::Struct(x) => {
                let has_child = struct_has_field(x, "children");
                let info = CompInfo::Struct {
                    name: x.ident.to_string(),
                    has_child,
                };
                acc.insert(x.ident.to_string(), info);
            }
            syn::Item::Enum(x) => {
                let fields = x
                    .variants
                    .iter()
                    .map(|x| {
                        let ty = x
                            .fields
                            .iter()
                            .map(|x| get_ident_from_type(&x.ty))
                            .collect::<Vec<_>>();
                        let ty = if let Some(t) = ty.get(0)
                            && let Some(i) = t
                        {
                            i.to_string()
                        } else {
                            "".to_string()
                        };
                        let kv = x
                            .attrs
                            .iter()
                            .map(|x| {
                                let x = x.meta.to_token_stream();
                                syn::parse2::<ConfigList>(x.into())
                                    .unwrap_or_else(|_| ConfigList::default())
                                    .0
                            })
                            .flatten()
                            .collect::<HashMap<_, _>>();
                        let has_id = if let Some(h) = kv.get("has_id") {
                            h == "true"
                        } else {
                            false
                        };
                        Field {
                            name: x.ident.to_string(),
                            r#type: ty,
                            has_id,
                        }
                    })
                    .collect();
                acc.insert(x.ident.to_string(), CompInfo::Enum { fields });
            }
            _ => {}
        }
        acc
    })
}
