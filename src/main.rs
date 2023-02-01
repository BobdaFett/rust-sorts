//
// Created 1/31/23 by Lucas Vas
// This is a collection of sorting algorithms that I've decided to implement in Rust.
//
use crate::util::*;
use crate::merge_sort::merge_sort;
use crate::bubble_sort::bubble_sort;
use crate::selection_sort::sel_sort;
use crate::quick_sort::quick_sort;

pub mod merge_sort;
pub mod bubble_sort;
pub mod util;
pub mod selection_sort;
pub mod quick_sort;

fn main() {
    // let mut arr1: Vec<i32> = Vec::new();
    // populate_arr(&mut arr1).unwrap();
    // // print_vec(&arr1);
    // println!("Using merge sort...");
    // let arr1_l = arr1.len();
    // merge_sort(&mut arr1, 0, arr1_l - 1);
    // print_vec(&arr1);
    
    // let mut arr2: Vec<i32> = Vec::new();
    // populate_arr(&mut arr2).unwrap();
    // print_vec(&arr2);
    // println!("Using bubble sort...");
    // bubble_sort(&mut arr2);
    // print_vec(&arr2);
    
    // let mut arr3: Vec<i32> = Vec::new();
    // populate_arr(&mut arr3).unwrap();
    // // print_vec(&arr3);
    // println!("Using selection sort...");
    // sel_sort(&mut arr3);
    // print_vec(&arr3);
    
    // let mut arr4: Vec<i32> = Vec::new();
    // populate_arr(&mut arr4).unwrap();
    // println!("Using quick sort...");
    // let arr4_l = arr4.len();
    // quick_sort(&mut arr4, 0, arr4_l - 1);
    // print_vec(&arr4);
}
