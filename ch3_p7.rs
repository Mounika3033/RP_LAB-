fn main() {
    let mut count = 0;
    let mut num = 0;

    while num < 20 {
        num += 3;
        count += 1;
    }

    println!("Loop ran {} times.", count);
}
