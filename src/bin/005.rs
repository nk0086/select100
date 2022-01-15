#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        a: i64, b: i64, mut c: i64,
        x: i64, y: i64
    };

    let c = 2 * c;
    let mut money = std::i64::MAX;

    for i in 0..=x.max(y) {
        let x_left = (x - i).max(0);
        let y_left = (y - i).max(0);
        money = money.min(x_left * a + y_left * b + i * c);
    }

    println!("{}", money);
}
