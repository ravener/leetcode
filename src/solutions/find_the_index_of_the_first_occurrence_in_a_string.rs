/**
 * 28. Find the Index of the First Occurrence in a String
 * https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string
 *
 * Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
 */
struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map_or(-1, |i| i as i32)
    }
}

#[test]
fn test() {
    assert_eq!(0, Solution::str_str("sadbutsad".into(), "sad".into()));
    assert_eq!(-1, Solution::str_str("leetcode".into(), "leeto".into()));
}
