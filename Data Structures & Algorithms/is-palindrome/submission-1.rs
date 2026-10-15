impl Solution {
    pub fn is_palindrome(s: String) -> bool {

        let chars : Vec<char> = s.chars().collect();
        let mut i =0;
        let mut j = chars.len() - 1;

        while i<j{

            if !chars[i].is_alphanumeric(){
                i+=1;
                continue;
            }
            if !chars[j].is_alphanumeric(){
                j-=1;
                continue;
            }            
            if chars[i].to_ascii_lowercase() != chars[j].to_ascii_lowercase(){
                return false;
            }

            i+= 1;
            j-= 1;
        }
        true
    }
}
