impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stk = Vec::new();
        let mut ans = 0;
        for (i, h) in heights.iter().enumerate(){
            let mut start = i;
            while let Some(&(j, h1)) = stk.last(){
                if h1 > h{
                    stk.pop();
                    ans = max(ans, h1 * ( (i-j) as i32 ));
                    println!("{} {}", h1, h);
                    start = j;
                }else{
                    break;
                }
            }


            stk.push((start, h));
        }
        while let Some((j, &h1)) = stk.pop(){
            ans = max(ans, h1 * ( (heights.len() - j) as i32 ));
        }

        ans
 
    }
}
