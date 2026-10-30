impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut c = std::collections::HashMap::new();
        let mut ans = 0;
        let mut l = 0;
        let mut h = 0;
        let chars : Vec<char> = s.chars().collect();

        while l< s.len() && h< s.len(){
            let ch = chars[h];
            *c.entry(ch).or_insert(0) += 1;
            while let Some(&count) = c.get(&ch){
                if count <= 1{
                    break;
                }
                *c.entry(chars[l]).or_insert(0) -= 1;
                l+=1;
            }
            ans = max(ans, h-l+1);
            h+=1;
        }

        ans as i32
    }
}
