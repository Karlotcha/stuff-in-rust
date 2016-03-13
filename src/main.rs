// use if lib.rs is not present
// mod my_functions;
// use my_functions::*;

mod sorts;
use sorts::*;

// extern crate stuff;
// use stuff::my_functions::*;

fn main() {
  // println!("increment 1 => {}", increment(1));
  // println!("add 1 2 => {}", add(1,2));
  // println!("multiply 2 3 => {}", multiply(2,3));
  // println!("square 7 => {}", square(7));
  // println!("2 power 5 => {}", power(2,5));
  // println!("2 power 0 => {}", power(2,0));
  // println!("2 power 1 => {}", power(2,1));

  let v = vec![13, 25, 3, 4543, 5, 0, 5, 1, 999];

  println!("selection_sort => {}", vect_to_string(&selection_sort(&v)));
  println!("insertion_sort => {}", vect_to_string(&insertion_sort(&v)));
  println!("merge_sort     => {}", vect_to_string(&merge_sort(&v)));
  println!("quick_sort     => {}", vect_to_string(&quick_sort(&v)));
}
