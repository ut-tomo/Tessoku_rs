use std::io;

fn lin_search (arr: &[i32], target: i32) -> bool {
  for &v in arr { 
    if v == target {
      return true;
    }
  }
  false
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let x: i32 = it.next().unwrap().parse().unwrap();

    s.clear();
    io::stdin().read_line(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(it.next().unwrap().parse::<i32>().unwrap());
    }

    if lin_search(&arr, x) {
        println!("Yes");
    } else {
        println!("No");
    }
}