fn main() {
    proconio::input! {
        s: i32,
        w: i32,
    }

    let ans = if s <= w {
        "unsafe"
    } else {
        "safe"
    };

    println!("{}", ans);
}

