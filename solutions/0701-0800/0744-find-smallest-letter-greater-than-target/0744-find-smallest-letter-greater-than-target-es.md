---
title: "0744 Find Smallest Letter Greater Than Target - ES"
problemUrl: "https://leetcode.com/problems/find-smallest-letter-greater-than-target/"
difficulty: "Easy"
pubDate: "2026-03-08"
tags: ["array", "binary-search"]
complexity:
  time: "O(n)"
  space: "O(1)"
---

# Find Smallest Letter Greater Than Target

## Problema
Dado un array de caracteres `letters` ordenado en orden ascendente y un carácter `target`, devolver el menor carácter en `letters` que sea estrictamente mayor que `target`. Si no existe tal carácter, devolver el primer elemento del array (el array se considera circular).

## Solución
Recorremos el array de izquierda a derecha. El primer carácter que sea estrictamente mayor que `target` es la respuesta, ya que el array está ordenado. Si ningún carácter cumple la condición, devolvemos el primer elemento del array.

### Implementación en Rust

```rust
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        for &c in &letters {
            if c > target {
                return c;
            }
        }

        letters[0]
    }
}
```
