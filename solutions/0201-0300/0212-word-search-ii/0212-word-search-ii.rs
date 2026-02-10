use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    word: Option<String>,
}

impl TrieNode {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.word = Some(word);
    }
}
impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = TrieNode::new();

        for word in words {
            root.insert(word);
        }

        let rows = board.len();
        let cols = board[0].len();
        let mut result = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if let Some(next_node) = &mut root.children[char_to_idx(board[r][c])] {
                    dfs(&mut board, r, c, next_node, &mut result);
                }
            }
        }

        result
    }
}

#[inline(always)]
fn char_to_idx(c: char) -> usize {
    (c as u8 - b'a') as usize
}

fn dfs(
    board: &mut Vec<Vec<char>>,
    r: usize,
    c: usize,
    node: &mut TrieNode,
    result: &mut Vec<String>,
) {
    if let Some(w) = node.word.take() {
        result.push(w);
    }

    let original_char = board[r][c];
    board[r][c] = '#';

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dr, dc) in directions {
        let new_r = r.wrapping_add(dr as usize);
        let new_c = c.wrapping_add(dc as usize);

        if new_r < board.len() && new_c < board[0].len() {
            let next_char = board[new_r][new_c];
            if next_char != '#' {
                if let Some(next_node) = &mut node.children[char_to_idx(next_char)] {
                    dfs(board, new_r, new_c, next_node, result);
                }
            }
        }
    }
    board[r][c] = original_char;
}
