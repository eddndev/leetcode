use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let word_set: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
        let mut memo: HashMap<usize, Vec<String>> = HashMap::new();

        Self::dfs(0, &s, &word_set, &mut memo)
    }

    fn dfs(
        start: usize,
        s: &str,
        word_set: &HashSet<&str>,
        memo: &mut HashMap<usize, Vec<String>>,
    ) -> Vec<String> {
        if let Some(res) = memo.get(&start) {
            return res.clone();
        }

        if start == s.len() {
            return vec![String::new()];
        }

        let mut results = Vec::new();

        for end in start + 1..=s.len() {
            let word = &s[start..end];

            if word_set.contains(word) {
                let sub_sentences = Self::dfs(end, s, word_set, memo);

                for sub in sub_sentences {
                    let mut sentence = String::from(word);
                    if !sub.is_empty() {
                        sentence.push(' ');
                        sentence.push_str(&sub);
                    }
                    results.push(sentence);
                }
            }
        }

        memo.insert(start, results.clone());
        results
    }
}
