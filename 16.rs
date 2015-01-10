fn drop<T>(a: Vec<T>, n: usize) -> Vec<T> {
  a.into_iter().enumerate().filter_map(|(i,x)|
    if i%n < n-1 { Some(x) } else { None }
    ).collect()
}

fn main() {
  let a = vec![1is, 1, 2, 3, 3, 4];
  println!("{:?}" , drop(a, 3));
}
