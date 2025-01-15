fn main() {
    proconio::input! {
        n: usize,
        v: [i64; n],
    }

    let mut ans = 0;
    let mut max_height = 0;

    for i in &v {
        max_height = max_height.max(*i);
        ans += max_height - i;
    }

    println!("{}", ans);
}

