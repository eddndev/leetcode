use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut l_rem = 0;
        let mut r_rem = 0;

        for c in s.chars() {
            if c == '(' {
                l_rem += 1;
            } else if c == ')' {
                if l_rem > 0 {
                    l_rem -= 1;
                } else {
                    r_rem += 1;
                }
            }
        }

        let s_chars: Vec<char> = s.chars().collect();
        let mut result = HashSet::new();
        let mut current_expr = String::new();

        Self::dfs(0, 0, l_rem, r_rem, &s_chars, &mut current_expr, &mut result);

        result.into_iter().collect()
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        index: usize,
        balance: i32,
        l_rem: i32,
        r_rem: i32,
        s: &Vec<char>,
        expr: &mut String,
        res: &mut HashSet<String>,
    ) {
        if index == s.len() {
            if l_rem == 0 && r_rem == 0 && balance == 0 {
                res.insert(expr.clone());
            }
            return;
        }

        let char_at = s[index];

        let is_duplicate = index > 0
            && s[index] == s[index - 1]
            && expr.len() != (index - (l_rem + r_rem) as usize);

        if char_at == '(' && l_rem > 0 {
            Self::dfs(index + 1, balance, l_rem - 1, r_rem, s, expr, res);
        } else if char_at == ')' && r_rem > 0 {
            Self::dfs(index + 1, balance, l_rem, r_rem - 1, s, expr, res);
        }

        expr.push(char_at);
        if char_at == '(' {
            Self::dfs(index + 1, balance + 1, l_rem, r_rem, s, expr, res);
        } else if char_at == ')' {
            if balance > 0 {
                Self::dfs(index + 1, balance - 1, l_rem, r_rem, s, expr, res);
            }
        } else {
            Self::dfs(index + 1, balance, l_rem, r_rem, s, expr, res);
        }

        expr.pop();
    }
}
