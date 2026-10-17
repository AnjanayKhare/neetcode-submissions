impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        nums.sort();
        let mut temp_ans = std::collections::HashSet::new();
        for i in 0..(n-2){
            let mut l = i+1;
            let mut h = n-1;

            while l<h{
                let curr = nums[i] + nums[l] + nums[h];
                if curr == 0{
                    temp_ans.insert(vec![nums[i], nums[l], nums[h]]);
                    l+=1;
                }else if curr > 0{
                    h-=1;
                }else{
                    l+=1;
                }
            } 
        }

        let ans : Vec<Vec<i32>> = temp_ans.into_iter().collect();

        ans

        
    }
}
