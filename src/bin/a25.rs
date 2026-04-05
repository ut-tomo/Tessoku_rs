use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut dp = vec![vec![0_usize; w]; h];
    dp[0][0] = 1;

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                dp[i][j] = 0;
                continue;
            }
            if i > 0 {
                dp[i][j] += dp[i - 1][j];
            }
            if j > 0 {
                dp[i][j] += dp[i][j - 1];
            }
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
