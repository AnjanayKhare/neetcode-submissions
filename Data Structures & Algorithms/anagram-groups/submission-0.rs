impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();

        for s in strs{
            let mut freq = vec![0;26];
            for ch in s.clone().chars(){
                let i = ch as usize - 'a' as usize;
                freq[i] += 1;
            }

            map.entry(freq).or_insert(Vec::new()).push(s);
        }

        let mut ans = Vec::new();
        for (_, val) in map{
            ans.push(val);
        }
        ans
    }
}
