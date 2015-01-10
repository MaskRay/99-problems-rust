use std::cmp::min;

fn insert_at<T>(x: T, a: Vec<T>, i: usize) -> Vec<T> {
  let mut a = a;
  let l = a.len();
  a.insert(min(i, l), x);
  a
}

fn main() {
  let a = vec![1is, 1, 2, 3, 3, 4];
  println!("{:?}" , insert_at(0, a, 1));
}
