/**
 * 412. Fizz Buzz
 * https://leetcode.com/problems/fizz-buzz
 * 
 * Given an integer n, return a string array answer (1-indexed) where:
 * 
 *  answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
 *  answer[i] == "Fizz" if i is divisible by 3.
 *  answer[i] == "Buzz" if i is divisible by 5.
 *  answer[i] == i (as a string) if none of the above conditions are true.
 * 
 * Example 1:
 * Input: n = 3
 * Output: ["1","2","Fizz"]
 * 
 * Example 2:
 * Input: n = 5
 * Output: ["1","2","Fizz","4","Buzz"]
 * Example 3:
 * 
 * Input: n = 15
 * Output: ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]
 * 
 * Constraints:
 *  1 <= n <= 104
 */
struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|i| match (i % 3, i % 5) {
                (0, 0) => String::from("FizzBuzz"),
                (0, _) => String::from("Fizz"),
                (_, 0) => String::from("Buzz"),
                (_, _) => i.to_string(),
            })
            .collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"]);
    assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    assert_eq!(
        Solution::fizz_buzz(15),
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ]
    );
}
