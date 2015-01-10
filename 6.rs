#![feature(advanced_slice_patterns)]
fn is_palindrome_rec<T: Eq>(a: &[T]) -> bool {
  match a {
    [] | [_] => true,
    [ref x, xs.., ref y] => x == y && is_palindrome_rec(xs),
  }
}

fn is_palindrome_iter<T: Eq>(a: &[T]) -> bool {
  a.iter().zip(a.iter().rev()).all(|(x,y)| x == y)
}

fn main() {
  let a = &[1i32,3,1];
  println!("{}", is_palindrome_rec(a));
  println!("{}", is_palindrome_iter(a));
}
