impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut nums2: Vec<i32> = Vec::new();
        for &n in nums.iter() {
            if n != val {
                nums2.push(n);
            }
        }
        nums.clear();
        nums.append(&mut nums2);
        nums.len() as i32
    }
}
