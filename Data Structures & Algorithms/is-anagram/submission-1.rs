use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut set = HashMap::new();
        for c in s.chars() {
            *set.entry(c).or_insert(0) += 1;
        }
        for ch in t.chars() {
            let d = set.entry(ch).or_insert(0);
            *d -= 1;
            if *d < 0 {
                return false;
            }
        }
        return true;
    }
}
