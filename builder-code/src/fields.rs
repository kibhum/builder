use proc_macro2::TokenStream as TokenStream2;
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

pub fn original_struct_setters<'a>(
    fields: &'a Punctuated<Field, Comma>,
    use_defaults: bool,
) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|f| {
            let (field_name, _field_type) = get_name_and_type(f);
            let field_name_as_string = field_name.as_ref().unwrap().to_string();

            // let error = quote!(expect(&format!("Field {} not set",#field_name_as_string)));

            let handle_type = if use_defaults {
                default_fallback()
            } else {
                panic_fallback(field_name_as_string)
            };

            quote!(#field_name:self.#field_name.#handle_type)

            // quote! {
            //     #field_name: self.#field_name.expect(concat!("field not set: ", #field_name_as_string))
            // }

            // if matches_type(field_type, "String"){
            //     quote! {
            //         #field_name: self.#field_name.expect(concat!("field not set: ", #field_name_as_string))
            //     }
            // }else{
            //     quote! {
            //         #field_name: self.#field_name.expect(concat!("field not set: ", #field_name_as_string))
            //     }
            // }
        })
        .collect()
}

fn panic_fallback(field_name_as_string: String) -> TokenStream2 {
    quote! {
        .expect(concat!("field not set: ", #field_name_as_string))
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

pub fn builder_methods(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|f| {
            let (field_name, field_type) = get_name_and_type(f);

            extract_attribute_from_field(f, "rename")
                .map(|a| &a.meta)
                .map(|m| match m {
                    Meta::List(nested) => {
                        let a: LitStr = nested.parse_args().unwrap();
                        Ident::new(&a.value(), a.span())
                    }
                    Meta::Path(_) => {
                        panic!("Expected brackets with name of the prop")
                    }
                    Meta::NameValue(MetaNameValue {
                        value:
                            Expr::Lit(
                                ExprLit {
                                    lit: Lit::Str(literal_string),
                                    ..
                                },
                                ..,
                            ),
                        ..
                    }) => Ident::new(&literal_string.value(), literal_string.span()),
                    _ => panic!("Expected key and value for rename attribute"),
                })
                .map(|attr| {
                    quote! {
                        pub fn #attr(mut self, input:#field_type)-> Self{
                        self.#field_name = Some(input);
                        self
                        }
                    }
                })
                .unwrap_or_else(|| {
                    quote! {
                        pub fn #field_name(mut self, input:#field_type)-> Self{
                        self.#field_name = Some(input);
                        self
                        }
                    }
                })
        })
        .collect()
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
