impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let thousands = ["", " Thousand", " Million", " Billion"];

        let mut res = String::new();
        let mut n = num;
        let mut i = 0;

        while n > 0 {
            if n % 1000 != 0 {
                res = format!("{}{}{}", Self::helper(n % 1000), thousands[i], res);
            }
            n /= 1000;
            i += 1;
        }

        res.trim().to_string()
    }

    fn helper(num: i32) -> String {
        let less_than_20 = [
            "",
            " One",
            " Two",
            " Three",
            " Four",
            " Five",
            " Six",
            " Seven",
            " Eight",
            " Nine",
            " Ten",
            " Eleven",
            " Twelve",
            " Thirteen",
            " Fourteen",
            " Fifteen",
            " Sixteen",
            " Seventeen",
            " Eighteen",
            " Nineteen",
        ];
        let tens = [
            "", " Ten", " Twenty", " Thirty", " Forty", " Fifty", " Sixty", " Seventy", " Eighty",
            " Ninety",
        ];

        if num == 0 {
            "".to_string()
        } else if num < 20 {
            less_than_20[num as usize].to_string()
        } else if num < 100 {
            format!("{}{}", tens[(num / 10) as usize], Self::helper(num % 10))
        } else {
            format!(
                "{} Hundred{}",
                less_than_20[(num / 100) as usize],
                Self::helper(num % 100)
            )
        }
    }
}
