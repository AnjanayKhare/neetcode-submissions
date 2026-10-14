impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let n = nums.len();
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        for n in nums{
            if seen.contains(&n){
                return true;
            }
            seen.insert(n);
        }

        false
    }
}
