use std::collections::btree_map::{BTreeMap, Vacant, Occupied};

fn length_sort<T>(a: &mut Vec<Vec<T>>) {
  a.sort_by(|x,y| x.len().cmp(&y.len()));
}

fn frequency_sort<T>(a: &mut Vec<Vec<T>>) {
  let mut fs = BTreeMap::<usize, usize>::new();
  for x in a.iter() {
    match fs.entry(x.len()) {
      Vacant(e) => { e.insert(1); },
      Occupied(mut e) => *e.get_mut() += 1
    }
  }
  a.sort_by(|x,y| fs.get(&x.len()).unwrap().cmp(fs.get(&y.len()).unwrap()));
}

fn main() {
  let mut a = vec![vec![1i32,2], vec![3], vec![5], vec![], vec![], vec![]];
  length_sort(&mut a);
  println!("{:?}", a);
  frequency_sort(&mut a);
  println!("{:?}", a);
}
