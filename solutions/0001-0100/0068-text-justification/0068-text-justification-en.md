---
title: "0068 Text Justification - EN"
problemUrl: "https://leetcode.com/problems/text-justification/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "simulation", "greedy"]
complexity:
  time: "O(N)"
  space: "O(N)"
---

# The Art of Typesetting by Hand

## The Problem
Given a list of words and a maximum width `maxWidth`, format the text such that each line has exactly `maxWidth` characters and is fully justified (aligned on both left and right). Words should be packed using a greedy algorithm: each line should contain as many words as possible, and extra spaces should be distributed as evenly as possible among the gaps. The last line must be left-aligned, padded with trailing spaces to fill the width.

## The Initial Intuition

When I first read this problem, my immediate thought was: this isn't a classic algorithms problem, it's an engineering problem. There's no hidden mathematical trick or exotic data structure. What's needed is to simulate exactly what a word processor does, step by step, without getting the details wrong.

And that's precisely where the difficulty lies. The edge cases are numerous: lines with a single word (no gaps to distribute spaces into), the last line (left-aligned instead of justified), and uneven space distribution (left gaps receive an extra space when the division isn't exact). The problem is "Hard" not because the idea is complex, but because the implementation must be flawless.

## The Greedy Packing Strategy

The first phase is deciding which words go on each line. I use a greedy approach: I accumulate words as long as the sum of their lengths plus the minimum spaces (one between each pair of words) doesn't exceed `maxWidth`. The moment a new word doesn't fit, I justify the current line and start a new one.

The key condition is `current_len + word.len() + current_line.len() > max_width`. Here `current_len` is the total character count of the accumulated words, and `current_line.len()` is the word count, which equals the minimum number of spaces needed if we were to add the new word (one space between each pair). If that sum exceeds the width, the word doesn't fit and we must justify what we have.

## Line-by-Line Justification

Once I know which words belong on a line, there are two cases:

**Case 1: Last line or a line with a single word.** Justification here is straightforward: words are separated by a single space, and the remaining width is filled with trailing spaces. No proportional distribution.

**Case 2: Intermediate line with multiple words.** I have `gaps` slots (one fewer than the number of words) and `total_spaces` spaces to distribute (total width minus the characters from words). I divide: `spaces_per_gap = total_spaces / gaps` gives me the base spaces per slot, and `extra_spaces = total_spaces % gaps` tells me how many slots get one more. The first `extra_spaces` gaps receive `spaces_per_gap + 1` spaces, and the rest receive `spaces_per_gap`. This guarantees that the difference between any two gaps is at most 1, and that the wider gaps are on the left, exactly as the problem requires.

### A Step-by-step Example

For `words = ["This", "is", "an", "example", "of", "text", "justification."]` with `maxWidth = 16`:

- **Line 1:** "This" (4) + "is" (2) + "an" (2) = 8 characters, 3 words. Try adding "example" (7): 8 + 7 + 3 = 18 > 16. Doesn't fit. Justify: 16 - 8 = 8 spaces for 2 gaps: 4 and 4. Result: `"This    is    an"`.
- **Line 2:** "example" (7) + "of" (2) + "text" (4) = 13 characters, 3 words. Try adding "justification." (14): 13 + 14 + 3 = 30 > 16. Doesn't fit. Justify: 16 - 13 = 3 spaces for 2 gaps: 2 and 1. Result: `"example  of text"`.
- **Line 3:** "justification." (14), last line. Left-align with padding: `"justification.  "`.

## Rust Solution

```rust
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut result = Vec::new();
        let mut current_line: Vec<&String> = Vec::new();
        let mut current_len = 0;

        for word in &words {
            if current_len + word.len() + current_line.len() > max_width {
                result.push(Self::justify_line(
                    &current_line,
                    current_len,
                    max_width,
                    false,
                ));

                current_line.clear();
                current_len = 0;
            }

            current_line.push(word);
            current_len += word.len();
        }

        if !current_line.is_empty() {
            result.push(Self::justify_line(
                &current_line,
                current_len,
                max_width,
                true,
            ));
        }

        result
    }

    fn justify_line(
        line: &[&String],
        line_char_len: usize,
        max_width: usize,
        is_last_line: bool,
    ) -> String {
        if is_last_line || line.len() == 1 {
            let mut s = String::with_capacity(max_width);

            for (i, word) in line.iter().enumerate() {
                if i > 0 {
                    s.push(' ');
                }
                s.push_str(word);
            }

            let remaining = max_width - s.len();
            for _ in 0..remaining {
                s.push(' ');
            }
            return s;
        }

        let gaps = line.len() - 1;
        let total_spaces = max_width - line_char_len;

        let spaces_per_gap = total_spaces / gaps;
        let extra_spaces = total_spaces % gaps;

        let mut s = String::with_capacity(max_width);

        for (i, word) in line.iter().enumerate() {
            s.push_str(word);

            if i < gaps {
                let spaces_to_add = spaces_per_gap + if i < extra_spaces { 1 } else { 0 };
                for _ in 0..spaces_to_add {
                    s.push(' ');
                }
            }
        }

        s
    }
}
```

The Rust implementation cleanly separates the two responsibilities: `full_justify` handles the greedy packing, and `justify_line` handles the space distribution. The use of `String::with_capacity(max_width)` is a detail that avoids unnecessary reallocations, since we know the exact size of each line in advance. The helper function receives an `is_last_line` flag to distinguish between full justification and left alignment, which keeps the logic clean without duplicating code. The division-and-modulo arithmetic for distributing spaces is concise and correct: `spaces_per_gap` provides the uniform base, and the first `extra_spaces` gaps absorb the remainder, guaranteeing that the maximum difference between any two gaps is exactly one.

## Conclusion

Text Justification is one of those problems that seem simple until you sit down to implement them. There's no "aha" moment, no clever data structure to discover. The difficulty lies in correctly handling every case: single-word lines, the last line, uneven space distribution. It's a pure exercise in implementation precision, and it's exactly the kind of problem that separates understanding an algorithm from translating it into bug-free code. Sometimes the real challenge isn't finding the idea, but executing it without cracks.
