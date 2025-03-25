fn main() {
    proconio::input! {
        s: String,
    }

    for c in s.chars() {
        if c == '2' {
            print!("2");
        }
    }
    println!();
}
