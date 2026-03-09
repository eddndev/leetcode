---
title: "0132 Palindrome Partitioning II - ES"
problemUrl: "https://leetcode.com/problems/palindrome-partitioning-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "dynamic-programming", "palindrome"]
complexity:
  time: "O(N^2)"
  space: "O(N)"
---

# Rompiendo Espejos con el Menor Numero de Golpes

## El Problema
Dada una cadena `s`, devolver el numero minimo de cortes necesarios para que cada subcadena resultante de la particion sea un palindromo.

## La Primera Impresion

Este problema es la continuacion natural de "Palindrome Partitioning" (donde se piden todas las particiones posibles), pero aqui el enfoque cambia radicalmente: ya no queremos enumerar todas las formas de cortar la cadena en palindromos, sino encontrar la que requiere **el menor numero de cortes**. Esa diferencia transforma el problema de uno de backtracking exhaustivo a uno de optimizacion pura.

Mi primer pensamiento fue intentar una tabla 2D clasica: `is_palindrome[i][j]` para saber si `s[i..=j]` es palindromo, combinada con otra tabla `dp[i]` que almacene el numero minimo de cortes para el prefijo `s[0..=i]`. Pero eso requiere `O(N^2)` de espacio solo para la tabla de palindromos. La pregunta es: podemos hacer algo mas elegante?

## Expansion desde el Centro

La clave esta en cambiar la perspectiva. En lugar de preguntar "para cada posicion final, cual es el mejor corte?", expandimos palindromos desde cada posible centro. Cada vez que encontramos un palindromo `s[l..=r]`, sabemos que podemos llegar a la posicion `r` con un corte justo antes de `l` (es decir, `dp[l-1] + 1`), o con cero cortes si `l == 0` (porque toda la subcadena desde el inicio es un palindromo). Si este valor es menor que el `dp[r]` actual, lo actualizamos.

### El arreglo DP

Inicializamos `dp[i] = i` para toda posicion. Esto representa el peor caso: cortar entre cada par de caracteres adyacentes, lo que siempre produce palindromos de un solo caracter. A partir de ahi, cada palindromo que descubramos solo puede mejorar estos valores.

### Palindromos de longitud impar y par

Para cada centro, realizamos dos expansiones:
- **Impar**: empezamos con `l = r = center`, expandiendo simetricamente mientras `s[l] == s[r]`.
- **Par**: empezamos con `l = center, r = center + 1`, capturando palindromos de longitud par como `"aa"` o `"abba"`.

En ambos casos, dentro del bucle de expansion, calculamos el corte potencial y actualizamos `dp[r]` si encontramos una mejora. Si `l` llega a `0`, el palindromo cubre desde el inicio de la cadena, asi que el costo es `0` (sin cortes necesarios para ese segmento).

### ¿Por que funciona?

Cada palindromo de la cadena sera descubierto por alguna expansion centrada. Al procesar todos los centros de izquierda a derecha, cuando evaluamos `dp[l-1]`, ese valor ya ha sido optimizado por todos los centros anteriores. Asi, el valor final `dp[n-1]` contiene el numero minimo de cortes para toda la cadena.

## Solucion en Rust

```rust
use std::cmp;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }

        let s = s.as_bytes();

        let mut dp: Vec<i32> = (0..n as i32).collect();

        for center in 0..n {
            let (mut l, mut r) = (center, center);
            while r < n && s[l] == s[r] {
                let new_cut = if l == 0 { 0 } else { dp[l - 1] + 1 };
                if new_cut < dp[r] {
                    dp[r] = new_cut;
                }

                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            }

            let (mut l, mut r) = (center, center + 1);
            while r < n && s[l] == s[r] {
                let new_cut = if l == 0 { 0 } else { dp[l - 1] + 1 };
                if new_cut < dp[r] {
                    dp[r] = new_cut;
                }

                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            }
        }

        dp[n - 1]
    }
}
```

La implementacion en Rust es directa y eficiente. Convertimos la cadena a `&[u8]` con `as_bytes()` para comparaciones rapidas a nivel de bytes. El vector `dp` se inicializa con `(0..n as i32).collect()`, que genera la secuencia `[0, 1, 2, ..., n-1]` -- el escenario de peor caso. Los dos bloques `while` son estructuralmente identicos excepto por la inicializacion de `(l, r)`: el primero maneja palindromos de longitud impar y el segundo los de longitud par. La guarda `if l == 0 { break; }` es necesaria porque `l` es `usize` (entero sin signo en Rust), y restar 1 de cero causaria un panic por desbordamiento. Notar que el `use std::cmp` en la parte superior del archivo original no se utiliza en esta version de la solucion -- probablemente fue un residuo de una iteracion anterior que usaba `cmp::min`.

## Conclusion

Este problema demuestra como la tecnica de expansion desde el centro, tipicamente asociada con encontrar palindromos, puede fusionarse elegantemente con programacion dinamica para resolver un problema de optimizacion. En lugar de construir una tabla booleana completa de palindromos y luego optimizar los cortes por separado, ambas operaciones ocurren simultaneamente durante la expansion. El resultado es un algoritmo `O(N^2)` en tiempo con solo `O(N)` de espacio -- un salto significativo respecto al enfoque ingenuo que necesitaria una tabla 2D completa. La solucion es un recordatorio de que a veces la mejor forma de resolver un problema no es atacarlo de frente, sino mirarlo desde un angulo diferente.
