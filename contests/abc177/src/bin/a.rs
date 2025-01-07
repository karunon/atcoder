fn main() {
    proconio::input! {
        d: i32,
        t: i32,
        s: i32,
    }

    let ans = if d <= s * t {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}

