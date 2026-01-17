use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (index, &value) in nums.iter().enumerate() {
            let sol: i32 = target - value;
            if let Some(&index_target) = map.get(&sol) {
                return vec![index as i32, index_target as i32];
            }
            map.insert(value, index);
        }

        Vec::new()
    }
}
