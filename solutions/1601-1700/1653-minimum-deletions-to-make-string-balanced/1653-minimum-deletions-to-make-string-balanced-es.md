---
title: "1653 Minimum Deletions to Make String Balanced - ES"
problemUrl: "https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["string", "dynamic-programming", "stack"]
complexity:
  time: "O(n)"
  space: "O(1)"
---

# Minimum Deletions to Make String Balanced: Rastreando el Costo Sobre la Marcha

## El Problema
Dada una cadena `s` que consiste unicamente de los caracteres `'a'` y `'b'`, se puede eliminar cualquier cantidad de caracteres de `s` para que la cadena restante quede balanceada. Una cadena esta balanceada si no existe ningun par de indices `(i, j)` tal que `i < j`, `s[i] = 'b'` y `s[j] = 'a'`. En otras palabras, todas las `'a'`s deben aparecer antes que todas las `'b'`s. Devolver el numero minimo de eliminaciones necesarias.

El enfoque de fuerza bruta intentaria cada punto de particion posible y contaria cuantas `'b'`s aparecen antes y cuantas `'a'`s aparecen despues. Eso funciona pero requiere precalcular sumas de prefijos. Hay una forma mas limpia que lo resuelve en una sola pasada.

## La Intuicion: Una Decision Continua

La idea clave es pensar en lo que sucede mientras recorremos la cadena de izquierda a derecha. Mantenemos dos estados: la cantidad de `'b'`s que hemos visto hasta ahora, y el minimo de eliminaciones necesarias para mantener todo balanceado hasta la posicion actual.

Cuando encontramos una `'b'`, no causa ningun problema por si sola. Una `'b'` que aparece despues de los caracteres anteriores esta bien para el balance. Simplemente incrementamos nuestro contador de `'b'`s.

Cuando encontramos una `'a'`, enfrentamos una decision. Esta `'a'` aparece despues de todas las `'b'`s que hemos visto hasta ahora, lo cual viola la condicion de balance. Tenemos dos opciones: eliminar esta `'a'` (lo que nos cuesta una eliminacion mas sobre nuestro resultado actual), o eliminar todas las `'b'`s que hemos visto hasta ahora (lo que cuesta exactamente `b_count`). Elegimos la que sea mas barata.

Esta es la transicion de DP disfrazada: `res = min(res + 1, b_count)`. La belleza esta en que `res + 1` representa "mantener la solucion optima anterior y simplemente eliminar esta nueva `'a'`", mientras que `b_count` representa "empezar de cero eliminando todas las `'b'`s vistas hasta ahora, haciendo que esta `'a'` sea valida." El minimo de ambos siempre nos da el optimo global en cada paso.

## Solucion en Rust

Iterar sobre `s.bytes()` en lugar de `s.chars()` evita el overhead de decodificacion UTF-8 ya que solo nos interesan caracteres ASCII. La llamada a `std::cmp::min` es toda la logica de DP condensada en una sola linea.

```rust
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut b_count = 0;
        let mut res = 0;

        for c in s.bytes() {
            if c == b'a' {
                res = std::cmp::min(res + 1, b_count);
            } else {
                b_count += 1;
            }
        }
        res
    }
}
```

## Conclusion

La complejidad temporal es $O(n)$ ya que hacemos una sola pasada por la cadena, y la complejidad espacial es $O(1)$ ya que solo rastreamos dos enteros. Lo que hace este problema satisfactorio es como la eleccion greedy en cada `'a'` -- eliminarla o borrar todas las `'b'`s anteriores -- produce naturalmente el minimo global sin necesidad de retroceder. Es un buen ejemplo de como la programacion dinamica a veces puede colapsar en un recorrido lineal con espacio constante cuando el estado es suficientemente pequeno.
