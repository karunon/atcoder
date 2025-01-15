fn main() {
    proconio::input! {
        n: usize,
        v: [(i64, i64); n],
    }

    let mut ans = 0;
    for (a, b) in &v {
        ans += (a + b) * (b - a + 1) / 2;
    }

    println!("{}", ans);
}

