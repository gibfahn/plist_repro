fn main() {
    let test_value: plist::Value = serde_yaml::from_str("true").unwrap();

    // Should print:
    //   Value: Boolean(true)
    println!("Value: {:?}", test_value);
}
