use crate::create_builder;
use crate::util::{create_builder_ident, create_field_struct_name};
use proc_macro2::{Punct, TokenStream as TokenStream2};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    Data::Struct, DataStruct, DeriveInput, Expr, ExprLit, Field, Fields::Named, FieldsNamed, Ident,
    Lit, LitStr, Meta, MetaNameValue, Type, punctuated::Punctuated, spanned::Spanned, token::Comma,
};
pub fn optional_default_asserts(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().unwrap();
            let ty = &f.ty;

            let assertion_ident = format_ident!("__{}DefaultAssertion", name);

            quote_spanned! {ty.span()=>
                struct #assertion_ident where #ty: core::default:Default;
            }
        })
        .collect()
}

// pub fn original_struct_setters<'a>(
//     fields: &'a Punctuated<Field, Comma>,
//     use_defaults: bool,
// ) -> Vec<TokenStream2> {
//     fields
//         .iter()
//         .map(|f| {
//             let (field_name, _field_type) = get_name_and_type(f);
//             let field_name_as_string = field_name.as_ref().unwrap().to_string();

//             // let error = quote!(expect(&format!("Field {} not set",#field_name_as_string)));

//             let handle_type = if use_defaults {
//                 default_fallback()
//             } else {
//                 panic_fallback(field_name_as_string)
//             };

//             quote!(#field_name:self.#field_name.#handle_type)

//             // quote! {
//             //     #field_name: self.#field_name.expect(concat!("field not set: ", #field_name_as_string))
//             // }

//             // if matches_type(field_type, "String"){
//             //     quote! {
//             //         #field_name: self.#field_name.expect(concat!("field not set: ", #field_name_as_string))
//             //     }
//             // }else{
//             //     quote! {
//             //         #field_name: self.#field_name.expect(concat!("field not set: ", #field_name_as_string))
//             //     }
//             // }
//         })
//         .collect()
// }

pub fn builder_definitions(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream2 {
    let builder_fields = fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        quote! {#field_name: Option<#field_type>}
    });
    let builder_name = create_builder_ident(name);

    quote! {
        pub struct #builder_name<T:MarkerTraitForBuilder>{
            marker: std::marker::PhantomData<T>,
            #(#builder_fields,)*
        }
    }
}

pub fn builder_impl_for_struct(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream2 {
    let builder_inits = fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {#field_name:None}
    });

    let first_field_name = fields.first().map(|f| f.ident.clone().unwrap()).unwrap();

    let builder_name = create_builder_ident(name);

    let generic = create_field_struct_name(&builder_name, &first_field_name);

    quote! {
        impl #name{
            pub fn builder()->#builder_name<#generic>{
                #builder_name{
                    marker:Default::default(),
                    #(#builder_inits,)*
                }
            }
        }
    }
}

fn panic_fallback(field_name_as_string: String) -> TokenStream2 {
    quote! {
        expect(concat!("field not set: ", #field_name_as_string))
    }
}

fn default_fallback() -> TokenStream2 {
    quote! {
       unwrap_or_default()
    }
}

// pub fn builder_methods(
//     fields: &Punctuated<Field, Comma>,
// ) -> impl Iterator<Item = TokenStream2> + '_ {
//     fields.iter().map(|f| {
//         let (field_name, field_type) = get_name_and_type(f);
//         let attr = extract_attribute_from_field(f, "rename")
//             .map(|a| &a.meta)
//             .map(|m| match m {
//                 Meta::List(nested) => {
//                     let a: LitStr = nested.parse_args().unwrap();
//                     Ident::new(&a.value(), a.span())
//                 }
//                 Meta::Path(_) => {
//                     panic!("Expected brackets with name of the prop")
//                 }
//                 Meta::NameValue(_) => {
//                     panic!("Did not expect name + value")
//                 }
//             });

//         if let Some(attr) = attr {
//             quote! {
//                 pub fn #attr(mut self, input: #field_type)->Self{
//                     self.#field_name = Some(input);
//                     self
//                 }
//             }
//         } else {
//             quote! {
//                 pub fn #field_name(mut self, input:#field_type)-> Self{
//                 self.#field_name = Some(input);
//                 self
//                 }
//             }
//         }
//     })
// }

// pub fn builder_methods(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream2> {
//     fields
//         .iter()
//         .map(|f| {
//             let (field_name, field_type) = get_name_and_type(f);

//             extract_attribute_from_field(f, "rename")
//                 .map(|a| &a.meta)
//                 .map(|m| match m {
//                     Meta::List(nested) => {
//                         let a: LitStr = nested.parse_args().unwrap();
//                         Ident::new(&a.value(), a.span())
//                     }
//                     Meta::Path(_) => {
//                         panic!("Expected brackets with name of the prop")
//                     }
//                     Meta::NameValue(MetaNameValue {
//                         value:
//                             Expr::Lit(
//                                 ExprLit {
//                                     lit: Lit::Str(literal_string),
//                                     ..
//                                 },
//                                 ..,
//                             ),
//                         ..
//                     }) => Ident::new(&literal_string.value(), literal_string.span()),
//                     _ => panic!("Expected key and value for rename attribute"),
//                 })
//                 .map(|attr| {
//                     quote! {
//                         pub fn #attr(mut self, input:#field_type)-> Self{
//                         self.#field_name = Some(input);
//                         self
//                         }
//                     }
//                 })
//                 .unwrap_or_else(|| {
//                     quote! {
//                         pub fn #field_name(mut self, input:#field_type)-> Self{
//                         self.#field_name = Some(input);
//                         self
//                         }
//                     }
//                 })
//         })
//         .collect()
// }

pub fn builder_methods(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream2 {
    let builder_name = create_builder_ident(name);
    let set_fields = original_struct_setters(fields);
    let assignments_for_all_fields = get_assignments_for_fields(fields);
    let mut previous_field = None;
    let reverse_names_and_types: Vec<&Field> = fields.iter().rev().collect();

    let methods: Vec<TokenStream2> = reverse_names_and_types
        .iter()
        .map(|f| {
            if let Some(next_in_list) = previous_field {
                previous_field = Some(f);
                builder_for_field(&builder_name, &assignments_for_all_fields, f, next_in_list)
            } else {
                previous_field = Some(f);
                builder_for_final_field(&builder_name, &assignments_for_all_fields, f)
            }
        })
        .collect();

    quote! {
        #(#methods)*

        impl #builder_name<FinalBuilder>{
            pub fn build(self)->#name{
                #name{
                    #(#set_fields,)*
                }
            }
        }
    }
}

