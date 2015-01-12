fn queens(n: usize) {
  let mut xs = std::iter::repeat(0).take(n).collect();
  fn go(xs: &mut Vec<usize>, n: usize, i: usize, l: usize, m: usize, r: usize) {
    if i == 0 {
      for &x in xs.iter() {
        print!("{} ", x);
      }
      println!("");
    } else {
      let mask = (1<<n)-1-(l|m|r);
      for j in range(0, n) {
        let jj = 1 << j;
        if (mask & jj) != 0 {
          xs[i-1] = j;
          go(xs, n, i-1, (l|jj)<<1, m|jj, (r|jj)>>1);
        }
      }
    }
  }
  go(&mut xs, n, n, 0, 0, 0);
}

fn main() {
  queens(8);
}
