use crate::util::swap_arr;

// The selection sort algorithm implemented in Rust.
pub fn sel_sort(arr: &mut Vec<i32>) {
    for i in 0..arr.len() - 1 {  // Length - 1
        let mut smallest = i;
        for j in i + 1..arr.len() {  // Length
            if arr[j] < arr[smallest] {
                smallest = j;
            }
        }
        swap_arr(arr, smallest, i);     
    }
}