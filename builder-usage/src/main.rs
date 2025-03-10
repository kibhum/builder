#[cfg(test)]
mod tests {
    use builder_macro::Builder;
    // #[test]
    // fn should_generate_builder_for_struct_with_no_properties() {
    //     #[derive(Builder)]
    //     struct ExampleStructNoFields {}

    //     let _: ExampleStructNoFields = ExampleStructNoFields::builder().build();
    // }

    // #[test]
    // fn should_generate_builder_for_struct_with_one_property() {
    //     #[derive(Builder)]
    //     struct GleipnirA {
    //         roots_of: String,
    //     }

    //     let gleipnir = GleipnirA::builder()
    //         .roots_of("mountains".to_string())
    //         .build();

    //     assert_eq!(gleipnir.roots_of, "mountains".to_string());
    // }

    // #[test]
    // fn should_generate_builder_for_struct_with_two_properties() {
    //     #[derive(Builder)]
    //     struct GleipnirB {
    //         roots_of: String,
    //         breath_of_a_fish: u8,
    //     }

    //     let gleipnir = GleipnirB::builder()
    //         .roots_of("mountain".to_string())
    //         .breath_of_a_fish(1)
    //         .build();

    //     assert_eq!(gleipnir.roots_of, "mountain".to_string());
    //     assert_eq!(gleipnir.breath_of_a_fish, 1);
    // }

    // #[test]
    // fn should_generate_builder_for_struct_with_multiple_properties() {
    //     #[derive(Builder)]
    //     struct GleipnirC {
    //         roots_of: String,
    //         breath_of_a_fish: u8,
    //         other_necessities: Vec<String>,
    //     }

    //     let gleipnir = GleipnirC::builder()
    //         .roots_of("mountain".to_string())
    //         .breath_of_a_fish(1)
    //         .other_necessities(vec![
    //             "sound of a cat".to_string(),
    //             "beard of a woman".to_string(),
    //             "spittle of a bird".to_string(),
    //         ])
    //         .build();

    //     assert_eq!(gleipnir.roots_of, "mountain".to_string());
    //     assert_eq!(gleipnir.breath_of_a_fish, 1);
    //     assert_eq!(gleipnir.other_necessities.len(), 3);
    // }

    // #[test]
    // #[should_panic]
    // fn should_panic_when_field_is_missing() {
    //     #[derive(Builder)]
    //     struct GleipnirD {
    //         _roots_of: String,
    //     }

    //     GleipnirD::builder().build();
    // }

    // #[test]
    // fn should_generate_builder_for_struct_with_one_renamed_property() {
    //     #[derive(Builder)]
    //     struct Gleipnir {
    //         #[rename("tops_of")]
    //         roots_of: String,
    //     }

    //     let gleipnir = Gleipnir::builder().tops_of("mountains".to_string()).build();

    //     assert_eq!(gleipnir.roots_of, "mountains".to_string());
    // }

    // #[test]
    // fn should_generate_builder_for_struct_with_two_props_one_custom_name() {
    //     #[derive(Builder)]
    //     struct Gleipnir {
    //         #[rename("tops_of")]
    //         roots_of: String,
    //         breath_of_a_fish: u8,
    //     }

    //     let gleipnir = Gleipnir::builder()
    //         .tops_of("mountains".to_string())
    //         .breath_of_a_fish(1)
    //         .build();

    //     assert_eq!(gleipnir.roots_of, "mountains".to_string());
    //     assert_eq!(gleipnir.breath_of_a_fish, 1);
    // }

    // #[test]
    // fn should_generate_builder_for_struct_with_one_renamed_property() {
    //     #[derive(Builder)]
    //     struct Gleipnir {
    //         #[rename = "tops_of"]
    //         roots_of: String,
    //     }

    //     let gleipnir = Gleipnir::builder().tops_of("mountains".to_string()).build();

    //     assert_eq!(gleipnir.roots_of, "mountains".to_string());
    // }

    // #[test]
    // fn should_use_defaults_when_attribute_is_present() {
    //     #[derive(Builder)]
    //     #[builder_defaults]
    //     struct ExampleStructTwoFields {
    //         string_value: String,
    //         int_value: i32,
    //     }

    //     let example = ExampleStructTwoFields::builder().build();
    //     assert_eq!(example.string_value, String::default());
    //     assert_eq!(example.int_value, Default::default());
    // }
}
