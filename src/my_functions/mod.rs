pub fn increment(i: i32) -> i32 { i + 1 }

pub fn add(a: i32, b: i32) -> i32 { a + b }

pub fn multiply(a: i32, b: i32) -> i32 { a * b }

pub fn square(a: i32) -> i32 { multiply(a,a) }

pub fn power(a: i32, n: i32) -> i32 {
  let mut r = 1;
  for i in 1..(n+1) { r = r * a }
  r
}
