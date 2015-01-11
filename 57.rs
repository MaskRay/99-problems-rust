use Tree::{Empty, Node};

#[derive(Clone, Show)]
enum Tree<T> {
  Empty,
  Node(T, Box<Tree<T>>, Box<Tree<T>>)
}

fn construct<T: Ord>(xs: Vec<T>) -> Tree<T> {
  fn insert<T: Ord>(t: Tree<T>, x: T) -> Tree<T> {
    match t {
      Empty =>
        Node(x, Box::new(Empty), Box::new(Empty)),
      Node(y, l, r) =>
        if x < y {
          Node(y, Box::new(insert(*l, x)), r)
        } else {
          Node(y, l, Box::new(insert(*r, x)))
        }
    }
  }
  xs.into_iter().fold(Empty, |t,x| insert(t, x))
}

fn main() {
  let a = vec![0,1];
  println!("{:?}", construct(a));
  let a = vec![1,2,0];
  println!("{:?}", construct(a));
}
