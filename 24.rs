use std::rand::{thread_rng, sample};

fn lotto_select(k: usize, n: usize) -> Vec<usize> {
  sample(&mut thread_rng(), range(0, n), k)
}

fn main() {
  println!("{:?}" , lotto_select(3, 5));
}
