use proc_macro2::Ident;
use quote::format_ident;

pub fn create_builder_ident(name: &Ident) -> Ident {
    format_ident!("{}Builder", name)
}

pub fn create_field_struct_name(builder: &Ident, field: &Ident) -> Ident {
    format_ident!("{}Of{}", field, builder)
}
