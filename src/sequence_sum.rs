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
}

pub fn partial_sum<F: Fn(usize) -> f64>(n: usize, sequence: F) -> f64 {
    (0..n).map(|n| sequence(n)).sum()
}
