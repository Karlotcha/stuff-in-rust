extern crate stuff;
use stuff::my_functions::*;

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
