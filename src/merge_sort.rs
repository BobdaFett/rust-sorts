// The end should be the size of the array - 1.
pub fn merge_sort(arr: &mut Vec<i32>, begin: i32, end: i32) {
    if begin >= end { return }
    
    let mid = begin + (end - begin) / 2;
    merge_sort(arr, begin, mid);
    merge_sort(arr, mid + 1, end);
    merge(arr, begin, mid, end);
}

// Merges two sections of an array.
// First array is [begin..mid]
// Second array is [mid+1..end]
fn merge(arr: &mut Vec<i32>, left: i32, mid: i32, right: i32) {
    let arr1_l = mid - left + 1;
    let arr2_l = right - mid;
    
    // Create temp arrays(?)
    let mut arr1: Vec<i32> = vec![0; arr1_l as usize];
    let mut arr2: Vec<i32> = vec![0; arr2_l as usize];
    
    for i in 0..arr1_l {
        let second_index = (i + left) as usize;
        arr1[i as usize] = arr[second_index];
    }
    for i in 0..arr2_l {
        let second_index = (mid + 1 + i) as usize;
        arr2[i as usize] = arr[second_index];
    }
    
    let mut arr1_i: usize = 0;
    let mut arr2_i: usize = 0;
    let mut merged_i: usize = left as usize;
    
    // Merge both arrays back into the main array.
    loop {
        if (arr1_i as i32) >= arr1_l { break } 
        if (arr2_i as i32) >= arr2_l { break }
        
        if arr1[arr1_i] <= arr2[arr2_i] {
            arr[merged_i] = arr1[arr1_i];
            arr1_i += 1;
        } else {
            arr[merged_i] = arr2[arr2_i];
            arr2_i += 1;
        }
        merged_i += 1;
    }
    
    // Ensure that arr1 is truly empty.
    while arr1_i < arr1_l as usize {
        arr[merged_i] = arr1[arr1_i];
        arr1_i += 1;
        merged_i += 1;
    }
    
    // Ensure that arr2 is truly empty.
    while arr2_i < arr2_l as usize {
        arr[merged_i] = arr2[arr2_i];
        arr2_i += 1;
        merged_i += 1;
    }
}