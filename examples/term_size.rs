extern crate termsize;

pub fn main() {
  termsize::get().map(|size| {
    println!("rows {} cols {}", size.rows, size.cols)
  });
}