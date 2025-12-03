use std::io;

fn count (n: i32, target: i32) -> i32{
  let mut count = 0;
  let mut min_sec: i32 ;
  let mut max_sec: i32 ;
  for i in 1..std::cmp::min(n+1, target-1){
    min_sec = std::cmp::max(1, target - i - n);
    max_sec = std::cmp::min(n, target - i - 1);
    count += std::cmp::max(0, max_sec - min_sec + 1);
  }
  count 
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut iter = line.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let x: i32 = iter.next().unwrap().parse().unwrap();

    println!("{}", count(n, x));
}