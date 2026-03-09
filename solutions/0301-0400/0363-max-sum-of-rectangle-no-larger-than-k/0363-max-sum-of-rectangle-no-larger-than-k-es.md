---
title: "0363 Max Sum of Rectangle No Larger Than K - ES"
problemUrl: "https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "prefix-sum", "ordered-set", "binary-search", "matrix"]
complexity:
  time: "O(M^2 * N * log N) donde M es el numero de filas y N es el numero de columnas"
  space: "O(N)"
---

# El Tesoro Acotado en la Cuadricula

## El Problema
Dada una matriz `m x n` llamada `matrix` y un entero `k`, devolver la suma maxima de un rectangulo en la matriz tal que dicha suma no sea mayor que `k`. Se garantiza que existira un rectangulo con suma no mayor que `k`.

## La Intuicion Inicial

Encontrar la suma maxima de un rectangulo en una matriz ya es un problema clasico, pero agregar la restriccion "no mayor que k" lo cambia todo. Sin la restriccion, el algoritmo de Kadane aplicado sobre columnas comprimidas lo resuelve limpiamente. Con la restriccion, no puedo simplemente rastrear el subarreglo maximo -- necesito encontrar la mejor suma de subarreglo que no exceda un limite dado.

Mi primer pensamiento es reducir el problema 2D a multiples problemas 1D. Si fijo dos limites de fila, la suma del rectangulo entre esas filas para cualquier rango de columnas se convierte en un problema de suma de subarreglo unidimensional sobre las sumas de columnas comprimidas. Esta es la tecnica estandar de compresion de filas: para cada par de limites de fila `(r1, r2)`, mantengo un arreglo de sumas por columna donde cada entrada acumula los valores de la matriz desde la fila `r1` hasta la fila `r2`.

## De 2D a 1D con Compresion de Columnas

Para una fila superior fija `r1`, itero la fila inferior `r2` hacia abajo. A medida que `r2` avanza, sumo incrementalmente `matrix[r2][c]` a `col_sums[c]` para cada columna `c`. Ahora `col_sums` representa las sumas verticales desde `r1` hasta `r2` para cada columna, y cualquier subarreglo contiguo de `col_sums` corresponde a un rectangulo en la matriz original.

El problema ahora se reduce a: dado un arreglo unidimensional `col_sums`, encontrar la suma maxima de subarreglo que sea como maximo `k`. Aqui es donde entra la combinacion clasica de sumas de prefijos con un conjunto ordenado.

## Sumas de Prefijos y Conjuntos Ordenados

Quiero el valor maximo de `prefix[j] - prefix[i]` donde `i < j` y este valor sea como maximo `k`. Reordenando, para cada `prefix[j]`, necesito el menor `prefix[i]` tal que `prefix[i] >= prefix[j] - k`. Esta es una consulta clasica de "limite inferior".

A medida que calculo la suma de prefijos acumulada, mantengo un `BTreeSet` con todas las sumas de prefijos vistas anteriormente. Para la suma de prefijo actual `current_prefix_sum`, consulto el conjunto buscando el menor valor mayor o igual a `current_prefix_sum - k`. Si tal valor `prev_sum` existe, entonces `current_prefix_sum - prev_sum` es una suma de subarreglo valida que no excede `k`, y actualizo el maximo global.

Insertar un valor semilla de `0` en el conjunto antes de procesar maneja el caso donde el prefijo completo desde el inicio constituye un rectangulo valido.

## La Optimizacion de Salida Temprana

Si en algun punto el maximo alcanza exactamente `k`, puedo retornar inmediatamente. Como `k` es el limite superior y estoy maximizando, ningun rectangulo futuro puede mejorar este resultado. Esta pequena optimizacion puede ahorrar computo significativo en ciertas entradas.

## Solucion en Rust

```rust
use std::cmp::max;
use std::collections::BTreeSet;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = matrix.len();
        if rows == 0 {
            return 0;
        }
        let cols = matrix[0].len();

        let mut max_sum = i32::MIN;

        for r1 in 0..rows {
            let mut col_sums = vec![0; cols];

            for r2 in r1..rows {
                for c in 0..cols {
                    col_sums[c] += matrix[r2][c];
                }

                let mut current_prefix_sum = 0;
                let mut set = BTreeSet::new();

                set.insert(0);

                for &val in &col_sums {
                    current_prefix_sum += val;

                    let target = current_prefix_sum - k;

                    if let Some(&prev_sum) = set.range(target..).next() {
                        max_sum = max(max_sum, current_prefix_sum - prev_sum);
                    }

                    set.insert(current_prefix_sum);
                }

                if max_sum == k {
                    return k;
                }
            }
        }

        max_sum
    }
}
```

El ciclo externo fija la fila superior `r1` y el ciclo interno extiende la fila inferior `r2`. Para cada par, `col_sums` acumula las sumas verticales de forma incremental. El ciclo mas interno calcula la suma de prefijos acumulada sobre el arreglo de columnas comprimido. El metodo `BTreeSet::range(target..)` de Rust devuelve un iterador comenzando desde el primer elemento mayor o igual a `target`, que es exactamente la consulta de limite inferior que necesito. Si tal elemento existe, la diferencia `current_prefix_sum - prev_sum` es una respuesta candidata. Despues de procesar todas las columnas para un par de filas dado, si `max_sum` ya alcanzo `k`, la funcion retorna tempranamente.

## Conclusion

Max Sum of Rectangle No Larger Than K combina elegantemente dos tecnicas poderosas: compresion de filas para reducir un problema 2D a 1D, y sumas de prefijos con un conjunto ordenado para encontrar eficientemente sumas de subarreglos acotadas. El `BTreeSet` en Rust proporciona inserciones y consultas de rango en O(log N), manteniendo el trabajo por par de filas en O(N log N). La complejidad total de O(M^2 * N * log N) es la mejor alcanzable para este problema sin recurrir a estructuras de datos mas exoticas. La salida temprana cuando la respuesta alcanza exactamente `k` es una optimizacion practica que rinde frutos cuando el limite es ajustado. Lo que comienza como un intimidante problema de optimizacion 2D con restriccion se convierte, mediante descomposicion sistematica, en una secuencia de operaciones bien comprendidas sobre conjuntos ordenados.
