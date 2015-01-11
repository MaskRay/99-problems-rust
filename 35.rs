fn factors(mut n: usize) -> Vec<usize> {
  let mut r = vec![];
  let mut i = 2;
  while i*i <= n {
    while n % i == 0 {
      n /= i;
      r.push(i);
    }
    i += 1;
  }
  if n > 1 {
    r.push(n);
  }
  r
}

fn main() {
  for n in range(0, 21) {
    println!("factors({}) = {:?}", n, factors(n));
  }
}
