fn is_prime(n: usize) -> bool {
  if n < 2 {
    return false;
  }
  let mut i = 2;
  while i*i <= n {
    if n%i == 0 {
      return false;
    }
    i += 1;
  }
  true
}

fn main() {
  for n in range(0, 20) {
    if is_prime(n) {
      print!("{} ", n);
    }
  }
}
