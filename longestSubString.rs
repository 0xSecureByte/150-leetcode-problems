use std::collections::HashMap;
use std::convert::TryInto;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_index: HashMap<char, usize> = HashMap::new();
        let mut start = 0;
        let mut max_length = 0;
        for (end, char) in s.chars().enumerate() {
            if let Some(&prev_index) = char_index.get(&char) {
                if prev_index >= start {
                    start = prev_index + 1;
                }
            }
            char_index.insert(char, end);
            max_length = max_length.max(end - start + 1);
        }
        max_length.try_into().unwrap_or(i32::MAX)
    }
}
