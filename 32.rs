fn gcd(mut a: usize, mut b: usize) -> usize {
  let mut t;
  while b != 0 {
    t = a%b;
    a = b;
    b = t;
  }
  a
}

fn main() {
  let s = &[(3,6),(6,9),(5,8),(15,20),(24,36)];
  for &(a,b) in s.iter() {
    println!("gcd({},{}) = {}", a, b, gcd(a, b));
  }
}
