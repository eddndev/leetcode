impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut rev = 0i32;

        while num != 0 {
            let digit = num % 10;
            num /= 10;

            match rev.checked_mul(10).and_then(|v| v.checked_add(digit)) {
                Some(val) => rev = val,
                None => return 0,
            }
        }

        rev
    }
}
