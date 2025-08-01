fn test_divisibility_by_3_4(number: i32) -> i32 {
    let divisible_by_3 = number % 3 == 0;
    let divisible_by_4 = number % 4 == 0;

    if divisible_by_3 && divisible_by_4 {
        0
    } else if divisible_by_3 {
        1
    } else if divisible_by_4 {
        2
    } else {
        -1
    }
}

fn main() {
    println!("{}", test_divisibility_by_3_4(12)); // 0
    println!("{}", test_divisibility_by_3_4(9));  // 1
    println!("{}", test_divisibility_by_3_4(8));  // 2
    println!("{}", test_divisibility_by_3_4(7));  // -1
}
