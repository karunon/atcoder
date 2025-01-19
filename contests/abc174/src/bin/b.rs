fn main() {
    proconio::input! {
        n: usize,
        d: f64,
        v: [(f64, f64); n],
    }

    let mut ans = 0;
    for &(x, y) in &v {
        if d >= x.hypot(y) {
            ans += 1;
        }
    }

    println!("{}", ans);
}

