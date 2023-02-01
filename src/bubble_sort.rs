use crate::util::swap_arr;

// A bubble sort implementation in Rust.
// Very easy to write, very inefficient.
pub fn bubble_sort(arr: &mut Vec<i32>) {
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                swap_arr(arr, j, j+ 1);
            }
        }
    }
}