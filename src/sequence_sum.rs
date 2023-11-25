pub fn main() {
    // let max_n = 1_000_000_000usize;
    // println!(
    //     "n: {:?} -> {:.32}",
    //     max_n,
    //     partial_sum(
    //         max_n,
    //         |n| {
    //             2f64.powi(-1 * n as i32)
    //         },
    //     ),
    // );

    // let taylor_num = 0.8;
    // println!(
    //     "x: {:?} -> {:.32}",
    //     taylor_num,
    //     taylor_sum(taylor_num),
    // );

    let harmonic_n = 100;
    println!(
        "harmonic sum \nn: {:?} -> {:.32}",
        harmonic_n,
        harmonic_sum(harmonic_n),
    )
}

pub fn harmonic_sum(n: usize) -> f64 {
    partial_sum(n, |n| {
        if n > 0 {
            1./(n as f64)
        } else {
            0.
        }
    })
}

pub fn taylor_sum(x: f64) -> f64 {
    if 0. < x && x < 1. {
        1./(1.-x)
    } else {
        partial_sum(1_000, |n| {
            x.powi(n as i32)
        })
    }
}

pub fn partial_sum<F: Fn(usize) -> f64>(n: usize, sequence: F) -> f64 {
    (0..n).map(|n| sequence(n)).sum()
}
