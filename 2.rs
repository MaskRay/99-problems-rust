fn main() {
  let a = vec![1i32,3,5];
  let len = a.len();
  println!("{:?}", &a[len-2..len]);
}
