const dx: &'static [isize] = &[1,2,2,1,-1,-2,-2,-1];
const dy: &'static [isize] = &[2,1,-1,-2,-2,-1,1,2];

fn knights(n: isize, x: isize, y: isize) -> Vec<(isize,isize)> {
  let mut moves = std::iter::repeat((0,0)).take((n*n) as usize).collect();
  let mut vis = std::iter::repeat(false).take((n*n) as usize).collect();
  fn go(moves: &mut Vec<(isize,isize)>, vis: &mut Vec<bool>, n: isize, k: isize, x: isize, y: isize) -> bool {
    let mut r = false;
    moves[(k-1) as usize] = (x, y);
    vis[(x*n+y) as usize] = true;
    if k == n*n {
      r = true;
    } else {
      let oc = 8;
      let mut cand = vec![];
      for i in range(0, 8) {
        let on = |&: x: isize, y: isize| {
          0 <= x && x < n && 0 <= y && y < n && ! vis[(x*n+y) as usize]
        };
        let cnt = |&: x: isize, y: isize| {
          let mut c = 0;
          for i in range(0, 8) {
            let xx = x+dx[i as usize];
            let yy = y+dy[i as usize];
            if on(xx, yy) { c += 1 }
          }
          c
        };
        let xx = x+dx[i];
        let yy = y+dy[i];
        if on(xx, yy) {
          let c = cnt(xx, yy);
          cand.push((c, xx, yy));
        }
      }
      for (_, xx, yy) in cand.into_iter() {
        if go(moves, vis, n, k+1, xx, yy) {
          r = true;
          break;
        }
      }
    }
    vis[(x*n+y) as usize] = false;
    r
  }
  go(&mut moves, &mut vis, n, 1, x, y);
  moves
}

fn main() {
  let moves = knights(5, 0, 0);
  for (x, y) in moves.into_iter() {
    print!("{},{} ", x, y);
  }
  println!("");
}
