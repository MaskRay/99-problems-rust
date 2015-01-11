fn group<T: Clone>(a: &[T], mut ns: Vec<usize>) -> Vec<Vec<Vec<T>>> {
  let mut r = vec![];
  let n = ns.len();
  let mut b: Vec<Vec<T>> = std::iter::repeat(vec![]).take(n).collect();
  fn go<T: Clone>(r: &mut Vec<Vec<Vec<T>>>, b: &mut Vec<Vec<T>>,
                  a: &[T], ns: &mut Vec<usize>, k: usize) {
    if k == a.len() {
      r.push(b.clone());
    } else {
      for i in range(0, ns.len()) {
        if ns[i] > 0 {
          ns[i] -= 1;
          b[i].push(a[k].clone());
          go(r, b, a, ns, k+1);
          b[i].pop();
          ns[i] += 1;
        }
      }
    }
  };
  go(&mut r, &mut b, a, &mut ns, 0);
  r
}

fn main() {
  let a: Vec<isize> = range(0is, 5).collect();
  println!("{:?}", group(a.as_slice(), vec![2,3]));
}
