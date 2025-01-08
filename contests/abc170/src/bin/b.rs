fn main() {
    proconio::input! {
        x: i32,
        y: i32,
    }

    let ans = if y % 2 == 0 && y <= 4 * x && 2 * x <= y {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}

