use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let (mut sol, mut i): (i32, usize) = (0, 0);
        let chars: Vec<char> = s.chars().collect();

        while i < chars.len() {
            let val: i32 = map[&chars[i]];

            if i + 1 < chars.len() {
                if let Some(val_next) = map.get(&chars[i + 1]).filter(|&&v| v > val) {
                    sol += (val_next - val);
                    i += 2;
                } else {
                    sol += val;
                    i += 1;
                }
            } else {
                sol += val;
                i += 1;
            }
        }

        sol
    }
}
