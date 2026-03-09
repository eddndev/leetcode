---
title: "0115 Distinct Subsequences - ES"
problemUrl: "https://leetcode.com/problems/distinct-subsequences/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "dynamic-programming"]
complexity:
  time: "O(M * N)"
  space: "O(M)"
---

# Contando Fantasmas Dentro de una Cadena

## El Problema
Dadas dos cadenas `s` y `t`, devolver el numero de subsecuencias distintas de `s` que son iguales a `t`. Una subsecuencia es una secuencia que se puede derivar de otra eliminando algunos elementos (o ninguno) sin cambiar el orden de los elementos restantes. Se garantiza que la respuesta cabe en un entero con signo de 32 bits.

## La Primera Impresion

A primera vista, este problema parece engañosamente cercano a un simple ejercicio de coincidencia de cadenas. Pero no pregunta *si* `t` existe dentro de `s` -- pregunta *de cuantas formas distintas* puedes extraer `t` de `s` eliminando caracteres selectivamente. Cada conjunto distinto de indices que eliges conservar forma una subsecuencia separada, y dos subsecuencias cuentan como diferentes aunque produzcan los mismos caracteres, siempre que las posiciones de los indices difieran.

Mi primer instinto fue la fuerza bruta: enumerar cada subconjunto posible de indices en `s` y verificar si la cadena resultante es igual a `t`. Pero con cadenas de hasta longitud 1000, eso es una explosion combinatoria. La idea clave es que este problema tiene **subestructura optima**: el numero de formas de construir `t[0..j]` a partir de `s[0..i]` depende de subproblemas mas pequeños, y estos subproblemas se superponen intensamente. Esa es la señal clasica de la programacion dinamica.

## Aplanando la Tabla en una Sola Fila

El enfoque clasico en 2D definiria `dp[i][j]` como el numero de formas de construir los primeros `j` caracteres de `t` a partir de los primeros `i` caracteres de `s`. La recurrencia es directa: para cada caracter en `s`, si coincide con `t[j]`, sumamos `dp[i-1][j-1]` (usar este caracter) a `dp[i-1][j]` (saltarlo); de lo contrario, simplemente arrastramos `dp[i-1][j]`.

Pero notemos que cada fila solo depende de la fila anterior. Eso significa que podemos colapsar toda la tabla en un unico arreglo unidimensional de longitud `m + 1`, donde `m = len(t)`. El truco esta en iterar **hacia atras** al actualizar el arreglo, para no sobreescribir valores que aun necesitamos en el calculo de la fila actual.

### El caso base

`dp[0] = 1`: hay exactamente una forma de construir una subsecuencia vacia desde cualquier prefijo de `s` -- no seleccionando nada. Todas las demas posiciones comienzan en cero.

### La transicion

Para cada caracter `s[i]`, recorremos `j` desde `m-1` hasta `0`. Si `s[i] == t[j]`, hacemos `dp[j+1] += dp[j]`. Esto captura la decision de "usar" `s[i]` para hacer match con `t[j]`, acumulando el conteo del subproblema donde `t[0..j]` ya fue construido. Al ir en reversa, cada `dp[j]` aun conserva su valor de la iteracion anterior de `s`, que es exactamente el escenario de "saltar este caracter" -- no necesitamos hacer nada extra para eso.

### ¿Por que `u64`?

Los conteos intermedios pueden crecer mas alla de lo que un entero de 32 bits puede contener, aunque la respuesta final quepa en `i32`. Usar `u64` para el arreglo DP evita el desbordamiento durante la acumulacion. Al final, convertimos `dp[m]` a `i32`.

## Solucion en Rust

```rust
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let m = t.len();
        let n = s.len();

        if n < m {
            return 0;
        }

        let mut dp = vec![0u64; m + 1];

        dp[0] = 1;

        for &s_char in s_bytes {
            for j in (0..m).rev() {
                if s_char == t_bytes[j] {
                    dp[j + 1] += dp[j];
                }
            }
        }

        dp[m] as i32
    }
}
```

La implementacion en Rust es notablemente concisa gracias al enfoque optimizado en espacio. Convertimos ambas cadenas a `&[u8]` con `as_bytes()` para trabajar a nivel de bytes, lo cual es eficiente y suficiente ya que el problema trabaja con caracteres ASCII. El retorno temprano cuando `n < m` es una guarda pequeña pero importante: si `s` es mas corta que `t`, no puede existir ninguna subsecuencia de `s` igual a `t`. La iteracion inversa `(0..m).rev()` es la pieza clave que hace correcta la optimizacion 1D -- asegura que leemos cada `dp[j]` antes de que sea modificado en el paso actual.

## Conclusion

Este problema es un hermoso ejemplo de como una pregunta de conteo sobre subsecuencias se mapea naturalmente a la programacion dinamica. La tabla 2D hace la recurrencia intuitiva, y la observacion de que cada fila solo depende de la anterior nos permite aplanar el espacio a `O(M)`. El truco de la iteracion inversa es un patron que vale la pena memorizar -- aparece en muchos problemas de DP donde se optimiza de dos dimensiones a una, como en la mochila 0/1. Al final, lo que parece un desafio combinatorio abrumador se reduce a un recorrido lineal limpio con un elegante bucle interno.
