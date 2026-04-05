use proconio::input;
use std::cmp::max;
fn main() {
    input! {
        n: usize,
        a: [i32; n],
        d: usize,
        lr: [(usize, usize); d],
    }
    let mut p = [0; 100000];
    let mut q = [0; 100000];

    p[0] = a[0];
    for i in 1..n {
        p[i] = max(p[i-1], a[i])
        }
    q[n-1] = a[n-1];
    for i in (0..n-1).rev() {
        q[i] = max(q[i+1], a[i])
        }

    for i in 0..d {
        println!("{}",max(p[lr[i].0-2], q[lr[i].1]))
    }
}

