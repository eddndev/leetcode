impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 || k == 0 {
            return 0;
        }
        let k = k as usize;

        if k >= n / 2 {
            return prices.windows(2).map(|w| (w[1] - w[0]).max(0)).sum();
        }

        let mut buy = vec![-1_000_000_000; k + 1];
        let mut sell = vec![0; k + 1];

        for price in prices {
            for j in 1..=k {
                buy[j] = buy[j].max(sell[j - 1] - price);

                sell[j] = sell[j].max(buy[j] + price);
            }
        }

        sell[k]
    }
}
