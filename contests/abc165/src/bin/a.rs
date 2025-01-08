fn main() {
    proconio::input! {
        k: i32,
        a: i32,
        b: i32,
    }

    let c = (b / k) * k;
    let ans = if c >= a {
        "OK"
    } else {
        "NG"
    };

    println!("{}", ans);
}

