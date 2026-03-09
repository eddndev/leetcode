---
title: "0066 Plus One - ES"
problemUrl: "https://leetcode.com/problems/plus-one/"
difficulty: "Easy"
pubDate: "2026-03-08"
tags: ["array", "math"]
complexity:
  time: "O(n)"
  space: "O(1)"
---

# Plus One

## Problema
Dado un entero grande representado como un array de dígitos `digits`, donde cada elemento es un dígito del número, sumar uno al número y devolver el array resultante. Los dígitos se almacenan de izquierda a derecha, del más significativo al menos significativo.

## Solución
Recorremos el array de derecha a izquierda. Si el dígito actual es menor que 9, simplemente le sumamos 1 y retornamos: no hay acarreo que propagar. Si el dígito es 9, se convierte en 0 y continuamos con el siguiente dígito hacia la izquierda.

Si salimos del bucle sin retornar, significa que todos los dígitos eran 9 (por ejemplo, `999`). En ese caso, todos quedaron en 0 y solo necesitamos insertar un `1` al inicio del array.

### Implementación en Rust

```rust
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for digit in digits.iter_mut().rev() {
            if *digit < 9 {
                *digit += 1;
                return digits;
            }
            *digit = 0;
        }
        digits.insert(0, 1);
        digits
    }
}
```
