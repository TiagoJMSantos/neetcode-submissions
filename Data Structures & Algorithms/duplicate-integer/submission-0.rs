use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for number in nums {
            let happened = set.insert(number);
            if !happened {return true;}
        }
        return false;
    }
}
