impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut input = x;
        let mut reversed = 0;

        while input > reversed {
            reversed = reversed * 10 + input % 10;
            input /= 10;
        }

        input == reversed || input == reversed / 10
    }
}
