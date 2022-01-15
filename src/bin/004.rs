#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [[u64; m]; n]
    };

    let mut ans = 0;
    for s in 0..m - 1 {
        for t in s + 1..m {
            let mut tmp = 0;
            for i in 0..n {
                tmp += a[i][s].max(a[i][t]);
            }

            ans = ans.max(tmp);
        }
    }

    println!("{}", ans);
}
