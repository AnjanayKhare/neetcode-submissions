impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {

        let mut stk = Vec::new();
        let n = temperatures.len();
        let mut ans = vec![0;n];

        for i in 0..n{
            let curr = temperatures[i];
            while let Some(&(j, prev)) = stk.last(){
                if curr > prev{
                    ans[j] = (i-j) as i32; 
                    stk.pop();
                }else{
                    break;
                }
            }
            stk.push((i, curr));
        }

        ans
    }
}
