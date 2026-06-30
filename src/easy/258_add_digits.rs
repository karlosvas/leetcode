// 258. Add Digits
// Easy
// Topics
// premium lock iconCompanies
// Hint
//
// Given an integer num, repeatedly add all its digits until the result has only one digit, and return it.

use crate::easy::Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut sum: String = num.to_string();

        while sum.len() > 1 {
            let mut aux: u32 = 0;

            for i in sum.chars() {
                aux += i.to_digit(10).unwrap();
            }

            sum = aux.to_string();
        }

        sum.parse::<i32>().unwrap()
    }
}

// Example 1:
//
// Input: num = 38
// Output: 2
// Explanation: The process is
// 38 --> 3 + 8 --> 11
// 11 --> 1 + 1 --> 2
// Since 2 has only one digit, return it.
//
// Example 2:
//
// Input: num = 0
// Output: 0
//
// Constraints:
//     0 <= num <= 231 - 1
