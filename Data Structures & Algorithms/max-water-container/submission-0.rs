impl Solution {
    pub fn max_area(nums: Vec<i32>) -> i32 {

        let n = nums.len();

        let mut l = 0;
        let mut h = n-1;
        let mut ans = 0;
        while (l<h){
            ans = ans.max(
                nums[l].min(nums[h]) * ((h-l) as i32)
            );

            if nums[l] < nums[h]{
                l+=1;
            }else{
                h-=1;
            }
        }

        ans


    }
}
