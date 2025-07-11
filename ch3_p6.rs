fn main() {
    'outer: for i in 1..=10 {
        for j in 1..=10 {
            if j == 6 {
                break 'outer;
            }
            print!("{}x{} = {}\t", i, j, i * j);
        }
        println!();
    }
}
