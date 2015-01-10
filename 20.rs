fn remove_at<T>(a: &mut Vec<T>, i: usize) -> &mut Vec<T> {
  a.remove(i);
  a
}

fn main() {
  let mut a = vec![1is, 1, 2, 3, 3, 4];
  println!("{:?}" , remove_at(&mut a, 2));
}
