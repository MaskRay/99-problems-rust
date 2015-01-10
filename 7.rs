use Node::{One, Many};

enum Node<T> {
  One(T),
  Many(Vec<Node<T>>),
}

fn flatten<T>(a: Node<T>) -> Vec<T> {
  match a {
    One(x) => vec![x],
    Many(xs) => xs.into_iter().flat_map(|x| flatten(x).into_iter()).collect(),
  }
}

fn main() {
  let a = Many(vec![Many(vec![One(1i32),One(2)]),One(3),One(4)]);
  println!("{:?}", flatten(a));
}
