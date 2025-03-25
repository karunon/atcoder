fn main() {
    proconio::input! {
        x: f64,
    }

    let ans = if x < 37.5 {
        3
    } else if x < 38.0 {
        2
    } else {
        1
    };

    println!("{}", ans);
}
