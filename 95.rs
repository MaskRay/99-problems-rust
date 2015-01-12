fn full_words(n: usize) -> String {
  let m = &["zero", "one", "two", "three", "four",
            "five", "six", "seven", "eight", "nine"];
  let mut r = String::new();
  for &b in n.to_string().as_bytes().iter() {
    if ! r.is_empty() {
      r = r + "-";
    }
    r = r + m[(b-48) as usize];
  }
  r
}

fn main() {
  println!("{}", full_words(815));
}
