impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }

        let n = n as i64;
        let mut count = 0;
        let mut i = 1;

        while i <= n {
            let prefix = n / (i * 10);
            let digit = (n / i) % 10;
            let suffix = n % i;

            if digit == 0 {
                count += prefix * i;
            } else if digit == 1 {
                count += prefix * i + (suffix + 1);
            } else {
                count += (prefix + 1) * i;
            }

            if i > n / 10 {
                break;
            }
            i *= 10;
        }

        count as i32
    }
}
