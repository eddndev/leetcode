---
title: "0087 Scramble String - ES"
problemUrl: "https://leetcode.com/problems/scramble-string/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "dynamic-programming", "recursion", "memoization"]
complexity:
  time: "O(N^4)"
  space: "O(N^3)"
---

# Desenredando el Arbol de Recursion

## El Problema
Dadas dos cadenas `s1` y `s2` de la misma longitud, determinar si `s2` es una version "scrambled" de `s1`. Una cadena scrambled se forma dividiendo recursivamente la cadena en dos partes no vacias en cualquier posicion, opcionalmente intercambiandolas, y luego continuando el proceso en las subcadenas resultantes.

## La Primera Impresion

Cuando me encontre con este problema por primera vez, parecia enganosamente simple: solo verificar si una cadena puede reorganizarse en otra. Pero "scramble" no significa "anagrama". La transformacion es recursiva y estructural: eliges un punto de corte en la cadena, la divides en dos partes, opcionalmente intercambias esas partes, y luego repites el proceso en cada parte de forma independiente. Esa naturaleza de arbol es lo que lo hace complejo.

Mi pensamiento inmediato fue recursion. Para dos cadenas de longitud `n`, puedo probar cada punto de corte posible `k` desde `1` hasta `n-1`. En cada corte, hay dos posibilidades:

1. **Sin intercambio:** La parte izquierda de `s1` (longitud `k`) coincide con la parte izquierda de `s2`, y la parte derecha de `s1` (longitud `n-k`) coincide con la parte derecha de `s2`.
2. **Con intercambio:** La parte izquierda de `s1` (longitud `k`) coincide con la parte **derecha** de `s2`, y la parte derecha de `s1` (longitud `n-k`) coincide con la parte **izquierda** de `s2`.

Esto nos da la estructura recursiva, pero sin memoizacion, los subproblemas superpuestos causarian una explosion exponencial. Ahi es donde la tabla memo se vuelve esencial.

## La Poda Que Lo Hace Funcionar

Antes de lanzarse a la recursion para un par de subcadenas dado, hay una optimizacion critica: verificar si siquiera son anagramas entre si. Si dos subcadenas no contienen las mismas frecuencias de caracteres, no tiene sentido explorar mas. Esto se hace con un simple arreglo de frecuencias de tamano 26. Esta poda elimina una cantidad masiva de ramas tempranamente y es lo que hace que la solucion sea practica a pesar de la complejidad teorica O(N^4).

Tambien esta el caso base trivial: si las dos subcadenas ya son identicas, retornamos `true` inmediatamente sin ningun corte adicional.

## La Tabla Memo

El estado de cada subproblema esta definido por tres valores: el indice de inicio en `s1`, el indice de inicio en `s2`, y la longitud de la subcadena que se esta comparando. Esto nos da una tabla memo tridimensional de tamano `(n+1) x n x n`, donde cada celda almacena un `Option<bool>`: `None` si aun no se ha calculado, `Some(true)` o `Some(false)` si ya se resolvio.

Al cachear cada resultado de subproblema, garantizamos que ningun par de subcadenas se evalua mas de una vez, convirtiendo la recursion exponencial en tiempo polinomial.

## Solucion en Rust

```rust
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let n = s1.len();
        if n != s2.len() {
            return false;
        }
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();

        let mut memo = vec![vec![vec![None; n]; n]; n + 1];

        Self::solve(s1_bytes, s2_bytes, 0, 0, n, &mut memo)
    }

    fn solve(
        s1: &[u8],
        s2: &[u8],
        i1: usize,
        i2: usize,
        len: usize,
        memo: &mut Vec<Vec<Vec<Option<bool>>>>,
    ) -> bool {
        if let Some(res) = memo[len][i1][i2] {
            return res;
        }
        if s1[i1..i1 + len] == s2[i2..i2 + len] {
            memo[len][i1][i2] = Some(true);
            return true;
        }

        let mut counts = [0; 26];
        for k in 0..len {
            counts[(s1[i1 + k] - b'a') as usize] += 1;
            counts[(s2[i2 + k] - b'a') as usize] -= 1;
        }
        if counts.iter().any(|&c| c != 0) {
            memo[len][i1][i2] = Some(false);
            return false;
        }

        for k in 1..len {
            if Self::solve(s1, s2, i1, i2, k, memo)
                && Self::solve(s1, s2, i1 + k, i2 + k, len - k, memo)
            {
                memo[len][i1][i2] = Some(true);
                return true;
            }

            if Self::solve(s1, s2, i1, i2 + len - k, k, memo)
                && Self::solve(s1, s2, i1 + k, i2, len - k, memo)
            {
                memo[len][i1][i2] = Some(true);
                return true;
            }
        }

        memo[len][i1][i2] = Some(false);
        false
    }
}
```

La implementacion en Rust aprovecha las fortalezas del lenguaje. Convertimos ambas cadenas a `&[u8]` con `as_bytes()` para que las comparaciones de subcadenas y la aritmetica de caracteres sean simples operaciones de bytes. La tabla memo usa `Option<bool>` para distinguir limpiamente entre "aun no calculado" y un resultado real, lo cual es mas expresivo que usar un valor centinela. La funcion `solve` es un DP top-down clasico: revisa el memo primero, prueba el caso trivial, poda con la verificacion de anagrama, y luego explora todos los puntos de corte con ambas configuraciones (con y sin intercambio). En el momento en que se encuentra un corte valido, hacemos cortocircuito y retornamos `true`.

## Conclusion

Este problema es un ejemplo hermoso de como la recursion con memoizacion puede domar una explosion combinatoria. La estructura recursiva refleja la definicion del problema perfectamente: probar cada corte, probar ambos ordenes, y cachear todo. La poda por anagrama es el ingrediente practico que mantiene el espacio de busqueda manejable. Sin ella, la solucion seguiria siendo correcta pero dolorosamente lenta. Con ella, obtenemos una solucion limpia O(N^4) que maneja las restricciones comodamente.
