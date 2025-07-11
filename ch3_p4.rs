fn main() {
    let some_value = Some(42);

    if let Some(x) = some_value {
        println!("Matched value: {}", x);
    } else {
        println!("No value found.");
    }
}
