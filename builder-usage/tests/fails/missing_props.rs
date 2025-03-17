use builder_macro::Builder;

#[derive(Builder)]
struct Gleipnir {
    roots_of: String,
    breath_of_a_fish: u8,
    anything_else: bool,
}

fn main() {
    let gleipnir = Gleipnir::builder()
        .tops_of("mountains".to_string())
        .breath_of_a_fish(1)
        // .anything_else(true)
        .build();
}
