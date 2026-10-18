impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stk : Vec<String>  = Vec::new();

        for t in tokens{
            if t == "+" ||t == "*" ||t == "/" || t == "-"{
                let n1 : i32 = stk.pop().unwrap().parse().unwrap();
                let n2 : i32 = stk.pop().unwrap().parse().unwrap();
                // println!("{}", t);
                let mut eval = 0;
                if t == "+"{
                    eval = n1+n2;
                } else if t == "-"{
                    eval = n2-n1;
                }else if t == "*"{
                    eval = n1 * n2;
                }else{
                    eval = n2/n1;
                }
                stk.push(eval.to_string());
            }else{
                stk.push(t);
            }
        }
        let ans:i32 = stk.pop().unwrap().parse().unwrap();

        ans
    }
}
 