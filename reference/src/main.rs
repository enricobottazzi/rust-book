
fn incr(n: &mut i32) {
    *n += 1;
  }
  fn main() {
    let mut n = 1;
    incr(& mut n);
    println!("{n}");
  }