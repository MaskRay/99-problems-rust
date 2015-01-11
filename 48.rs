use Bool::{Var,Not,And,Or};

enum Bool {
  Var(&'static str),
  Not(Box<Bool>),
  And(Box<Bool>, Box<Bool>),
  Or(Box<Bool>, Box<Bool>),
}

impl std::ops::Not for Bool {
  type Output = Bool;
  fn not(self) -> Bool {
    Not(Box::new(self))
  }
}

impl std::ops::BitAnd for Bool {
  type Output = Bool;
  fn bitand(self, rhs: Bool) -> Bool {
    And(Box::new(self), Box::new(rhs))
  }
}

impl std::ops::BitOr for Bool {
  type Output = Bool;
  fn bitor(self, rhs: Bool) -> Bool {
    Or(Box::new(self), Box::new(rhs))
  }
}

fn table(a: &[&str], e: &Bool) -> Vec<(Vec<bool>, bool)> {
  match e {
    &Var(ref v) => {
      let i = a.position_elem(v).unwrap();
      let mut r = vec![];
      let mut b = vec![];
      fn go(r: &mut Vec<(Vec<bool>, bool)>, b: &mut Vec<bool>, i: usize, n: usize, k: usize) {
        if k == n {
          r.push((b.clone(), b[i]));
        } else {
          b.push(false);
          go(r, b, i, n, k+1);
          *b.last_mut().unwrap() = true;
          go(r, b, i, n, k+1);
          b.pop();
        }
      }
      go(&mut r, &mut b, i, a.len(), 0);
      r
    },
    &Not(ref e) =>
      table(a, &**e).into_iter().map(|(b, r)| (b, !r)).collect(),
    &And(ref e0, ref e1) =>
      table(a, &**e0).into_iter().zip(table(a, &**e1).into_iter()).map(
        |((a,r0),(_,r1))| (a, r0 || r1)
      ).collect(),
    &Or(ref e0, ref e1) =>
      table(a, &**e0).into_iter().zip(table(a, &**e1).into_iter()).map(
        |((a,r0),(_,r1))| (a, r0 || r1)
      ).collect(),
  }
}

fn main() {
  println!("{:?}", table(&["a", "b", "c"], &(! (Var("a") & Var("b") | Var("c")))));
}
