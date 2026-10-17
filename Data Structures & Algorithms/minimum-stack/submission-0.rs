struct MinStack {
    stk : Vec<(i32, i32)>
}

impl MinStack {
    pub fn new() -> Self {
        Self{
            stk: Vec::new()
        }
    }

    pub fn push(&mut self, val: i32) {
        if let Some(&(_, min_val)) = self.stk.last(){
            self.stk.push((val, min(min_val, val)));
        }else{
            self.stk.push((val, val));
        }
    }

    pub fn pop(&mut self) {
        self.stk.pop();
    }

    pub fn top(&self) -> i32 {
        if let Some(&(ans, _)) = self.stk.last(){
            return ans;
        }

        -1
    }

    pub fn get_min(&self) -> i32 {
         if let Some(&(_, ans)) = self.stk.last(){
            return ans;
        }
        -1
    }
}
