fn main() {
    let a = 10;
    let b = 5;
    let operator = '+';

    match operator {
        '+' => println!("Sum = {}", a + b),
        '-' => println!("Difference = {}", a - b),
        '*' => println!("Product = {}", a * b),
        '/' => println!("Quotient = {}", a / b),
        '%' => println!("Remainder = {}", a % b),
        _ => println!("Invalid operator"),
    }
}
