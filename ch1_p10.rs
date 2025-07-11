fn main() {
    let string_literal = "Hello, Rust!";  // String literal (immutable)
    let string_object = string_literal.to_string();  // Converting the string literal to a String object

    println!("String literal: {}", string_literal);
    println!("String object: {}", string_object);

    // Alternatively, you can also use String::from() to convert a string literal to a String object
    let string_object_from = String::from(string_literal);
    println!("String object from String::from(): {}", string_object_from);
}
