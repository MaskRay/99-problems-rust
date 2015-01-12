use Tree::{Empty, Node};

#[derive(Clone, Show, PartialEq)]
enum Tree<T> {
  Empty,
  Node(T, Box<Tree<T>>, Box<Tree<T>>)
}

fn leaves<T: PartialEq>(t: &Tree<T>) -> Vec<&Tree<T>> {
  match *t {
    Empty => vec![],
    Node(_, ref l, ref r) =>
      if **l == Empty && **r == Empty {
        vec![t]
      } else {
        leaves(&**l).into_iter().chain(leaves(&**r).into_iter()).collect()
      }
  }
}

fn main() {
  let e = Box::new(Empty);
  let a1 = Box::new(Node(1, e.clone(), e.clone()));
  let a2 = Box::new(Node(2, a1.clone(), e.clone()));
  let a3 = Box::new(Node(2, e.clone(), a1.clone()));
  let a = Node(1, a2.clone(), a2.clone());
  println!("{:?}", leaves(&a));
}
