---
title: "0010 Regular Expression Matching - ES"
problemUrl: "https://leetcode.com/problems/regular-expression-matching/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "dynamic-programming", "recursion"]
complexity:
  time: "O(M * N)"
  space: "O(M * N)"
---

# Cuando los Patrones Cobran Vida

## El Problema
Dada una cadena `s` y un patrón `p`, implementar el matching de expresiones regulares con soporte para `'.'` (coincide con cualquier carácter individual) y `'*'` (coincide con cero o más del elemento precedente). El matching debe cubrir **toda** la cadena de entrada, no solo una parte.

## La Primera Impresión

Cuando vi este problema por primera vez, pensé que sería cuestión de recorrer ambas cadenas carácter por carácter. Pero entonces apareció el `'*'`, y todo cambió. El asterisco no actúa solo: depende del carácter que lo precede, y puede significar "ignórame por completo" o "repíteme tantas veces como necesites". Esa dualidad es lo que convierte un problema aparentemente simple en uno genuinamente difícil.

Mi primer instinto fue la recursión. Si el patrón tiene `a*`, puedo elegir no consumir ningún carácter (cero ocurrencias) o consumir uno y seguir intentando. Pero la recursión pura explota exponencialmente. Ahí es donde la **programación dinámica** entra en escena.

## Construyendo la Tabla DP

Mi enfoque fue construir una tabla `dp[i][j]` donde cada celda responde a la pregunta: "¿Los primeros `i` caracteres de `s` coinciden con los primeros `j` caracteres de `p`?"

### El caso base

La celda `dp[0][0]` es `true`: una cadena vacía coincide con un patrón vacío. Pero hay un detalle sutil: un patrón como `a*b*c*` también puede coincidir con una cadena vacía, porque cada `x*` puede representar cero ocurrencias. Por eso, necesitamos inicializar la primera fila recorriendo el patrón y propagando `dp[0][j-2]` cada vez que encontramos un `'*'`.

### Las transiciones

Para cada celda `dp[i][j]`, hay tres escenarios posibles:

1. **Match directo o con punto:** Si `p[j-1]` es igual a `s[i-1]` o es `'.'`, simplemente heredamos el resultado diagonal: `dp[i][j] = dp[i-1][j-1]`.

2. **Asterisco:** Aquí está la verdadera complejidad. El `'*'` nos da dos caminos:
   - **Cero ocurrencias:** Ignoramos los dos últimos caracteres del patrón (`x*`), así que miramos `dp[i][j-2]`.
   - **Una o más ocurrencias:** Si el carácter precedente al `*` coincide con `s[i-1]` (o es `'.'`), podemos "consumir" un carácter de `s` y quedarnos en la misma posición del patrón: `dp[i-1][j]`.

3. **No hay coincidencia:** La celda queda en `false`.

Lo elegante de este enfoque es que captura toda la complejidad del backtracking en una tabla bidimensional, evitando el trabajo redundante.

## Solución en C

```c
#include <stdbool.h>
#include <string.h>

bool isMatch(char *s, char *p) {
    int m = strlen(s);
    int n = strlen(p);

    // DP Table
    bool dp[m + 1][n + 1];

    for (int i = 0; i <= m; i++) {
        for (int j = 0; j <= n; j++) {
            dp[i][j] = false;
        }
    }

    // Initial State
    // Empty String VS Empty pattern is TRUE
    dp[0][0] = true;

    for (int j = 1; j <= n; j++) {
        if (p[j - 1] == '*') {
            dp[0][j] = dp[0][j - 2];
        }
    }

    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            if (p[j - 1] == '.' || p[j - 1] == s[i - 1]) {
                dp[i][j] = dp[i - 1][j - 1];
            }

            else if (p[j - 1] == '*') {
                bool zero_match = dp[i][j - 2];

                bool char_match = (p[j - 2] == s[i - 1] || p[j - 2] == '.');

                bool one_plus_match = char_match && dp[i - 1][j];

                dp[i][j] = zero_match || one_plus_match;
            }

            else {
                dp[i][j] = false;
            }
        }
    }

    return dp[m][n];
}
```

En C, la implementación es bastante directa. Declaramos la tabla DP en el stack con VLA (Variable Length Arrays), lo cual nos ahorra la complejidad de manejar memoria dinámica. Las variables `zero_match`, `char_match` y `one_plus_match` hacen que la lógica del asterisco sea fácil de seguir, en lugar de comprimir todo en una sola línea críptica.

## Solución en Rust

```rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let (m, n) = (s.len(), p.len());
        let mut dp = vec![vec![false; n + 1]; m + 1];

        dp[0][0] = true;

        for j in 2..=n {
            if p[j - 1] == b'*' {
                dp[0][j] = dp[0][j - 2];
            }
        }

        for i in 1..=m {
            for j in 1..=n {
                if p[j - 1] == b'*' {
                    dp[i][j] = dp[i][j - 2];
                    if p[j - 2] == b'.' || p[j - 2] == s[i - 1] {
                        dp[i][j] = dp[i][j] || dp[i - 1][j];
                    }
                } else if p[j - 1] == b'.' || p[j - 1] == s[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }

        dp[m][n]
    }
}
```

La versión en Rust es más compacta. Convertimos las cadenas a `&[u8]` con `as_bytes()` para poder comparar byte a byte de forma eficiente y evitar la complejidad de trabajar con caracteres Unicode donde no lo necesitamos. El loop de inicialización del patrón vacío arranca desde `j = 2` porque un `'*'` nunca puede aparecer en la posición 0 (siempre necesita un carácter precedente).

## Conclusión

Este problema es un clásico de programación dinámica que enseña algo fundamental: cuando un problema tiene decisiones que se ramifican (consumir o no consumir, coincidir cero o más veces), la DP nos permite explorar todas las ramas sin repetir trabajo. La tabla `dp` actúa como una memoria compartida de todas las subpreguntas que ya resolvimos, y la respuesta final simplemente está esperándonos en `dp[m][n]`.
