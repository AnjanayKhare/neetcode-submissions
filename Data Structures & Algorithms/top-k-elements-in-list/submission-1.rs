impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::{BinaryHeap, HashMap};
        use std::cmp::Reverse as R;
        let mut c = HashMap::new();
        let mut heap = BinaryHeap::new();

        for n in nums{
            *c.entry(n).or_insert(0) += 1;
        }

        for (num, count) in c{
            heap.push(R((count, num)));

            if heap.len() > k as usize{
                heap.pop();
            }
        }

        let mut ans = Vec::new();
        for _ in 0..(k as usize){
            let R((count, num)) = heap.pop().unwrap();
            ans.push(num);
        }

        ans
    }
}
