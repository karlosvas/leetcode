// 219. Contains Duplicate II
// Easy

use std::collections::HashMap;

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<&i32, usize> = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            if map.contains_key(n) {
                let diff: i32 = (i as i32 - *map.get(n).unwrap() as i32).abs();
                if diff <= k {
                    return true;
                } else if diff > k {
                    map.insert(n, i);
                }
            } else {
                map.insert(n, i);
            }
        }

        false
    }
}

// Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array
// such that nums[i] == nums[j] and abs(i - j) <= k.
//
// Example 1:
// Input: nums = [1,2,3,1], k = 3
// Output: true
//
// Example 2:
// Input: nums = [1,0,1,1], k = 1
// Output: true
//
// Example 3:
// Input: nums = [1,2,3,1,2,3], k = 2
// Output: false
//
// Constraints:
//     1 <= nums.length <= 105
//     -109 <= nums[i] <= 109
//     0 <= k <= 105

#[cfg(test)]
mod tests {
    use super::*;

    crate::check_case!(
        c1,
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
        true
    );
    crate::check_case!(
        c2,
        Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
        true
    );
    crate::check_case!(
        c3,
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
        false
    );
}
