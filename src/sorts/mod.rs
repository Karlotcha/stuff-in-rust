pub fn vect_to_string(v: &Vec<i32>) -> String {
  let mut r = "".to_string();
  for i in v { r = r + &i.to_string() + " " }
  r
}

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
  let mut new_v = v.to_vec();
  // let n = new_v.len();

  new_v
}

pub fn merge_sort(v: &Vec<i32>) -> &Vec<i32> { v }

pub fn quick_sort(v: &Vec<i32>) -> &Vec<i32> { v }

// helpers

// swap v[i] and v[j]
pub fn swap(mut v: Vec<i32>, i: usize, j: usize) -> Vec<i32> {
  let a = v[i];
  v[i] = v[j];
  v[j] = a;

  v
}
