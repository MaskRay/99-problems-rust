fn rle<T: Eq>(a: Vec<T>) -> Vec<(usize, T)> {
  let mut r: Vec<(usize, T)> = vec![];
  for x in a.into_iter() {
    let mut flag = true;
    if let Some(&mut (ref mut l, ref y)) = r.last_mut() {
      if x == *y {
        flag = false;
        *l += 1;
      }
    }
    if flag {
      r.push((1, x))
    }
  }
  r
}

fn main() {
  let a = vec![1i32, 1, 2, 3, 3, 4];
  println!("{:?}", rle(a));
}
