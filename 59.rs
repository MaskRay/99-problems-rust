use std::rc::Rc;
use Tree::{Empty, Node};

#[derive(Clone, Show)]
enum Tree<T> {
  Empty,
  Node(T, Rc<Tree<T>>, Rc<Tree<T>>)
}

fn hbal_tree(n: usize) -> Vec<Tree<char>> {
  match n {
    0 =>
      vec![Empty],
    1 =>
      vec![Node('x', Rc::new(Empty), Rc::new(Empty))],
    _ => {
      let mut r = vec![];
      let xs: Vec<Rc<Tree<char>>> = hbal_tree(n-2).into_iter().map(|t| Rc::new(t)).collect();
      let ys: Vec<Rc<Tree<char>>> = hbal_tree(n-1).into_iter().map(|t| Rc::new(t)).collect();
      for x in ys.iter() {
        for y in xs.iter() {
          r.push(Node('x', x.clone(), y.clone()));
          r.push(Node('x', y.clone(), x.clone()));
        }
        for y in ys.iter() {
          r.push(Node('x', x.clone(), y.clone()));
        }
      }
      r
    }
  }
}

fn main() {
  println!("{}", hbal_tree(3).len() == 15);
}
