#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

use itertools::Itertools;
use std::io;

#[fastout]
fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: Vec<u32> = input
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        if (input[0], input[1]) == (0, 0) {
            break;
        }

        let n = input[0];
        let x = input[1];
        let mut ans = 0;
        for i in (1..=n).combinations(3) {
            let num: u32 = i.iter().sum();
            if num == x {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}
