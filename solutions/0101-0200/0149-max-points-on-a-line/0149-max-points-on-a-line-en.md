---
title: "0149 Max Points on a Line - EN"
problemUrl: "https://leetcode.com/problems/max-points-on-a-line/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["hash-table", "math", "geometry"]
complexity:
  time: "O(N^2)"
  space: "O(N)"
---

# The Conspiracy of Collinear Points

## The Problem
Given an array of `points` where `points[i] = [xi, yi]` represents a point on the X-Y plane, return the maximum number of points that lie on the same straight line.

## The Geometry Hidden in Slopes

At first glance, this problem seems to invite a brute-force triple loop: for each pair of points define a line, then count how many other points fall on it. That would be O(N^3) and it works, but there's something more elegant waiting beneath the surface.

The fundamental observation is that a line is completely defined by a point and a slope. If I fix a reference point and compute the slope toward every other point, those that share the same slope are necessarily on the same line (the one passing through my reference point with that inclination). The problem thus reduces to: for each point, find the most popular slope among the slopes toward all other points.

## The Floating-Point Trap

Here lies the most dangerous trap in the problem. If we represent the slope as a floating-point number (`dy / dx`), the imprecisions of floating-point arithmetic can cause two slopes that should be identical to differ at the last decimal. Two points that are geometrically on the same line could produce "different" slopes when divided.

The solution is to never divide at all. Instead of storing the slope as a float, I represent it as a reduced fraction `(dy, dx)` where I've divided both components by their greatest common divisor. This turns each slope into a unique pair of integers that I can use as a key in a HashMap with zero risk of imprecision.

## Normalization: The Detail That Makes It Work

Representing the slope as `(dy, dx)` isn't sufficient on its own. The slope from point A to point B might produce `(2, -3)`, while from A to another collinear point C it might produce `(-2, 3)`. Geometrically it's the same slope, but the tuples are different.

To fix this, I normalize with two rules:

1. **Divide by the GCD** of the absolute values to get the irreducible fraction.
2. **Force `dx` to always be positive.** If `dx` is negative, I multiply both by -1. This guarantees that `(2, 3)` and `(-2, -3)` are represented identically.

Special cases also need attention: if `dx == 0`, the line is vertical and I encode it as `(1, 0)`. If `dy == 0`, the line is horizontal and I encode it as `(0, 1)`. With these conventions, every line has exactly one canonical representation.

## The Complete Algorithm

For each point `i`, I create a fresh HashMap. Then, for each point `j > i`, I compute the normalized slope between `i` and `j`, and increment the counter for that slope in the map. The maximum value in the map tells me how many points share the most common slope with point `i` -- adding 1 for point `i` itself, I get the total number of collinear points through it. The global maximum across all reference points is the answer.

### A Concrete Example

With the points `[[1,1], [2,2], [3,3]]`:

- Fixing point `[1,1]`: slope toward `[2,2]` is `(1,1)`, slope toward `[3,3]` is `(2,2)` which reduces to `(1,1)`. The HashMap has `{(1,1): 2}`. Local max: 2 + 1 = 3.

All three points lie on the same line, and the algorithm detects it without ever computing a float.

## Rust Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 2 {
            return n as i32;
        }

        let mut max_points = 1;

        for i in 0..n {
            let p1 = &points[i];
            let mut slopes: HashMap<(i32, i32), i32> = HashMap::new();

            for j in i + 1..n {
                let p2 = &points[j];

                let delta_y = p2[1] - p1[1];
                let delta_x = p2[0] - p1[0];

                let slope = Self::get_normalized_slope(delta_y, delta_x);

                *slopes.entry(slope).or_insert(0) += 1;
            }

            let current_max = slopes.values().max().unwrap_or(&0) + 1;
            max_points = max_points.max(current_max);
        }

        max_points
    }

    fn get_normalized_slope(dy: i32, dx: i32) -> (i32, i32) {
        if dx == 0 {
            return (1, 0);
        }
        if dy == 0 {
            return (0, 1);
        }

        let divisor = Self::gcd(dy.abs(), dx.abs());
        let mut res_dy = dy / divisor;
        let mut res_dx = dx / divisor;

        if res_dx < 0 {
            res_dy = -res_dy;
            res_dx = -res_dx;
        }

        (res_dy, res_dx)
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
```

The Rust implementation is particularly satisfying. The `HashMap<(i32, i32), i32>` uses integer tuples as keys, something that in many languages would require implementing custom hashing but in Rust works directly because tuples of types that implement `Hash` are automatically `Hash`. The `get_normalized_slope` function encapsulates all the normalization logic: it handles the special cases of vertical and horizontal lines first, then reduces the fraction with the GCD and forces the positive sign on `dx`. The outer loop only needs to iterate `j` from `i + 1` because the slope from `i` to `j` is identical to the slope from `j` to `i` -- this avoids duplicate work and the HashMap is reset on each outer iteration, keeping space at O(N).

## Conclusion

This problem hides an important lesson about data representation. The natural temptation to use floating-point division to compute slopes is precisely the trap that makes it Hard. By replacing a real number with a normalized irreducible fraction, we eliminate all ambiguity and convert a continuous geometry problem into a discrete counting problem with hash maps. The quadratic complexity is unavoidable -- we need to compare every pair of points -- but the constant factor is low and the implementation comes out clean. Sometimes, the best way to solve a geometry problem is to refuse to do geometry and think in integer arithmetic instead.
