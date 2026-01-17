use std::collections::BTreeSet;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let set_nums: BTreeSet<i32> = nums.drain(..).collect();
        nums.extend(set_nums.iter());
        nums.len() as i32
    }
}
