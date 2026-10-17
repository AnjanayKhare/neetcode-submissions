impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars : Vec<char> = s.chars().collect();

        let mut stk = Vec::new();


        for i in 0..chars.len(){
            if let Some(&top) = stk.last(){
                if  (top == '{' && chars[i] == '}') || 
                    (top == '(' && chars[i] == ')') || 
                    (top == '[' && chars[i] == ']'){
                    stk.pop();
                }
                else{
                    stk.push(chars[i]);
                }
            }else{
                stk.push(chars[i]);
            }
        }
        stk.len() == 0
    }
}
