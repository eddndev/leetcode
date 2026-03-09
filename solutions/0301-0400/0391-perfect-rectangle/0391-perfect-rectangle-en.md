---
title: "0391 Perfect Rectangle - EN"
problemUrl: "https://leetcode.com/problems/perfect-rectangle/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["geometry", "hash-set", "math", "sweep-line"]
complexity:
  time: "O(N) where N is the number of rectangles"
  space: "O(N)"
---

# The Jigsaw of Axis-Aligned Pieces

## The Problem
Given an array `rectangles` where `rectangles[i] = [xi, yi, ai, bi]` represents an axis-aligned rectangle with bottom-left corner `(xi, yi)` and top-right corner `(ai, bi)`, return `true` if all the rectangles together form an exact cover of a rectangular region with no overlaps and no gaps.

## The Initial Intuition

At first glance, this seems like it might require sweepline algorithms or complex interval tracking to detect overlaps and gaps among thousands of rectangles. But there is a beautifully elegant observation that reduces the entire problem to two simple checks: one on areas and one on corners.

If the rectangles form a perfect cover, two things must be true simultaneously. First, the total area of all individual rectangles must equal the area of the bounding rectangle -- the smallest rectangle that contains all of them. Second, and more subtly, the corners must behave in a very specific way.

## The Corner Parity Insight

Consider what happens at corners when rectangles tile a plane perfectly. Interior corners -- those shared by multiple rectangles -- always appear an even number of times (two rectangles share an edge, or four meet at a point). These cancel out. The only corners that appear an odd number of times are the four corners of the overall bounding rectangle, because each of them belongs to exactly one small rectangle.

This gives me a powerful invariant: if I track every corner of every rectangle using a set, toggling membership (inserting if absent, removing if present), then after processing all rectangles, the set should contain exactly four points -- and those four points must be the corners of the bounding rectangle.

## Area Verification Prevents False Positives

The corner check alone is not sufficient. Consider two identical rectangles stacked on top of each other -- their corners would cancel perfectly, but they overlap. The area check catches this: the sum of individual areas would be double the bounding rectangle's area. Conversely, the area check alone does not prevent configurations where rectangles fit within the bounding box without covering it completely. Together, the two checks form a necessary and sufficient condition for a perfect rectangle cover.

## Putting It Together

My algorithm makes a single pass through all rectangles. For each one, I update the global bounding box by tracking `min_x`, `min_y`, `max_x`, and `max_y`. I accumulate the total area using `i64` to avoid overflow. I toggle each of the four corners in a `HashSet` -- if the corner is already present, I remove it; otherwise, I insert it.

After the pass, I verify three things: the set contains exactly four corners, those corners match the bounding rectangle, and the accumulated area equals the bounding rectangle's area.

## Rust Solution

```rust
use std::collections::HashSet;
use std::i32;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        if rectangles.is_empty() {
            return false;
        }

        let mut corners = HashSet::new();
        let mut area_sum: i64 = 0;

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for rect in &rectangles {
            let x1 = rect[0];
            let y1 = rect[1];
            let x2 = rect[2];
            let y2 = rect[3];

            min_x = min_x.min(x1);
            min_y = min_y.min(y1);
            max_x = max_x.max(x2);
            max_y = max_y.max(y2);

            area_sum += (x2 - x1) as i64 * (y2 - y1) as i64;

            let points = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)];

            for p in points.iter() {
                if corners.contains(p) {
                    corners.remove(p);
                } else {
                    corners.insert(*p);
                }
            }
        }

        if corners.len() != 4 {
            return false;
        }

        let expected_corners = [
            (min_x, min_y),
            (min_x, max_y),
            (max_x, min_y),
            (max_x, max_y),
        ];

        for p in expected_corners.iter() {
            if !corners.contains(p) {
                return false;
            }
        }

        let expected_area = (max_x - min_x) as i64 * (max_y - min_y) as i64;

        area_sum == expected_area
    }
}
```

The implementation is refreshingly straightforward for a Hard problem. The `HashSet` of `(i32, i32)` tuples serves as the toggle mechanism -- each corner is either added or removed, effectively computing parity. The area computation uses `i64` to handle cases where coordinates range up to 100,000 and the product of two differences could overflow `i32`. After the single pass, the three validation checks run in constant time since I am only comparing against four expected corners and one expected area.

## Conclusion

Perfect Rectangle is one of those problems where the difficulty lies not in the code but in the insight. The brute-force approach of checking every pair of rectangles for overlap would be quadratic and painful. The sweepline approach works but requires careful interval tree management. The corner parity approach, combined with area verification, solves the problem in O(N) time with O(N) space, and the resulting code is barely twenty lines of logic. The key realization -- that a perfect tiling produces exactly four surviving corners after parity cancellation, and that these must be the bounding rectangle's corners -- transforms a geometric nightmare into an exercise in set manipulation. It is a reminder that the most powerful algorithms sometimes come not from sophisticated data structures, but from a deep geometric observation.
