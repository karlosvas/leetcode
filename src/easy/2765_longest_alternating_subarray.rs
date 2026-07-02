// 2765. Longest Alternating Subarray
// Easy
// Topics
// premium lock iconCompanies
// Hint
//
// You are given a 0-indexed integer array nums. A subarray s of length m is called alternating if:
//     m is greater than 1.
//     s1 = s0 + 1.
//     The 0-indexed subarray s looks like [s0, s1, s0, s1,...,s(m-1) % 2]. In other words, s1 - s0 = 1, s2 - s1 = -1, s3 - s2 = 1, s4 - s3 = -1, and so on up to s[m - 1] - s[m - 2] = (-1)m.
// Return the maximum length of all alternating subarrays present in nums or -1 if no such subarray exists.
// A subarray is a contiguous non-empty sequence of elements within an array.

use crate::easy::Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let (mut l, mut r): (usize, usize) = (0, 1);
        let mut subtract_to_target: i32 = 1;
        let mut max_len: i32 = 1;
        let n: usize = nums.len();

        while l < n && r < n {
            let current = nums[l];
            let next = nums[r];

            if current == next - subtract_to_target {
                r += 1;
                subtract_to_target = (subtract_to_target + 1) % 2;
            } else if r - l > 1 {
                subtract_to_target = 1;
                max_len = max_len.max(r as i32 - l as i32);
                l = r - 1;
            } else {
                l = r;
                r += 1;
            }
        }

        max_len = max_len.max(r as i32 - l as i32);
        if max_len == 1 { -1 } else { max_len }
    }
}
// Example 1:
//
// Input: nums = [2,3,4,3,4]
//
// Output: 4
//
// Explanation:
//
// The alternating subarrays are [2, 3], [3,4], [3,4,3], and [3,4,3,4]. The longest of these is [3,4,3,4], which is of length 4.
//
// Example 2:
//
// Input: nums = [4,5,6]
//
// Output: 2
//
// Explanation:
//
// [4,5] and [5,6] are the only two alternating subarrays. They are both of length 2.
//
//
// Constraints:
//
//     2 <= nums.length <= 100
//     1 <= nums[i] <= 104
