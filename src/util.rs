// A file that has a bunch of utility functions.
use rand::{Rng, thread_rng};

#[derive(Debug)]
pub enum SortError {
    ArrayExistsError
}

// Prints out all of the values of a Vec<i32> because I don't know how to use templates yet.
pub fn print_vec(a: &Vec<i32>) {
    for (i, v) in a.iter().enumerate() {
        if i == (a.len() - 1) {
            print!("{}", v);
        } else {
            print!("{}, ", v);
        }
    }
    // Empty line at the end for nice formatting.
    println!("\n");
}

// Creates random values for a 100 element long Vec<i32>.
// Will not push values to the vec if there are already values in it - returns ArrayExistsError
pub fn populate_arr(a: &mut Vec<i32>) -> Result<(), SortError> {
    if a.len() > 0 {
        return Err(SortError::ArrayExistsError)
    }
    
    for _ in 0..10000 {
        a.push(thread_rng().gen_range(1..=100000));
    }
    
    Ok(())
}

// Swaps two elements at indexes a and b in Vec<i32> arr.
pub fn swap_arr(arr: &mut Vec<i32>, a: usize, b: usize) {
    let temp = arr[a];
    arr[a] = arr[b];
    arr[b] = temp;
}