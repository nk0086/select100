#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        s: String
    };

    let mut v = vec![0; s.len() + 1];

    let ss = ['A', 'C', 'G', 'T'];
    let set: HashSet<&char> = ss.iter().collect();
    for (i, si) in s.chars().enumerate() {
        if set.contains(&si) {
            v[i + 1] = v[i] + 1;
        }
    }

    println!("{}", v.iter().max().unwrap());
}
