use std::{env, fs};

fn add_two<const X: usize, const Y: usize>(mut input: [[i64; X]; Y]) -> [[i64; X]; Y] {
    input.iter_mut().for_each(|x_array| {
        x_array.iter_mut().for_each(|val| {
            *val += 2;
        })
    });

    input
}

pub fn main() {
    let array = [[0; 3]; 2];
    println!("{:?}", add_two(array));
}
