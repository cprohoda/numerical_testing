pub fn main() {
    let range = Delta {
        start: 0.1,
        end: 0.5,
        steps: 7,
        current: 0,
    };
    range.for_each(|x| println!("{:?}", x));
}

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
