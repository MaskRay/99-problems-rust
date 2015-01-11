use std::rc::Rc;
use Tree::{Empty, Node};

#[derive(Clone, Show)]
enum Tree<T> {
  Empty,
  Node(T, Rc<Tree<T>>, Rc<Tree<T>>)
}

fn cbal_tree(n: usize) -> Vec<Rc<Tree<char>>> {
  if n == 0 {
    vec![Rc::new(Empty)]
  } else if n % 2 == 1 {
    let xs = cbal_tree(n/2);
    let mut r = vec![];
    for x in xs.iter() {
      for y in xs.iter() {
        r.push(Rc::new(Node('x', x.clone(), y.clone())));
      }
    }
    r
  } else {
    let xs = cbal_tree(n/2-1);
    let ys = cbal_tree(n/2);
    let mut r = vec![];
    for x in xs.iter() {
      for y in ys.iter() {
        r.push(Rc::new(Node('x', x.clone(), y.clone())));
        r.push(Rc::new(Node('x', y.clone(), x.clone())));
      }
    }
    r
  }
}

fn main() {
  println!("{:?}", cbal_tree(3));
  println!("{:?}", cbal_tree(4));
  println!("{}", cbal_tree(40).len() == 524288);
}
