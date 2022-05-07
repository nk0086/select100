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
    for i in 0..m - 1 {
        for j in i + 1..m {
            let mut tmp = 0;
            for k in 0..n {
                tmp += a[k][i].max(a[k][j]);
            }

            ans = ans.max(tmp);
        }
    }

    println!("{}", ans);
}
