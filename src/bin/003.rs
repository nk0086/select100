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

    let mut ans = 0;
    for i in 0..s.len() {
        for j in i..s.len() {
            if judge(&s[i..j + 1]) {
                ans = ans.max(j + 1 - i);
            }
        }
    }

    println!("{}", ans);
}

fn judge(ss: &str) -> bool {
    let set: HashSet<char> = vec!['A', 'T', 'G', 'C'].into_iter().collect();
    ss.chars()
        .map(|x| if set.contains(&x) { 1 } else { 0 })
        .sum::<usize>()
        == ss.len()
}
