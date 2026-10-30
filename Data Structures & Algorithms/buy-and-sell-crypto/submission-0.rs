impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        let mut ans = 0;
        let mut mn = prices[0];
        for p in prices{
            ans = max(ans, p - mn);
            mn = min(mn, p);
        }

        ans
    }
}
