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

// TODO - DRY these tests
#[test]
fn test_selection_sort() {
  let v = vec![13, 25, 3, 4543, 5, 0, 5, 1, 999];
  let ordered_v = vec![0, 1 ,3 ,5 , 5, 13, 25, 999, 4543];

  assert_eq!(ordered_v, selection_sort(&v));
  // assert_eq!([], selection_sort(&[]));
}

#[test]
fn test_insertion_sort() {
  let v = vec![13, 25, 3, 4543, 5, 0, 5, 1, 999];
  let ordered_v = vec![0, 1 ,3 ,5 , 5, 13, 25, 999, 4543];

  assert_eq!(ordered_v, insertion_sort(&v));
  // assert_eq!([], selection_sort(&[]));
}

#[test]
fn test_merge_sort() {
  let v = vec![13, 25, 3, 4543, 5, 0, 5, 1, 999];
  let ordered_v = vec![0, 1 ,3 ,5 , 5, 13, 25, 999, 4543];

  assert_eq!(ordered_v, merge_sort(&v));
  // assert_eq!([], selection_sort(&[]));
}

#[test]
fn test_quick_sort() {
  let v = vec![13, 25, 3, 4543, 5, 0, 5, 1, 999];
  let ordered_v = vec![0, 1 ,3 ,5 , 5, 13, 25, 999, 4543];

  assert_eq!(ordered_v, merge_sort(&v));
  // assert_eq!([], selection_sort(&[]));
}
