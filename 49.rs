fn gray(n: usize) -> Vec<String> {
  match n {
    0 => vec![],
    1 => vec!["0".to_string(), "1".to_string()],
    _ => {
      let xs = gray(n-1);
      let zero = xs.iter().map(|x| format!["0{}", x]);
      let one = xs.iter().rev().map(|x| format!["1{}", x]);
      zero.chain(one).collect()
    }
  }
}

fn main() {
  println!("{:?}", gray(3));
}
