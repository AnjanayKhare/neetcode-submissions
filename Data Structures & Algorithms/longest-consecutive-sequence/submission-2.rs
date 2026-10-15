impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 0{
            return 0;
        }
        nums.sort();
        let mut ans = 1;
        let mut chain = 1;
        // println!("{:?}", nums);
        for i in 1..nums.len(){
            if nums[i] == nums[i-1]{
                continue;
            }
            if nums[i] == nums[i-1] + 1{
                chain += 1;
            }else{
                chain = 1;
            }

            // println!("{} {}", nums[i], chain);

            ans = ans.max(chain);
        }

        ans
    }
}
