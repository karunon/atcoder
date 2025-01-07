fn main() {
    proconio::input! {
        n: i32,
        m: i32,
    }

    let ans = if n == m {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}

