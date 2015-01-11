use std::rand::{thread_rng, Rng};

fn rnd_permu<T>(a: Vec<T>) -> Vec<T> {
  let mut a = a;
  thread_rng().shuffle(a.as_mut_slice());
  a
}

fn main() {
  println!("{:?}" , rnd_permu(range(0us, 5).collect()));
}
