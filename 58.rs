use std::rc::Rc;
use Tree::{Empty, Node};

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

fn sym_cbal_trees(n: usize) -> Vec<Rc<Tree<char>>> {
  cbal_tree(n).into_iter().filter(|t| is_symmetric(&**t)).collect()
}

fn main() {
  println!("{}", sym_cbal_trees(57).len() == 256);
}
