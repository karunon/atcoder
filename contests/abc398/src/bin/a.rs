fn main() {
    proconio::input! {
        n: i32,
    }

    let mut ans = String::new();

    if n % 2 == 1 {
        for i in 0..(n - 1) / 2 {
            ans.push_str("-");
        }
        ans.push_str("=");
        for i in 0..(n - 1) / 2 {
            ans.push_str("-");
        }
    } else {
        for i in 0..(n - 2) / 2 {
            ans.push_str("-");
        }
        ans.push_str("==");
        for i in 0..(n - 2) / 2 {
            ans.push_str("-");
        }
    }
    println!("{}", ans);
}
