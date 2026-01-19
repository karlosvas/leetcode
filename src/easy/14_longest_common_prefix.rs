use crate::easy::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut min_len: usize = usize::MAX;
        let mut min_str: String = String::new();
        for i in &strs {
            if i.len() < min_len {
                min_len = i.len();
                min_str = i.to_string();
            }
        }

        let mut index = 0;
        for &char_min_str in min_str.as_bytes() {
            for s in &strs {
                if s.as_bytes()[index] != char_min_str {
                    return strs[0][0..index].to_string();
                }
            }
            index += 1;
        }

        strs[0][0..index].to_string()
    }
}

// 14. Longest Common Prefix
// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

// Example 1:

// Input: strs = ["flower","flow","flight"]
// Output: "fl"
// Example 2:

// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

// Constraints:

// 1 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] consists of only lowercase English letters if it is non-empty.
