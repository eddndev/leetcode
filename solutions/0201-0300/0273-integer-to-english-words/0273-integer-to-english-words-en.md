---
title: "0273 Integer to English Words - EN"
problemUrl: "https://leetcode.com/problems/integer-to-english-words/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "recursion", "math"]
complexity:
  time: "O(1) since the input is bounded by 2^31 - 1 (at most 10 digits)"
  space: "O(1) for the same reason"
---

# Teaching a Machine to Read Numbers Aloud

## The Problem
Given a non-negative integer `num`, convert it to its English words representation. The input is guaranteed to be less than 2^31 - 1 (at most 2,147,483,647). For example, `1234567` becomes `"One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"`, and `0` becomes `"Zero"`.

## Why This Problem Is Deceptively Tricky

At first glance, this looks like a straightforward mapping exercise. Just look up each digit and concatenate words, right? But the English language does not work that way. Numbers are not read digit by digit -- they are read in groups of three with scale words (Thousand, Million, Billion) separating the groups. Within each group, the rules change depending on the magnitude: hundreds have their own word, the teens are irregular (Eleven, Twelve, Thirteen...), and the tens follow yet another pattern (Twenty, Thirty, Forty...). A brute-force series of conditionals would sprawl into an unreadable mess.

The real challenge is finding a clean decomposition that handles all these cases without duplication.

## The Strategy: Divide by Thousands, Conquer by Recursion

### Chunking into Groups of Three

I noticed that English reads large numbers in groups of three digits, separated by scale words. Take `2,147,483,647`: it is read as "Two Billion" + "One Hundred Forty Seven Million" + "Four Hundred Eighty Three Thousand" + "Six Hundred Forty Seven". Each group of three digits follows the exact same rules -- it is just a number from 0 to 999. The only difference is which scale word follows it.

So the outer loop is simple: repeatedly extract the last three digits via `n % 1000`, convert them to words using a helper, append the appropriate scale word (Thousand, Million, or Billion), and then shift `n` right by dividing by 1000.

### The Helper: Converting 0-999

The helper function handles numbers from 0 to 999 using three tiers of logic:

1. **Base case (0)**: return an empty string, because a zero-group should not produce any words (we do not say "Zero Thousand").

2. **Less than 20**: direct lookup from an array. This covers the unique words One through Nineteen. The teens are irregular in English, so there is no shortcut -- they must be listed explicitly.

3. **20 to 99**: the tens word (Twenty, Thirty, ..., Ninety) plus a recursive call for the ones digit.

4. **100 to 999**: the hundreds word plus "Hundred" plus a recursive call for the remainder (the last two digits).

The recursion is shallow -- at most two levels deep -- so there is no risk of stack overflow or performance issues.

### Handling Spacing

I prepend a space before each word in the lookup arrays (e.g., `" One"`, `" Two"`). This means every fragment naturally starts with a space, and the final result just needs a `.trim()` to remove the leading space. This approach avoids the messy logic of conditionally inserting spaces between words.

### A Walkthrough

For `num = 1234567891`:

```
Iteration 1: n % 1000 = 891.  helper(891) = " Eight Hundred Ninety One"
  Scale: "" (ones group). Partial result: " Eight Hundred Ninety One"

Iteration 2: n % 1000 = 567.  helper(567) = " Five Hundred Sixty Seven"
  Scale: " Thousand". Partial result: " Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"

Iteration 3: n % 1000 = 234.  helper(234) = " Two Hundred Thirty Four"
  Scale: " Million". Partial result: " Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"

Iteration 4: n % 1000 = 1.    helper(1) = " One"
  Scale: " Billion". Partial result: " One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"

After trim: "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
```

Notice how `n % 1000 != 0` guards against zero-groups. If the input were `1000000`, the second and third groups (both 000) are skipped entirely, producing just `"One Million"` with no spurious "Thousand" or trailing spaces.

## Rust Solution

```rust
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
```

The outer function handles the special case of zero upfront -- it is the only input that should produce the word "Zero". The `thousands` array maps group index to scale word: index 0 is the ones group (no suffix), 1 is Thousand, 2 is Million, 3 is Billion. The `while` loop peels off three digits at a time from the right. The guard `n % 1000 != 0` ensures we never produce output for a zero-group, which would otherwise insert a dangling scale word like "Thousand" with no number before it. The `helper` function is a small recursive converter for numbers 0-999. By embedding a leading space in every word in the lookup arrays, the concatenation is always clean -- `format!` just glues fragments together, and a single `trim()` at the end strips the one extra leading space. The recursion depth never exceeds two (hundreds -> tens -> ones), so this is effectively iterative in cost.

## Conclusion

Integer to English Words is a problem that rewards structural thinking over brute-force case analysis. The key insight is that English naturally groups digits by threes, so the conversion decomposes into an outer loop over thousand-groups and an inner recursive handler for numbers 0-999. The teens irregularity is handled by a flat lookup table, the tens by another, and the hundreds by a recursive call. Leading spaces baked into the word arrays eliminate fiddly spacing logic. The result is a clean, readable solution that handles every edge case -- from zero to two billion -- without a single ad-hoc conditional.
