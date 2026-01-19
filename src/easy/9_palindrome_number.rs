use crate::easy::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let new_x: Vec<char> = x.to_string().chars().collect();

        for index in 0..(new_x.len() / 2) {
            if new_x[index] != new_x[new_x.len() - 1 - index] {
                return false;
            }
        }

        true
    }
}

// 9. Palindrome Number
// Given an integer x, return true if x is a palindrome, and false otherwise.

// Example 1:

// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.
// Example 2:

// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
// Example 3:

// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

// Constraints:
// -231 <= x <= 231 - 1
