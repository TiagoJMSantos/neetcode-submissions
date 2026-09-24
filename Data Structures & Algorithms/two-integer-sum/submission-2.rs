use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mapa = HashMap::new();
        for (j, &num) in nums.iter().enumerate() {
            let complemento = target - num;
            if let Some(&i) = mapa.get(&complemento) {
                return vec![i as i32, j as i32];
            }
            mapa.insert(num, j);
        }
        vec![]
    }
}