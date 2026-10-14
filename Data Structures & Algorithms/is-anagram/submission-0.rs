impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let s1 : Vec<char> = s.chars().collect();
        let s2 : Vec<char> = t.chars().collect();

        let mut counter1 = vec![0;26];
        let mut counter2 = vec![0;26];

        for ch in s1{
            counter1[ch as usize - 'a' as usize] += 1;
        }

        for ch in s2{
            counter2[ch as usize - 'a' as usize] += 1;
        }

        for i in 0..26{
            if counter1[i] != counter2[i]{
                return false;
            }
        }

        true
    }
}
