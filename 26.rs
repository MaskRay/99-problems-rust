use std::rand::{thread_rng, Rng};

fn combinations<T: Clone>(mut a: Vec<T>, k: usize) -> Vec<Vec<T>> {
  let mut r = vec![];
  let mut b = vec![];
  fn go<T: Clone>(r: &mut Vec<Vec<T>>, b: &mut Vec<T>, a: &Vec<T>, i: usize, k: usize) {
    if k == 0 {
      r.push(b.clone());
    } else {
      for ii in range(i, a.len()) {
        b.push(a[ii].clone());
        go(r, b, a, ii+1, k-1);
        b.pop();
      }
    }
  }
  go(&mut r, &mut b, &a, 0, k);
  r
}

fn main() {
  println!("{:?}" , combinations(range(0us, 5).collect(), 3));
}
