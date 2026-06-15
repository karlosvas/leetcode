use std::collections::HashMap;

// Problem: Isomorphic Strings
// Difficulty: Easy

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let t_char: Vec<char> = t.chars().collect();
        let mut map: HashMap<char, char> = HashMap::new();
        let mut duplicated: HashMap<char, bool> = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            let t: char = t_char[i];

            if (!map.contains_key(&c) && duplicated.get(&t).copied().unwrap_or_default())
                || (map.contains_key(&c) && *map.get(&c).unwrap() != t)
            {
                return false;
            }

            map.insert(c, t);
            duplicated.insert(t, true);
        }

        true
    }
}

// Given two strings `s` and `t`, _determine if they are isomorphic_.
//
// Two strings `s` and `t` are isomorphic if the characters in `s` can be replaced to get `t`.
//
// All occurrences of a character must be replaced with another character while preserving the order of characters.
// No two characters may map to the same character, but a character may map to itself.
//
// Example 1:
// Input: s = "egg", t = "add"
// Output: true
// Explanation:
// The strings `s` and `t` can be made identical by:
// - Mapping `'e'` to `'a'`.
// - Mapping `'g'` to `'d'`.
//
// Example 2:
// Input: s = "foo", t = "bar"
// Output: false
// Explanation:
// The strings `s` and `t` can not be made identical as `'o'` needs to be mapped to both `'a'` and `'r'`.
//
// Example 3:
// Input: s = "paper", t = "title"
// Output: true
//
// Constraints:
// - `1 <= s.length <= 5 * 10^4`
// - `t.length == s.length`
// - `s` and `t` consist of any valid ascii character.

#[cfg(test)]
mod tests {
    use super::*;

    crate::check_case!(
        c1,
        Solution::is_isomorphic("egg".to_string(), "add".to_string()),
        true
    );
    crate::check_case!(
        c2,
        Solution::is_isomorphic("foo".to_string(), "bar".to_string()),
        false
    );
    crate::check_case!(
        c3,
        Solution::is_isomorphic("paper".to_string(), "title".to_string()),
        true
    );
}
