/**
 * 1108. Defanging an IP Address
 * https://leetcode.com/problems/defanging-an-ip-address
 * 
 * Given a valid (IPv4) IP address, return a defanged version of that IP address.
 * A defanged IP address replaces every period "." with "[.]".
 */
struct Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace('.', "[.]")
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::defang_i_paddr(String::from("1.1.1.1")),
        "1[.]1[.]1[.]1"
    );

    assert_eq!(
        Solution::defang_i_paddr(String::from("255.100.50.0")),
        "255[.]100[.]50[.]0"
    );
}
