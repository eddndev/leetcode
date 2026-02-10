use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut word_set: HashSet<String> = word_list.into_iter().collect();

        if !word_set.contains(&end_word) {
            return vec![];
        }

        let mut layer: HashSet<String> = HashSet::new();
        layer.insert(begin_word.clone());

        let mut parents: HashMap<String, Vec<String>> = HashMap::new();

        let mut found = false;

        while !layer.is_empty() && !found {
            for w in &layer {
                word_set.remove(w);
            }

            let mut next_layer: HashSet<String> = HashSet::new();

            for word in &layer {
                let mut chars: Vec<char> = word.chars().collect();

                for i in 0..chars.len() {
                    let old_char = chars[i];

                    for c in 'a'..='z' {
                        if c == old_char {
                            continue;
                        }

                        chars[i] = c;
                        let new_word: String = chars.iter().collect();

                        if word_set.contains(&new_word) {
                            if new_word == end_word {
                                found = true;
                            }

                            next_layer.insert(new_word.clone());

                            parents
                                .entry(new_word)
                                .or_insert(Vec::new())
                                .push(word.clone());
                        }
                    }
                    chars[i] = old_char;
                }
            }
            layer = next_layer;
        }

        let mut result = Vec::new();
        if found {
            let mut current_path = vec![end_word.clone()];
            Self::backtrack(
                &end_word,
                &begin_word,
                &parents,
                &mut current_path,
                &mut result,
            );
        }

        result
    }

    fn backtrack(
        current: &String,
        target: &String,
        parents: &HashMap<String, Vec<String>>,
        path: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        if current == target {
            let mut full_path = path.clone();
            full_path.reverse();
            result.push(full_path);
            return;
        }

        if let Some(parent_list) = parents.get(current) {
            for parent in parent_list {
                path.push(parent.clone());
                Self::backtrack(parent, target, parents, path, result);
                path.pop();
            }
        }
    }
}
