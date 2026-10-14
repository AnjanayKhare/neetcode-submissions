impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut pre = vec![1;n];
        let mut post = vec![1;n];

        for i in 1..n{
            pre[i] = pre[i-1] * nums[i-1];
        }

        for i in (0..n-1).rev(){
            post[i] = post[i+1] * nums[i+1];
        }

        let mut ans = vec![1; n];

        for i in 0..n{
            ans[i] = pre[i] * post[i];
        }

        ans
    }
}
