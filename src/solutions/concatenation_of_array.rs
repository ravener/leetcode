/**
 * 1929. Concatenation of Array
 * https://leetcode.com/problems/concatenation-of-array
 * 
 * Given an integer array nums of length n, you want to create an array ans of length 2n where ans[i] == nums[i] and ans[i + n] == nums[i] for 0 <= i < n (0-indexed).
 * Specifically, ans is the concatenation of two nums arrays.
 * Return the array ans.
 */
struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        [nums.clone(), nums.clone()].concat()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::get_concatenation(vec![1, 2, 1]),
        vec![1, 2, 1, 1, 2, 1]
    );

    assert_eq!(
        Solution::get_concatenation(vec![1, 3, 2, 1]),
        vec![1, 3, 2, 1, 1, 3, 2, 1]
    );
}
