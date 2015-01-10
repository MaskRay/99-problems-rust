use Elem::{Unique, Several};

#[derive(Show)]
enum Elem<T> {
  Unique(T),
  Several(usize, T)
}

fn decode<T: Clone>(a: Vec<Elem<T>>) -> Vec<T> {
  a.into_iter().flat_map(|e|
    match e {
      Unique(x) => vec![x],
      Several(n, x) => std::iter::repeat(x).take(n).collect()
    }.into_iter()
  ).collect()
}

fn main() {
  let a: Vec<Elem<isize>> = vec![Several(2us, 1is), Unique(2is), Several(2us, 3is), Unique(4is)];
  println!("{:?}", decode(a));
}
