fn dupli<T: Clone>(a: Vec<T>) -> Vec<T> {
  a.into_iter().flat_map(|x| vec![x.clone(),x].into_iter()).collect()
}

fn main() {
  let a = vec![1is, 1, 2, 3, 3, 4];
  println!("{:?}" , dupli(a));
}
