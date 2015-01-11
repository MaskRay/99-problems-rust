use Bool::{Var,Not,And,Or};

enum Bool {
  Var(&'static str),
  Not(Box<Bool>),
  And(Box<Bool>, Box<Bool>),
  Or(Box<Bool>, Box<Bool>),
}

fn table(a: &str, b: &str, e: &Bool) -> Vec<(bool,bool,bool)> {
  match e {
    &Var(ref v) =>
      if **v == *a {
        vec![ (false, false, false)
            , (false, true, false)
            , (true, false, true)
            , (true, true, true)
            ]
      } else {
        vec![ (false, false, false)
            , (false, true, true)
            , (true, false, false)
            , (true, true, true)
            ]
      },
    &Not(ref e) =>
      table(a, b, &**e).into_iter().map(|(a,b,r)| (a,b,!r)).collect(),
    &And(ref e0, ref e1) =>
      table(a, b, &**e0).into_iter().zip(table(a, b, &**e1).into_iter()).map(
        |((a,b,r0),(_,_,r1))| (a,b,r0&&r1)
      ).collect(),
    &Or(ref e0, ref e1) =>
      table(a, b, &**e0).into_iter().zip(table(a, b, &**e1).into_iter()).map(
        |((a,b,r0),(_,_,r1))| (a,b,r0||r1)
      ).collect(),
  }
}

fn main() {
  let x = Box::new(Var("a"));
  let y = Box::new(Var("b"));
  println!("{:?}", table("a","b", &And(x, y)));
  let x = Box::new(Var("a"));
  let y = Box::new(Var("b"));
  println!("{:?}", table("a","b", &Or(x, y)));
}
