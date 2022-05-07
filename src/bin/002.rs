#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize
    };

    let mut count = 0;
    for i in 1..=n {
        count += if i % 2 == 1 && check(i) == 8 { 1 } else { 0 };
    }

    println!("{}", count);
}

fn check(n: usize) -> usize {
    let mut count = 0;
    let mut tmp = 1;

    while tmp * tmp <= n {
        if n % tmp == 0 {
            count += 2;
            if tmp * tmp == n {
                count -= 1;
            }
        }

        tmp += 1;
    }

    count
}
