---
title: "0041 First Missing Positive - ES"
problemUrl: "https://leetcode.com/problems/first-missing-positive/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["array", "cyclic-sort", "in-place"]
complexity:
  time: "O(N)"
  space: "O(1)"
---

# El Numero que Nadie Invito

## El Problema
Dado un arreglo de enteros sin ordenar `nums`, encontrar el menor entero positivo que no esta presente en el arreglo. El algoritmo debe ejecutarse en tiempo O(N) y usar espacio constante O(1).

## La Trampa de las Soluciones Obvias

Cuando uno lee "encontrar el menor positivo faltante", la primera idea es ordenar el arreglo y recorrerlo buscando el primer hueco. Funciona, pero ordenar cuesta O(N log N), y el problema pide O(N). La segunda idea es usar un HashSet: recorrer el arreglo, meter todo en el set, y luego iterar desde 1 hasta encontrar el ausente. Funciona en O(N), pero el set consume O(N) de espacio extra, y el problema exige espacio constante.

La restriccion de O(1) en espacio es lo que convierte este problema en Hard. No puedo crear estructuras auxiliares. Tengo que trabajar con lo que ya tengo: el propio arreglo.

## La Observacion Clave

Hay una observacion fundamental que desbloquea todo: si el arreglo tiene `n` elementos, la respuesta siempre esta en el rango `[1, n+1]`. En el mejor caso, el arreglo contiene exactamente los numeros `1, 2, 3, ..., n`, y la respuesta es `n+1`. En cualquier otro caso, algun numero en `[1, n]` esta ausente.

Esto significa que los numeros negativos, los ceros y los numeros mayores que `n` son irrelevantes. Puedo ignorarlos por completo. Y si solo me importan los numeros del 1 al `n`, puedo usar el propio arreglo como una tabla hash improvisada: el numero `k` deberia estar en la posicion `k-1`.

## Cyclic Sort: Cada Numero a su Casa

La tecnica se llama **cyclic sort**. La idea es recorrer el arreglo y, para cada elemento, enviarlo a la posicion donde "deberia" estar. El numero 1 va a `nums[0]`, el 3 va a `nums[2]`, el 5 va a `nums[4]`. Si un numero es negativo, cero, o mayor que `n`, simplemente lo ignoro y sigo adelante.

El truco esta en el `while`: para cada posicion `i`, no avanzo hasta que el elemento actual este en su lugar correcto o sea un valor que no me interesa. Cada swap coloca al menos un numero en su posicion definitiva, asi que el total de swaps a lo largo de todo el arreglo es como maximo `n`. Esto garantiza O(N) en total, aunque parezca que hay un bucle anidado.

Despues de reorganizar, hago una segunda pasada lineal: la primera posicion donde `nums[i] != i + 1` me da la respuesta. Si todas las posiciones estan correctas, la respuesta es `n + 1`.

## Solucion en Rust

```rust
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 && nums[(nums[i] - 1) as usize] != nums[i] {
                let target_index = (nums[i] - 1) as usize;
                nums.swap(i, target_index);
            }
        }

        for i in 0..n {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }

        (n + 1) as i32
    }
}
```

La condicion del `while` tiene tres partes que trabajan juntas como un filtro preciso. Primero, `nums[i] > 0` descarta negativos y ceros. Segundo, `nums[i] <= n as i32` descarta numeros fuera del rango util. Tercero, `nums[(nums[i] - 1) as usize] != nums[i]` verifica que el destino no tenga ya el valor correcto, lo cual previene ciclos infinitos cuando hay duplicados.

Lo que me gusta de esta implementacion es que `nums.swap(i, target_index)` hace el intercambio de forma segura y expresiva. Rust nos obliga a calcular `target_index` antes del swap porque no podemos tomar dos referencias mutables al mismo slice simultaneamente con indexado directo. El metodo `swap` maneja eso internamente de forma segura.

El casting entre `i32` y `usize` es inevitable porque LeetCode define la firma con `Vec<i32>`, pero los indices en Rust son `usize`. Es un poco verboso, pero no tiene costo en tiempo de ejecucion.

## Conclusion

Este problema es un ejemplo brillante de como la restriccion mas severa (espacio O(1)) es la que guia hacia la solucion mas elegante. En lugar de construir una estructura auxiliar, convertimos el propio arreglo en un mapa donde cada posicion responde la pregunta "¿el numero `i+1` existe?". El cyclic sort logra esta reorganizacion en tiempo lineal con una invariante simple: cada swap coloca un numero en su hogar definitivo. Al final, el primer residente ausente es nuestra respuesta.
