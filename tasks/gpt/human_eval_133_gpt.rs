/// You are given a list of numbers.
/// You need to return the sum of squared numbers in the given list,
/// round each element in the list to the upper int(Ceiling) first.
///
/// Examples:
/// For lst = [1,2,3] the output should be 14
/// For lst = [1,4,9] the output should be 98
/// For lst = [1,3,5,7] the output should be 84
/// For lst = [1.4,4.2,0] the output should be 29
/// For lst = [-2.4,1,1] the output should be 6

use vstd::prelude::*;

fn main() {}

verus!{
fn sum_squares(lst: Vec<i32>) -> (ret: i32) {
    let mut sum: i32 = 0;
    let mut i = 0;
    while i < lst.len() {
        let num: i32 = lst[i];
        sum += num * num;
        i += 1;
    }
    sum
}
}
