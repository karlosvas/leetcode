// 693. Binary Number with Alternating Bits
// Easy
// Topics
// premium lock iconCompanies
//
// Given a positive integer, check whether it has alternating bits: namely, if two adjacent bits will always have different values.

use crate::easy::Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let x = n ^ (n >> 1);
        x & (x + 1) == 0
    }
}

// Example 1:
//
// Input: n = 5
// Output: true
// Explanation: The binary representation of 5 is: 101
//
// Example 2:
//
// Input: n = 7
// Output: false
// Explanation: The binary representation of 7 is: 111.
//
// Example 3:
//
// Input: n = 11
// Output: false
// Explanation: The binary representation of 11 is: 1011.
//
// Constraints:
//
//     1 <= n <= 231 - 1
