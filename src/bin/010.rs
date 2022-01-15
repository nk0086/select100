#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize, a: [usize; n], q: usize, m: [usize; q]
    };

    for i in 0..q {
        println!("{}", if solve(m[i], n, &a) { "Yes" } else { "No" });
    }
}

fn solve(mi: usize, n: usize, a: &Vec<usize>) -> bool {
    for i in 0..1 << n {
        let mut tmp = 0;
        for j in 0..n {
            if i & (1 << j) > 0 {
                tmp += a[j];
            }
        }

        if tmp == mi {
            return true;
        }
    }

    false
}
