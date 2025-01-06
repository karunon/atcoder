fn main() {
    proconio::input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }

    let rad = (60. * h - 11. * m) / 360. * std::f64::consts::PI;
    println!("{}", (a.powf(2.) + b.powf(2.) - 2. * a * b * rad.cos()).sqrt());
}

