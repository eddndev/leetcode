impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy1 = i32::MIN;
        let mut sell1 = 0;

        let mut buy2 = i32::MIN;
        let mut sell2 = 0;

        for price in prices {
            buy1 = buy1.max(-price);

            sell1 = sell1.max(buy1 + price);

            buy2 = buy2.max(sell1 - price);

            sell2 = sell2.max(buy2 + price);
        }

        sell2
    }
}
