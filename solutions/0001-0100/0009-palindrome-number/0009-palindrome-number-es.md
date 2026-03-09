---
title: "0009 Palindrome Number - ES"
problemUrl: "https://leetcode.com/problems/palindrome-number/"
difficulty: "Easy"
pubDate: "2026-03-08"
tags: ["math"]
complexity:
  time: "O(log n)"
  space: "O(1)"
---

# Palindrome Number

## Problema
Dado un entero `x`, devolver `true` si `x` es un palíndromo, es decir, si se lee igual de izquierda a derecha que de derecha a izquierda.

Los números negativos nunca son palíndromos (el signo `-` rompe la simetría). Tampoco lo son los números que terminan en `0`, excepto el propio `0`.

## Solución
La tentación inmediata es convertir el número a string y comparar caracteres. Funciona, pero usa memoria extra innecesaria.

El truco es **invertir solo la mitad del número**. Extraemos dígitos del final y los acumulamos hasta que la mitad invertida sea mayor o igual que la mitad restante. En ese punto, comparamos ambas mitades: si son iguales, es palíndromo. Si el número tiene cantidad impar de dígitos, dividimos la mitad invertida entre 10 para descartar el dígito central.

La condición `input > reversed` como guarda del `while` es la clave: nos garantiza que solo procesamos la mitad de los dígitos, lo cual además nos da complejidad `O(log n)`.

### Implementación en Rust

```rust
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut input = x;
        let mut reversed = 0;

        while input > reversed {
            reversed = reversed * 10 + input % 10;
            input /= 10;
        }

        input == reversed || input == reversed / 10
    }
}
```
