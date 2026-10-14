impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map = HashMap::new();

        let n = nums.len();

        for i in 0..n{
            let n1 = nums[i];
            let n2 = target - n1;
            if let Some(&j) = map.get(&n2){
                return vec![j as i32, i as i32];
            }
            map.insert(n1, i);
        }

        vec![-1, -1]
    }
}
