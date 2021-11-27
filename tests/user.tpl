mod definition;
use definition::{{{types}}};

fn main() {{
    let item: {ser_type} = include!("generated.rs");
    assert_eq!(item, {value});
}}