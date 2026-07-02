// 387. First Unique Character in a String
// Solved
// Easy
// Topics
// premium lock iconCompanies
//
// Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.

use std::collections::HashMap;

use crate::easy::Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1
        }

        for (i, c) in s.chars().enumerate() {
            if *map.get(&c).unwrap() == 1 {
                return i as i32;
            }
        }

        -1
    }
}

// Example 1:
// Input: s = "leetcode"
// Output: 0
//
// Explanation:
// The character 'l' at index 0 is the first character that does not occur at any other index.
//
// Example 2:
// Input: s = "loveleetcode"
// Output: 2
//
// Example 3:
// Input: s = "aabb"
// Output: -1
//
//
// Constraints:
//     1 <= s.length <= 105
//     s consists of only lowercase English letters.
