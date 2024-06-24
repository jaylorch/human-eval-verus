/// We have an array 'arr' of N integers arr[1], arr[2], ..., arr[N]. The
/// numbers in the array will be randomly ordered. Your task is to determine if
/// it is possible to get an array sorted in non-decreasing order by performing 
/// the following operation on the given array:
///     You are allowed to perform right shift operation any number of times.
///
/// One right shift operation means shifting all elements of the array by one
/// position in the right direction. The last element of the array will be moved to
/// the starting position in the array i.e. 0th index. 
///
/// If it is possible to obtain the sorted array by performing the above operation
/// then return true else return false.
/// If the given array is empty then return true.
///
/// Note: The given list is guaranteed to have unique elements.
///
/// # Examples
///
/// ```
/// assert_eq!(move_one_ball(vec![3, 4, 5, 1, 2]), true);
/// assert_eq!(move_one_ball(vec![3, 5, 4, 1, 2]), false);
/// ```

use vstd::prelude::*;
fn main() {}

verus!{
fn move_one_ball(arr: Vec<i32>) -> (result: bool) {
    
    if arr.len() == 0 {
        return true;
    }

    let mut count = 0;
    let n = arr.len();
    let mut i = 0;

    while i < n {
        if arr[i] > arr[(i + 1) % n] {
            count += 1;
        }
        if count > 1 {
            return false;
        }
        i += 1;
    }

    true
}
}
