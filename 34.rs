fn phi(mut n: usize) -> usize {
  let mut r = n;
  let mut i = 2;
  while i*i <= n {
    if n%i == 0 {
      while n % i == 0 {
        n /= i
      }
      r = r/i*(i-1);
    }
    i += 1;
  }
  if n > 1 {
    r = r/n*(n-1);
  }
  r
}

fn main() {
  for n in range(0, 12) {
    println!("phi({}) = {}", n, phi(n));
  }
}
