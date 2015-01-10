fn pack<T: Eq>(a: Vec<T>) -> Vec<Vec<T>> {
  let mut r: Vec<Vec<T>> = vec![];
  for x in a.into_iter() {
    if r.is_empty() || r.last().unwrap().last().unwrap() != &x {
      r.push(vec![x])
    } else {
      r.last_mut().unwrap().push(x)
    }
  }
  r
}

fn main() {
  let a = vec![1i32, 1, 2, 3, 3, 4];
  println!("{:?}", pack(a));
}
