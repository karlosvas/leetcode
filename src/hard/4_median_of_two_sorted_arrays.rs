use crate::Solution;

// Problem: Median of Two Sorted Arrays
// Difficulty: Hard

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        nums1.append(&mut nums2);
        nums1.sort();

        let len = nums1.len();
        let mid = len / 2;

        if len % 2 == 1 {
            nums1[mid] as f64
        } else if len > 3 {
            (nums1[mid] + nums1[mid - 1]) as f64 / 2.0
        } else {
            (nums1[0] + nums1[1]) as f64 / 2.0
        }
    }
}

// Given two sorted arrays `nums1` and `nums2` of size `m` and `n` respectively, return **the median** of the two sorted arrays.
//
// The overall run time complexity should be `O(log (m+n))`.
//
// Example 1:
// Input: nums1 = [1,3], nums2 = [2]
// Output: 2.00000
// Explanation: merged array = [1,2,3] and median is 2.
//
// Example 2:
// Input: nums1 = [1,2], nums2 = [3,4]
// Output: 2.50000
// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
//
// Constraints:
// - nums1.length == m
// - nums2.length == n
// - 0 <= m <= 1000
// - 0 <= n <= 1000
// - 1 <= m + n <= 2000
// - -10^6 <= nums1[i], nums2[i] <= 10^6
