fn main() {
    proconio::input! {
        x: i32,
    }

    let ans = if x >= 30 {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}

