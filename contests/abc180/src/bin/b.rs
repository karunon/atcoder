fn main() {
    proconio::input! {
        n: usize,
        v: [f64; n],
    }

    let manhattan_distance = {
        let mut d = 0.;
        for x in &v {
            d += x.abs();
        }
        d
    };

    let euclidean_distance = {
        let mut d = 0.;
        for x in &v {
            d += x.abs().powf(2.);
        }
        d.sqrt()
    };

    let chebyshev_distance = {
        let mut d = 0.;
        for x in &v {
            d = x.abs().max(d);
        }
        d
    };


    println!("{}", manhattan_distance);
    println!("{}", euclidean_distance);
    println!("{}", chebyshev_distance);
}

