fn main() {
    proconio::input! {
        n: usize,
        mut l: [i32; n],
    }

    l.sort();

    let mut ans = 0;

    for k in 0..n {
        for j in 0..k {
            if l[k] == l[j] {
                continue;
            }
            for i in 0..j {
                if l[j] == l[i] {
                    continue;
                }
                if l[i] + l[j] > l[k] {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}

