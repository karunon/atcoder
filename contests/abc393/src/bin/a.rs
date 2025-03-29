fn main() {
    proconio::input! {
        s1: String,
        s2: String,
    }

    let ans = if s1 == "sick" && s2 == "sick" {
        1
    } else if s1 == "sick" && s2 == "fine" {
        2
    } else if s1 == "fine" && s2 == "sick" {
        3
    } else {
        4
    };

    println!("{}", ans);
}
