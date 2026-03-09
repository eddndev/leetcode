---
title: "1200 Minimum Absolute Difference - ES"
problemUrl: "https://leetcode.com/problems/minimum-absolute-difference/"
difficulty: "Easy"
pubDate: "2026-03-08"
tags: ["array", "sorting"]
complexity:
  time: "O(n log n)"
  space: "O(n)"
---

# Minimum Absolute Difference

## Problema
Dado un array de enteros distintos `arr`, encontrar todos los pares de elementos que tengan la diferencia absoluta mínima entre cualquier par de elementos del array. Devolver una lista de pares ordenados de menor a mayor, donde cada par `[a, b]` cumple que `a < b`.

## Solución
Ordenamos el array. La diferencia absoluta mínima solo puede ocurrir entre elementos consecutivos en el array ordenado. Hacemos una primera pasada con ventanas de tamaño 2 para encontrar la diferencia mínima, y luego una segunda pasada para recopilar todos los pares consecutivos cuya diferencia sea igual a esa mínima.

### Implementación en Rust

```rust
impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();

        let min_diff = arr.windows(2).map(|w| w[1] - w[0]).min().unwrap();

        arr.windows(2)
            .filter(|w| (w[1] - w[0]) == min_diff)
            .map(|w| vec![w[0], w[1]])
            .collect()
    }
}
```
