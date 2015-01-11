fn gcd(mut a: usize, mut b: usize) -> usize {
  let mut t;
  while b != 0 {
    t = a%b;
    a = b;
    b = t;
  }
  a
}

fn phi(mut n: usize) -> usize {
  range(0, n).filter(|&i| gcd(i, n) == 1).count()
}

fn main() {
  for n in range(0, 12) {
    println!("phi({}) = {}", n, phi(n));
  }
}
