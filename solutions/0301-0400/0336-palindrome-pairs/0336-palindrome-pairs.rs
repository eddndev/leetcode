use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        // Construimos un mapa de palabra -> índice para búsquedas rápidas
        for (i, word) in words.iter().enumerate() {
            map.insert(word.as_str(), i);
        }

        let mut res = Vec::new();

        for (i, word) in words.iter().enumerate() {
            let n = word.len();
            let chars = word.as_bytes();

            for j in 0..=n {
                let s1 = &chars[0..j];
                let s2 = &chars[j..n];

                if is_palindrome(s1) {
                    let mut rev_s2 = s2.to_vec();
                    rev_s2.reverse();
                    if let Ok(target) = std::str::from_utf8(&rev_s2) {
                        if let Some(&k) = map.get(target) {
                            if k != i {
                                res.push(vec![k as i32, i as i32]);
                            }
                        }
                    }
                }

                if s2.len() > 0 && is_palindrome(s2) {
                    let mut rev_s1 = s1.to_vec();
                    rev_s1.reverse();
                    if let Ok(target) = std::str::from_utf8(&rev_s1) {
                        if let Some(&k) = map.get(target) {
                            if k != i {
                                res.push(vec![i as i32, k as i32]);
                            }
                        }
                    }
                }
            }
        }

        res
    }
}

fn is_palindrome(chars: &[u8]) -> bool {
    let mut left = 0;
    let mut right = chars.len();
    if right == 0 {
        return true;
    }
    right -= 1;

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
