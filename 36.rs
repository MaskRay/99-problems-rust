fn factors_mul(mut n: usize) -> Vec<(usize, usize)> {
  let mut r = vec![];
  let mut i = 2;
  while i*i <= n {
    if n % i == 0 {
      let mut c = 0;
      while n % i == 0 {
        n /= i;
        c += 1;
      }
      r.push((i, c));
    }
    i += 1;
  }
  if n > 1 {
    r.push((n, 1));
  }
  r
}

fn main() {
  for &n in (&[7, 12, 315]).iter() {
    println!("factors_mul({}) = {:?}", n, factors_mul(n));
  }
}
