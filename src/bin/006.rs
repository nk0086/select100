#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    };

    let mut ans = 0;
    for num in 0..1000 {
        let tmp = "000".to_owned() + &num.to_string();
        let tmp = &tmp[tmp.len() - 3..];

        let mut count = 0;
        let mut index = 0;
        for si in s.iter() {
            if index == 3 {
                break;
            }

            if si.to_string() == tmp[index..index + 1] {
                count += 1;
                index += 1;
            }
        }

        if count == 3 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
