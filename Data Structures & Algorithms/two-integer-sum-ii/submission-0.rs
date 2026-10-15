impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {

        let mut l = 0;
        let mut h = numbers.len() - 1;

        while l<h{
            let curr = numbers[l] + numbers[h];
            if curr == target{
                return vec![l as i32 +1, h as i32 +1];
            }

            if curr > target{
                h-=1;
            }else{
                l+=1;
            }
        }

        vec![-1, -1]
    }
}
