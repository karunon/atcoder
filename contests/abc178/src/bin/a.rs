fn main() {
    proconio::input! {
        x: i32,
    }

    let ans = if x == 0 {
        "1"
    } else {
        "0"
    };

    println!("{}", ans);
}

