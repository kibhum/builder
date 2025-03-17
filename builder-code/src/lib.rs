mod fields;
use fields::{
    builder_definitions, builder_field_definitions, builder_impl_for_struct, builder_init_values,
    builder_methods, marker_trait_and_structs, optional_default_asserts, original_struct_setters,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Attribute, Data::Struct, DataStruct, DeriveInput, Field, Fields::Named, FieldsNamed, Ident,
    Type, punctuated::Punctuated, token::Comma,
};
mod util;

const DEFAULTS_ATTRIBUTES_NAME: &str = "builder_defaults";

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();
    let name = ast.ident;

    // let builder = format_ident!("{}Builder", name);
    // let use_defaults = use_defaults(&ast.attrs);

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("Only implemented for structs"),
    };

    // let default_assertions = if use_defaults {
    //     optional_default_asserts(fields)
    // } else {
    //     vec![]
    // };

    let builder = builder_definitions(&name, fields);
    let builder_method_for_struct = builder_impl_for_struct(&name, fields);
    let marker_and_structs = marker_trait_and_structs(&name, fields);
    let builder_methods = builder_methods(&name, fields);

    quote! {
        // struct #builder {
        //     #(#builder_fields,)*
        // }

        // impl #builder{
        //     #(#builder_methods)*

        //     pub fn build(self)->#name{
        //         #name{
        //             #(#original_struct_set_fields,)*
        //         }
        //     }
        // }

        // impl #name{
        //     pub fn builder()->#builder{
        //         #builder{
        //             #(#builder_inits,)*
        //         }
        //     }
        // }

        // #(#default_assertions)*

        #builder
        #builder_method_for_struct
        #marker_and_structs
        #builder_methods
    }
}

fn use_defaults(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attribute| attribute.path().is_ident(DEFAULTS_ATTRIBUTES_NAME))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn builder_struct_name_should_be_present_in_output() {
    //     let input = quote! {
    //         struct StructWithNoFields{}
    //     };

    //     let actual = create_builder(input);

    //     assert!(actual.to_string().contains("StructWithNoFieldsBuilder"))
    // }

    // #[test]
    // fn builder_struct_with_expected_methods_should_be_present_in_output() {
    //     let input = quote! {
    //         struct StructWithNoFields {}
    //     };

    //     let expected = quote! {
    //         struct StructWithNoFieldsBuilder{}
    //     };

    //     let actual = create_builder(input);

    //     assert_eq!(actual.to_string(), expected.to_string());
    // }
    // #[test]
    // fn assert_with_parsing() {
    //     let input = quote! {
    //        struct StructWithNoFields{}
    //     };

    //     let actual = create_builder(input);
    //     let derived: DeriveInput = syn::parse2(actual).unwrap();
    //     let name = derived.ident;
    //     assert_eq!(name.to_string(), "StructWithNoFieldsBuilder");
    // }
}
