impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut h = nums.len() - 1;

        while l <= h{
            let mid = (h+l) / 2;

            if nums[mid] == target{
                return mid as i32;
            } else if nums[mid ] > target {
                if mid == 0{
                    return -1;
                }
                h = mid -1;
            }else{
                l = mid + 1;
            }
        }

        -1
    }
}
