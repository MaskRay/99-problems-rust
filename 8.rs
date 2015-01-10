fn compress<T: Eq>(a: Vec<T>) -> Vec<T> {
  let mut r = vec![];
  for x in a.into_iter() {
    if r.is_empty() || r.last().unwrap() != &x {
      r.push(x)
    }
  }
  r
}

fn compress_lib<T: Eq>(mut a: Vec<T>) -> Vec<T> {
  a.dedup();
  a
}

fn main() {
  let a = vec![1i32, 1, 2, 3, 3, 4];
  println!("{:?}", compress(a.clone()));
  println!("{:?}", compress_lib(a));
}
