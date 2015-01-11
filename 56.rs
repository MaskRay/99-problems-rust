use std::rc::Rc;
use Tree::{Empty, Node};

enum Tree<T> {
  Empty,
  Node(T, Rc<Tree<T>>, Rc<Tree<T>>)
}

fn is_symmetric<T>(t: &Tree<T>) -> bool {
  fn f<T>(t0: &Tree<T>, t1: &Tree<T>) -> bool {
    match (t0, t1) {
      (&Empty, &Empty) => true,
      (&Node(_, ref a, ref b), &Node(_, ref c, ref d)) =>
        f(&**a, &**d) && f(&**b, &**c),
      _ => false
    }
  }

  match *t {
    Empty => true,
    Node(_, ref l, ref r) => f(&**l, &**r)
  }
}

fn main() {
  let e = Rc::new(Empty);
  let a1 = Rc::new(Node(1, e.clone(), e.clone()));
  let a2 = Rc::new(Node(2, a1.clone(), e.clone()));
  let a3 = Rc::new(Node(2, e.clone(), a1.clone()));
  let a = Node(1, a2.clone(), a2.clone());
  let b = Node(1, a3.clone(), a2.clone());
  println!("{}", is_symmetric(&a));
  println!("{}", is_symmetric(&b));
}
