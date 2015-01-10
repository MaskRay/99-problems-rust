fn rotate<T: Clone>(a: Vec<T>, n: usize) -> Vec<T> {
  let t = n % a.len();
  let mut r = vec![];
  r.push_all(a.slice_from(t));
  r.push_all(a.slice_to(t));
  r
}

fn main() {
  let a = vec![1is, 1, 2, 3, 3, 4];
  println!("{:?}" , rotate(a, 2));
}
