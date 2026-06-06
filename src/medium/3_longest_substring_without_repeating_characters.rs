use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let letters: Vec<char> = s.chars().collect();
        let mut char_appearances: HashMap<char, usize> = HashMap::new();
        let mut l = 0;
        let mut max_sol = 0;

        for (i, c) in letters.iter().enumerate() {
            if char_appearances.contains_key(&c) {
                let last_same_char = char_appearances.get(&c).unwrap();
                if last_same_char+1 > l {
                    l = last_same_char+1;
                }
            } 
            char_appearances.insert(c.clone(), i);
            if (i+1) - l > max_sol { max_sol = (i+1) - l; }
        }

        max_sol as i32
    }
}
