fn rang(x: isize, y: isize) -> Vec<isize> {
  if x < y {
    range(x, y+1).collect()
  } else {
    range(y, x+1).rev().collect()
  }
}

fn main() {
  println!("{:?}" , rang(4, 9));
  println!("{:?}" , rang(9, 4));
}
