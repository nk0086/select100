#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [u64; n],
        q: usize,
        t: [u64; q]
    };

    let mut ans = 0;
    for i in 0..q {
        if binary_search(&s, t[i]) {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn binary_search(array: &Vec<u64>, k: u64) -> bool {
    let mut ok = array.len() as i64;
    let mut ng = -1;

    while (ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        if array[mid as usize] < k {
            ng = mid;
        } else if array[mid as usize] > k {
            ok = mid;
        } else {
            return true;
        }
    }

    false
}

//問題の誤読実装
fn upper_bound(array: &Vec<u64>, k: u64) -> u64 {
    let mut ok = array.len() as i64;
    let mut ng = -1;

    while (ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        if array[mid as usize] <= k {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    ok as u64
}

fn lower_bound(array: &Vec<u64>, k: u64) -> u64 {
    let mut ok = array.len() as i64;
    let mut ng = -1;

    while (ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        if array[mid as usize] < k {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    ok as u64
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simple() {
        let v = vec![0, 1, 2, 2, 3, 4, 5];
        assert_eq!(upper_bound(&v, 2), 4);
    }

    #[test]
    fn u_max() {
        let v = vec![1, 2, 3];
        assert_eq!(upper_bound(&v, 5), 3);
    }

    #[test]
    fn l_max() {
        let v = vec![1, 2, 3];
        assert_eq!(lower_bound(&v, 5), 3);
    }
}
