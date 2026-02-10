impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n_usize = n as usize;
        let mut fact = vec![1; n_usize];
        for i in 1..n_usize {
            fact[i] = fact[i - 1] * i as i32;
        }

        let mut numbers: Vec<char> = (1..=n as u8).map(|digit| (b'0' + digit) as char).collect();

        let mut k = k - 1;
        let mut result = String::with_capacity(n_usize);

        for i in (0..n_usize).rev() {
            let factorial = fact[i];

            let index = (k / factorial) as usize;

            result.push(numbers.remove(index));

            k %= factorial;
        }

        result
    }
}
