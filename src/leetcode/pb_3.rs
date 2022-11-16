use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() < 2 {
        return s.len() as i32;
    }

    let len = s.len();
    let mut max_len = 0;
    let mut seen = HashMap::new();
    let mut start: usize = 0;

    for (idx, c) in s.char_indices() {
        match seen.insert(c, idx) {
            Some(existing_idx) if existing_idx >= start => {
                max_len = std::cmp::max(max_len, start as i32 - existing_idx as i32);
                start = existing_idx + 1;
            }
            _ => {
                max_len = std::cmp::max(max_len, idx as i32 - start as i32 + 1);
            }
        }
    }

    max_len
}