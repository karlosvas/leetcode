use crate::easy::Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::new();
        let mut carry = 0;

        let mut i = a.len();
        let mut j = b.len();

        while i > 0 || j > 0 {
            let bit_a = if i > 0 {
                i -= 1;
                (a.as_bytes()[i] - b'0') as u8
            } else {
                0
            };

            let bit_b = if j > 0 {
                j -= 1;
                (b.as_bytes()[j] - b'0') as u8
            } else {
                0
            };

            if bit_a == bit_b && bit_a == 1 {
                result.insert(0, (b'0' + carry) as char);
                if carry == 0 {
                    carry ^= 1
                }
            } else if bit_a == 1 || bit_b == 1 {
                result.insert(0, (b'0' + carry ^ 1) as char);
            } else {
                result.insert(0, (b'0' + carry) as char);
                if carry == 1 {
                    carry ^= 1
                }
            }
        }

        if carry > 0 {
            result.insert(0, '1');
        }

        result
    }
}

// 67. Add Binary
// Given two binary strings a and b, return their sum as a binary string.

// Example 1:

// Input: a = "11", b = "1"
// Output: "100"
// Example 2:

// Input: a = "1010", b = "1011"
// Output: "10101"

// Constraints:

// 1 <= a.length, b.length <= 104
// a and b consist only of '0' or '1' characters.
// Each string does not contain leading zeros except for the zero itself.
