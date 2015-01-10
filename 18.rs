use std::cmp::min;

fn slice<'a, T>(a: &'a [T], i: usize, j: usize) -> &'a [T] {
  a.slice(min(i, a.len()), min(j, a.len()))
}

fn main() {
  let a = &[1is, 1, 2, 3, 3, 4];
  println!("{:?}" , slice(a, 2, 5));
}
