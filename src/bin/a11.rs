use proconio::input;

fn main(){
    input!{
        n: usize,
        x: i32,

        a: [i32; n],
    }
    let mut t1 = 0;
    let mut t2 = n-1;
    while t1 <= t2 {
        let m = (t1 + t2)/2;
        if x > a[m]{
            t1 = m + 1;
        }
        if x == a[m]{
            println!("{}",m+1);
            break;
        }
        if x < a[m]{
            t2 = m - 1;
        }

    }
}

