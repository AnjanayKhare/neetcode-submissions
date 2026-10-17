use std::cmp::max;

impl Solution {
    pub fn trap(arr: Vec<i32>) -> i32 {

        let mut ans = 0;

        let n = arr.len();

        let mut left = vec![0;n];
        let mut right = vec![0;n];

        for i in 1..n{
            left[i] = left[i-1].max(arr[i-1]);
        }
        for i in (0..(n-1)).rev(){
            right[i] = right[i+1].max(arr[i+1]);
        }

        for i in 0..n{
            let water = min(left[i], right[i]) - arr[i];
            // println!("{}", water);
            ans += water.max(0);
        }

        ans
    }
}
