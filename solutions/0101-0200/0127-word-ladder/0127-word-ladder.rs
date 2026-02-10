use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // Convert the vector to a hashset for search
        let mut word_set: HashSet<String> = word_list.into_iter().collect();

        // If the final word doesn't exist, return 0
        if !word_set.contains(&end_word) {
            return 0;
        }

        // Init the queue for BFS
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        queue.push_back((begin_word, 1));

        while let Some((current_word, level)) = queue.pop_front() {
            // If end_word reached, return's the current level
            if current_word == end_word {
                return level;
            }

            let mut current_bytes = current_word.into_bytes();
            let len = current_bytes.len();

            for i in 0..len {
                let original_char = current_bytes[i];

                for c in b'a'..=b'z' {
                    if c == original_char {
                        continue;
                    }

                    current_bytes[i] = c;

                    if let Ok(next_word_str) = std::str::from_utf8(&current_bytes) {
                        if word_set.contains(next_word_str) {
                            word_set.remove(next_word_str);

                            queue.push_back((next_word_str.to_string(), level + 1));
                        }
                    }
                }

                current_bytes[i] = original_char;
            }
        }

        0
    }
}
