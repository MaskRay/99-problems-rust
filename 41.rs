fn primes(bgn: usize, end: usize) -> Vec<usize> {
  let mut ps = vec![2];
  let mut i = 3;
  loop {
    if i >= end {
      break;
    }
    let mut flag = true;
    for &p in ps.iter() {
      if p*p > i {
        break;
      }
      if i % p == 0 {
        flag = false;
        break;
      }
    }
    if flag {
      ps.push(i);
    }
    i += 2;
  }
  ps.into_iter().skip_while(|&p| p < bgn).collect()
}

fn goldbach(n: usize) -> Option<(usize, usize)> {
  let ps = primes(2, n);
  for &p in ps.iter() {
    if ps.binary_search(&(n-p)).is_ok() {
      return Some((p, n-p));
    }
  }
  None
}

fn goldbach_list(bgn: usize, end: usize) -> Vec<(usize, Option<(usize, usize)>)>{
  std::iter::range_step(bgn+bgn%2, end, 2).map(|x|
    (x, goldbach(x))
  ).collect()
}

fn goldbach_limit(bgn: usize, end: usize, lim: usize) -> usize {
  goldbach_list(bgn, end).into_iter().filter(|&(_,x)|
    match x {
      Some((a, b)) if a > lim && b > lim => true,
      _ => false,
    }
  ).count()
}

fn main() {
  println!("{:?}", goldbach_list(3, 21));
  println!("{}", goldbach_limit(3, 21, 4));
}