fn builder_for_field(
    builder_name: &Ident,
    field_assignments: &Vec<TokenStream2>,
    current_field: &Field,
    next_field_in_list: &Field,
) -> TokenStream2 {
    let (field_name, field_type) = get_name_and_type(current_field);
    let (next_field_name, _) = get_name_and_type(next_field_in_list);
    let current_field_struct_name =
        create_field_struct_name(&builder_name, field_name.as_ref().unwrap());
    let next_field_struct_name =
        create_field_struct_name(&builder_name, next_field_name.as_ref().unwrap());

    quote! {
        impl #builder_name<#current_field_struct_name>{
            pub fn #field_name(mut self, input: #field_type)->#builder_name<#next_field_struct_name>{
    self.#field_name=Some(input);
        #builder_name{
            marker:Default::default(),
            #(#field_assignments,)*
        }
            }
        }
    }
}

fn builder_for_final_field(
    builder_name: &Ident,
    field_assignments: &Vec<TokenStream2>,
    field: &Field,
) -> TokenStream2 {
    let (field_name, field_type) = get_name_and_type(field);
    let field_struct_name = create_field_struct_name(&builder_name, field_name.as_ref().unwrap());

    quote! {
        impl #builder_name<#field_struct_name>{
            pub fn #field_name(mut self, input: #field_type)->#builder_name<FinalBuilder>{
    self.#field_name=Some(input);
        #builder_name{
            marker:Default::default(),
            #(#field_assignments,)*
        }
            }
        }
    }
}

pub fn builder_init_values(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream2> + '_ {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {#field_name:None}
    })
}

pub fn builder_field_definitions(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream2> + '_ {
    fields.iter().map(|f| {
        let (name, f_type) = get_name_and_type(f);
        quote! {pub #name: Option<#f_type>}
    })
}

fn matches_type(ty: &Type, type_name: &str) -> bool {
    if let Type::Path(p) = ty {
        let first_match = p.path.segments[0].ident.to_string();
        return first_match == *type_name;
    }
    false
}

fn get_name_and_type<'a>(field: &'a Field) -> (&'a Option<Ident>, &'a Type) {
    let field_name = &field.ident;
    let field_type = &field.ty;
    (field_name, field_type)
}

fn extract_attribute_from_field<'a>(f: &'a Field, name: &'a str) -> Option<&'a syn::Attribute> {
    f.attrs.iter().find(|&attr| attr.path().is_ident(name))
}

pub fn marker_trait_and_structs(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream2 {
    let builder_name = create_builder_ident(name);

    let structs_and_impls = fields.iter().map(|f| {
        let field_name = &f.ident.clone().unwrap();
        let struct_name = create_field_struct_name(&builder_name, field_name);
        quote! {
            pub struct #struct_name{}
            impl MarkerTraitForBuilder for #struct_name{}
        }
    });

    quote! {
        pub trait MarkerTraitForBuilder{}

        #(#structs_and_impls)*

        pub struct FinalBuilder{}

        impl MarkerTraitForBuilder for FinalBuilder{}

    }
}

fn get_assignments_for_fields(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|f| {
            let (field_name, _) = get_name_and_type(f);

            quote! {
                #field_name: self.#field_name
            }
        })
        .collect()
}

pub fn original_struct_setters(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|f| {
            let field_name = &f.ident;
            let field_name_as_string = field_name.as_ref().unwrap().to_string();

            let handle_type = panic_fallback(field_name_as_string);

            quote! {
                #field_name: self.#field_name.#handle_type
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::{
        Field, FieldMutability, Ident, Path, PathSegment, Type, TypePath, Visibility,
        punctuated::Punctuated,
    };

    use super::get_name_and_type;

    fn get_name_and_type_give_back_name() {
        let p = PathSegment {
            ident: Ident::new("string", Span::call_site()),
            arguments: Default::default(),
        };

        let mut pun = Punctuated::new();
        pun.push(p);
        let ty = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: pun,
            },
        });

        let f = Field {
            attrs: vec![],
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(Ident::new("example", Span::call_site())),
            colon_token: None,
            ty,
        };

        let (actual_name, _) = get_name_and_type(&f);
        assert_eq!(
            actual_name.as_ref().unwrap().to_string(),
            "example".to_string()
        );
    }
}
