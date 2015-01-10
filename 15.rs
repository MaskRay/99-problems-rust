fn repli<T: Clone>(n: usize, a: Vec<T>) -> Vec<T> {
  a.into_iter().flat_map(|x| std::iter::repeat(x).take(n)).collect()
}

fn main() {
  let a = vec![1is, 1, 2, 3, 3, 4];
  println!("{:?}" , repli(3, a));
}
