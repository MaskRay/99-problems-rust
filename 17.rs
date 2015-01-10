fn split<'a, T>(a: &'a [T], n: usize) -> (&'a [T], &'a [T]) {
  a.split_at(std::cmp::min(n, a.len()))
}

fn main() {
  let a = &[1is, 1, 2, 3, 3, 4];
  println!("{:?}" , split(a, 13));
}
