/**
 * 2235. Add Two Integers
 * https://leetcode.com/problems/add-two-integers
 * 
 * Given two integers num1 and num2, return the sum of the two integers.
 */
struct Solution;

impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum(12, 5), 17);
    assert_eq!(Solution::sum(-10, 4), -6);
}
