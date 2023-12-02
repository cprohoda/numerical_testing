pub fn main() {
    let range = Delta {
        start: 0.0,
        end: 1.0,
        steps: 1_000_000,
        current: 0,
    };
    println!("{:?}", numerical_integration(range,
        |x| {
           x.powi(2)
        })
    );
}

#[derive(Clone)]
pub struct Delta {
    start: f64,
    end: f64,
    steps: usize,
    current: usize,
}

impl Iterator for Delta {
    type Item = (f64, f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.steps {
            return None;
        }
        let percent_start = (self.current as f64) / (self.steps as f64);
        let current_start = (self.end - self.start) * percent_start + self.start;

        let percent_end = ((self.current + 1) as f64) / (self.steps as f64); 
        let current_end = self.end.min(
            (self.end - self.start) * percent_end + self.start
        );

        let current_mid = (current_end + current_start) / 2.0;
        self.current += 1;

        Some((current_start, current_mid, current_end))
    }
}

pub fn numerical_integration<F: Fn(f64) -> f64>(range: Delta, f: F) -> f64 {
    range.map(|x| {
        f(x.1) * (x.2 - x.0)
    }).sum()
}

pub fn truncated_taylor_expansion<F: Fn(f64) -> f64>(f: F, dx: f64) -> impl Fn(f64, f64) -> f64 {
    // f(x) = sum_n: 1/(n!) * f^n(a) * (x - a)^n
    // currently only the first two terms
    // truncated taylor expansion:
    // f(x) = sum_N-1_terms + 1/N! * f^N(y) * (x - a)^N
    // where y is between x and a 
    move |x, a| {
        // Centered difference approximations
        let first_order = (f(a+dx) - f(a-dx)) / (2. * dx);
        let second_order = (f(a+dx) - 2. * f(a) + f(a-dx)) / (2. * dx.powi(2));
        f(a) + first_order * (x - a) + 0.5 * second_order * (x - a).powi(2)
    }
}
