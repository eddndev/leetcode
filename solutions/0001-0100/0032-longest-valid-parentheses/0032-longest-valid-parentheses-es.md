---
title: "0032 Longest Valid Parentheses - ES"
problemUrl: "https://leetcode.com/problems/longest-valid-parentheses/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "stack", "dynamic-programming"]
complexity:
  time: "O(N)"
  space: "O(N)"
---

# El Arte de Cerrar lo que se Abre

## El Problema
Dada una cadena que contiene solo los caracteres `'('` y `')'`, encontrar la longitud de la subcadena de parentesis validos mas larga. Una subcadena valida es aquella donde cada parentesis de apertura tiene su correspondiente parentesis de cierre, en el orden correcto.

## El Engano de la Simplicidad

Cuando vi este problema por primera vez, pense que bastaria con contar parentesis abiertos y cerrados. Si coinciden, tenemos una subcadena valida. Pero rapidamente me di cuenta de que eso no es suficiente. Consideremos `"())()"`: tiene la misma cantidad de `(` y `)`, pero no es una secuencia valida completa. El orden importa, y lo que realmente necesitamos es encontrar el **segmento continuo mas largo** donde los parentesis estan perfectamente balanceados.

Mi primer intento fue con programacion dinamica, pero luego pense en una solucion mas elegante: usar un **stack**. La idea es que el stack no solo rastrea los parentesis abiertos, sino que tambien nos da un punto de referencia para calcular longitudes.

## La Estrategia del Stack con Centinela

El truco clave es inicializar el stack con `-1`. Este valor actua como un **centinela**: representa la posicion justo antes del inicio de cualquier subcadena valida. Sin el, tendriamos que manejar un monton de casos especiales.

Al recorrer la cadena:

1. **Si encontramos `(`:** Simplemente empujamos su indice al stack. Es una promesa de apertura que esperamos cerrar mas adelante.

2. **Si encontramos `)`:** Primero sacamos el tope del stack (el `pop`). Despues de hacer pop:
   - **Si el stack queda vacio:** No habia un `(` correspondiente. Este `)` se convierte en el nuevo limite, asi que empujamos su indice como el nuevo centinela.
   - **Si el stack no esta vacio:** Tenemos un match. La longitud de la subcadena valida actual es `i - stack.top()`. Actualizamos el maximo si es necesario.

Lo brillante de este enfoque es que el tope del stack siempre representa **la posicion justo antes del inicio de la subcadena valida actual**. Eso hace que calcular la longitud sea una simple resta.

### Un Ejemplo paso a paso

Para `"(()":
- Iniciamos con stack = `[-1]`
- `i=0`, `(`: stack = `[-1, 0]`
- `i=1`, `(`: stack = `[-1, 0, 1]`
- `i=2`, `)`: pop `1`, stack = `[-1, 0]`, longitud = `2 - 0 = 2`
- Resultado: `2`

Para `")()())`:
- Iniciamos con stack = `[-1]`
- `i=0`, `)`: pop `-1`, stack vacio, push `0`. Stack = `[0]`
- `i=1`, `(`: stack = `[0, 1]`
- `i=2`, `)`: pop `1`, stack = `[0]`, longitud = `2 - 0 = 2`
- `i=3`, `(`: stack = `[0, 3]`
- `i=4`, `)`: pop `3`, stack = `[0]`, longitud = `4 - 0 = 4`
- `i=5`, `)`: pop `0`, stack vacio, push `5`. Stack = `[5]`
- Resultado: `4`

## Solucion en Rust

```rust
use std::cmp;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }

        let mut stack: Vec<i32> = Vec::with_capacity(n + 1);

        stack.push(-1);

        let mut max_len = 0;

        for (i, &byte) in s.as_bytes().iter().enumerate() {
            if byte == b'(' {
                stack.push(i as i32);
            } else {
                stack.pop();

                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    let current_len = (i as i32) - stack.last().unwrap();
                    max_len = cmp::max(max_len, current_len);
                }
            }
        }

        max_len
    }
}
```

En la implementacion en Rust, convertimos la cadena a bytes con `as_bytes()` para comparar directamente contra `b'('` y `b')'`, evitando la sobrecarga de trabajar con caracteres Unicode. El stack se inicializa con capacidad `n + 1` para evitar realocaciones innecesarias. Usamos `i32` en lugar de `usize` para poder almacenar el centinela `-1` sin complicaciones, y `stack.last().unwrap()` nos da el tope sin consumirlo, lo cual es exactamente lo que necesitamos despues del `pop`.

## Conclusion

Este problema es un ejemplo perfecto de como un stack puede resolver problemas que parecen requerir programacion dinamica. El centinela `-1` es la clave de todo: elimina los casos especiales y convierte el calculo de longitudes en una operacion trivial. En lugar de rastrear intervalos o construir tablas, dejamos que el stack haga todo el trabajo de contabilidad. A veces la estructura de datos correcta es todo lo que necesitas para que una solucion O(N) caiga por su propio peso.
