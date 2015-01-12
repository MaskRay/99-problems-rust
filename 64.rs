use Tree::{Empty, Node};

#[derive(Clone, Show)]
enum Tree<T> {
  Empty,
  Node(T, Box<Tree<T>>, Box<Tree<T>>)
}

fn layout_binary_tree<T>(t: &Tree<T>) -> Tree<(&T,usize,usize)> {
  fn go<T>(t: &Tree<T>, x: usize, y: usize) -> (usize, Tree<(&T,usize,usize)>) {
    match *t {
      Empty =>
        (x, Empty),
      Node(ref w, ref l, ref r) => {
        let (lx, lt) = go(&**l, x, y+1);
        let (rx, rt) = go(&**r, lx+1, y+1);
        (rx, Node((w, lx, y), Box::new(lt), Box::new(rt)))
      }
    }
  }
  go(t, 1, 1).1
}

fn main() {
  let e = Box::new(Empty);
  let a1 = Box::new(Node(1, e.clone(), e.clone()));
  let a2 = Box::new(Node(2, a1.clone(), e.clone()));
  let a3 = Box::new(Node(2, e.clone(), a1.clone()));
  let a = Node(1, a2.clone(), a2.clone());
  println!("{:?}", layout_binary_tree(&a));
}
