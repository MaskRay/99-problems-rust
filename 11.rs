use Elem::{Unique, Several};

#[derive(Show)]
enum Elem<T> { Unique(T), Several(usize, T), }

fn encode_bis<T: Clone + Eq>(a: Vec<T>) -> Vec<Elem<T>> {
  let mut r: Vec<Elem<T>> = vec![];
  for x in a.into_iter() {
    let w =
      match r.last() {
        Some(&Unique(ref y)) if x == *y => Some(Several(2, y.clone())),
        Some(&Several(l, ref y)) if x == *y => Some(Several(l + 1, y.clone())),
        _ => None,
      };
    match w {
      None => r.push(Unique(x)),
      Some(w) => *r.last_mut().unwrap() = w,
    }
  }
  r
}

fn main() {
  let a = vec![1is, 1, 2, 3, 3, 4];
  println!("{:?}" , encode_bis(a));
}
