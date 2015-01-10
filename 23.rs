use std::rand::{thread_rng, sample};

fn rnd_select<'a, T>(a: &'a [T], n: usize) -> Vec<&'a T> {
  sample(&mut thread_rng(), a.iter(), n)
}

fn main() {
  let a = &[1i32, 2, 3, 4, 5];
  println!("{:?}" , rnd_select(a, 3));
}
