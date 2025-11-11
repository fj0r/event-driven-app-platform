use syn::{Data, DeriveInput};

pub fn struct_has_field(ast: &DeriveInput, field: &str) -> bool {
    let mut has_field = false;
    if let Data::Struct(d) = &ast.data {
        for i in &d.fields {
            if let Some(id) = &i.ident
                && id.to_string() == field
            {
                has_field = true;
                break;
            }
        }
    };
    has_field
}
