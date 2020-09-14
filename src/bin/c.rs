#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let ans = (1900 * m + 100 * (n - m)) * 2u32.pow(m as u32) as usize;
    println!("{}", ans);
}
