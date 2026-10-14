impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::{BinaryHeap, HashMap};
        let mut c = HashMap::new();
        let mut heap = BinaryHeap::new();

        for n in nums{
            *c.entry(n).or_insert(0) += 1;
        }

        for (num, count) in c{
            heap.push((count, num));
        }

        let mut ans = Vec::new();
        for _ in 0..(k as usize){
            let (count, num) = heap.pop().unwrap();
            ans.push(num);
        }

        ans
    }
}
