extern crate stuff;
use stuff::my_functions::*;
use stuff::sorts::*;

#[test]
fn test_increment() {
  assert_eq!(2, increment(1));
}

#[test]
fn test_add() {
  assert_eq!(3, add(1,2));
}

#[test]
fn test_multiply() {
  assert_eq!(6, multiply(2,3));
}

#[test]
fn test_square() {
  assert_eq!(49, square(7));
}

#[test]
fn test_power() {
  assert_eq!(32, power(2,5));
  assert_eq!(1, power(2,0));
  assert_eq!(2, power(2,1));
}

fn test_sort_algorithm(sort_function:  fn(&Vec<i32>) -> Vec<i32>) {
  let v = vec![13, 25, 3, 4543, 5, 0, 5, 1, 999];
  let ordered_v = vec![0, 1 ,3 ,5 , 5, 13, 25, 999, 4543];
  let empty: Vec<i32> = vec!();

  assert_eq!(ordered_v, sort_function(&v));
  assert_eq!(empty, sort_function(&empty));
}

#[test]
fn test_selection_sort() {
  test_sort_algorithm(selection_sort);
}

#[test]
fn test_insertion_sort() {
  test_sort_algorithm(insertion_sort);
}

#[test]
fn test_merge_sort() {
  test_sort_algorithm(merge_sort);
}

#[test]
fn test_quick_sort() {
  test_sort_algorithm(quick_sort);
}
