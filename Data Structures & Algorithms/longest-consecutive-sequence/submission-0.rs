impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {

        let mut map = std::collections::HashMap::new();
        nums.sort();
        for n in nums{

            let last_length = *map.entry(n-1).or_insert(0);
            map.insert(n, last_length + 1);

        }

        let mut ans = 0;
        for (_, val) in map{
            ans = ans.max(val);
        }

        ans
    }
}
