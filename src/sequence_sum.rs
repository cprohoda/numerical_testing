pub fn main() {
    let max_n = 1_000_000_000usize;
    println!(
        "n: {:?} -> {:.32}",
        max_n,
        partial_sum(
            max_n,
            |n| {
                2f64.powi(-1 * n as i32)
            },
        ),
    );

    let taylor_num = 0.8;
    println!("{:.32}", taylor_num);
    println!(
        "x: {:?} -> {:.32}",
        taylor_num,
        taylor_sum(taylor_num),
    );
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
