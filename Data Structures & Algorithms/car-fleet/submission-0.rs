impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
         let n = speed.len();

        let mut temp = Vec::new();
        for i in 0..n{
            temp.push((position[i], speed[i]));
        }

        temp.sort();
        let mut stk = Vec::new();

        for i in (0..n).rev(){
            let pos = temp[i].0;
            let spd = temp[i].1;
            if let Some(&(p, s)) = stk.last(){
                let t1 = (target-p) as f64 / s as f64;
                let t2 = (target-pos) as f64 / spd as f64;
                if t1 < t2 {
                    stk.push((pos, spd));
                }
            }else{
                stk.push((pos, spd));
            }
        }
        stk.len() as i32
    }
}
