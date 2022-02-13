use getter_macro::Getter;

#[derive(Getter)]
struct Sample {
    field1: String,
    field2: String,
}

fn main() {
    let sample = Sample {
        field1: "field 1".to_string(),
        field2: "field 2".to_string(),
    };
    println!("{}", sample.get_field1());
    println!("{}", sample.get_field2());
}
