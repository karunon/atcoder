fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
    }

    for i in 0..n - 2 {
        if a[i] == a[i + 1] && a[i + 1] == a[i + 2] {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
