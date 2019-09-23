//! Binary Search
//! =============
//!
//! This is an implementation of binary search
//! I may group these algorithms in a crate

use std::cmp::Ordering;

/// Returns the index position of the value being searched for in the
/// given array,
///
/// Returns Some(index position)
/// or None
fn binary_search<T: Ord>(arr: &[T], value: T, low: usize, high: usize) -> Option<usize> {
    if high < low {
        return None;
    }

    let mid = (low + high) / 2;

    match value.cmp(&arr[mid]) {
        Ordering::Equal => Some(mid),
        Ordering::Less => binary_search(arr, value, low, mid - 1),
        Ordering::Greater => binary_search(arr, value, mid + 1, high),
    }
}

/// Example of how to use the function
fn main() {
    // Array to be searched
    let arr: [i32; 5] = [2, 3, 4, 10, 40];
    // Highest postion in the array to search
    let n = arr.len();
    // x = value to search for
    let x = 10;

    let result = binary_search(&arr, x, 0, n - 1);

    if let Some(v) = result {
        println!("Mid: {} @ pos {}", arr[v], v);
    }
}
