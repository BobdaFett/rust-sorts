use crate::util::swap_arr;

// I'm not honestly totally sure how this algorithm works.

// The quick sort algorithm implemented in Rust.
pub fn quick_sort(arr: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        // pi stands for partitioning index
        let pi = partition(arr, low, high);
        quick_sort(arr, low, pi - 1);
        quick_sort(arr, pi, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low as i32 - 1;  // can't have a negative usize?
    
    for j in low..=high-1 {
        if arr[j] < pivot {
            i += 1;
            swap_arr(arr, i as usize, j);
        }
    }
    
    swap_arr(arr, (i + 1) as usize, high);
    return (i + 1) as usize;
}