fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
    }

    for i in 0..n - 1 {
        if a[i] >= a[i + 1] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
