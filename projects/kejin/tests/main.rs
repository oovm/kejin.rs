use std::collections::BTreeMap;
use enhance::{EnhanceLevel, EnhanceMap};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let mut mappings = BTreeMap::default();
    mappings.insert(10, EnhanceLevel::simple(0.5, 0.5, 0));
    let map = EnhanceMap::<String> {
        mapping:mappings,
    };
    println!("{}", map.as_matrix().as_wolfram(false));
    let json = serde_json::to_string_pretty( &map).unwrap();
    println!("{}", json);
}