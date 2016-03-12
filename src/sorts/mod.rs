pub fn selection_sort(v: &Vec<i32>) -> Vec<i32> {
  let mut new_v = v.to_vec();
  let n = new_v.len();

  for i in 0..n {
    let mut index_min = i;

    for j in i..n {
      if new_v[j] < new_v[index_min] { index_min = j }
    }

    new_v = swap(new_v, i, index_min);
  }

  new_v
}

pub fn insertion_sort(v: &Vec<i32>) -> Vec<i32> {
  let mut new_v = vec!();
  let n = new_v.len();

  for e in v { new_v = insert(new_v, *e) }

  new_v
}

pub fn merge_sort(v: &Vec<i32>) -> Vec<i32> {
  let mut new_v = v.to_vec();

  if new_v.len() <= 1 {
    new_v
  } else {
    let (left, right) = split(new_v);

    merge(merge_sort(&left), merge_sort(&right))
  }
}

pub fn quick_sort(v: &Vec<i32>) -> &Vec<i32> { v }

// helpers

// no idea if that's the Rust way (???)
pub fn vect_to_string(v: &Vec<i32>) -> String {
  let mut r = "".to_string();
  for i in v { r = r + &i.to_string() + " " }
  r
}

// swap v[i] and v[j]
pub fn swap(mut v: Vec<i32>, i: usize, j: usize) -> Vec<i32> {
  let a = v[i];
  v[i] = v[j];
  v[j] = a;

  v
}

// insert element e inside vector v
pub fn insert(mut v: Vec<i32>, e: i32) -> Vec<i32> {
  v.push(e);
  let n = v.len();

  for i in 0..n {
    if v[n-1] < v[i] { v = swap(v, n-1, i) }
  }

  v
}

// split an array into 2 of similar size
pub fn split(v: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
  let n = v.len();

  if n <= 1 {
    (v, vec!())
  } else {
    let (mut left, mut right) = (vec!(), vec!());

    for i in 0..n {
      if i < n/2 {
        left.push(v[i])
      } else {
        right.push(v[i])
      }
    }

    (left, right)
  }
}

// merge 2 ordered arrays into one
pub fn merge(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
  let mut v = vec!();
  let (mut i, mut j) = (0, 0);
  let (m, n) = (left.len(), right.len());

  if m == 0 { return right }
  if n == 0 { return left  }

  while i < m || j < n {
    if i == m  { v.push(right[j]); j=j+1; continue; }
    if j == n  { v.push(left[i]);  i=i+1; continue; }

    if left[i] <= right[j] {
      v.push(left[i]);
      i=i+1;
    } else {
      v.push(right[j]);
      j=j+1;
    }
  }

  v
}
