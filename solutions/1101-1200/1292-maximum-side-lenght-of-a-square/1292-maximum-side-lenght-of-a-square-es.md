---
title: "1292 Maximum Side Length of a Square - ES"
problemUrl: "https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["array", "binary-search", "matrix", "prefix-sum"]
complexity:
  time: "O(m * n)"
  space: "O(m * n)"
---

# Maximum Side Length of a Square: Sumas Prefijas y Crecimiento Incremental

## El Problema
Dada una matriz `mat` de `m x n` y un entero `threshold`, necesitamos encontrar la longitud maxima del lado de una submatriz cuadrada cuya suma de elementos sea menor o igual a `threshold`. Si no existe tal cuadrado, devolvemos 0.

Por ejemplo, dada una matriz 3x3 y un umbral de 4, debemos verificar cada posible submatriz cuadrada y encontrar la mas grande cuya suma total no exceda 4.

A primera vista, parece que podria involucrar una verificacion por fuerza bruta de cada cuadrado posible en cada posicion, pero eso seria demasiado lento para matrices grandes.

## La Intuicion: Sumas de Submatrices Rapidas
Cuando vi este problema por primera vez, inmediatamente pense en **sumas prefijas 2D**. Esta es la tecnica clasica para calcular la suma de cualquier subregion rectangular de una matriz en tiempo O(1), una vez que se ha hecho el preprocesamiento en O(m * n).

La suma prefija `dp[i][j]` almacena la suma de todos los elementos en el rectangulo desde `(0, 0)` hasta `(i-1, j-1)`. Usando inclusion-exclusion, la suma de cualquier rectangulo se calcula como:

```
sum(r1, c1, r2, c2) = dp[r2][c2] - dp[r1][c2] - dp[r2][c1] + dp[r1][c1]
```

Asi que verificar si un cuadrado dado cabe dentro del umbral se convierte en una operacion de tiempo constante. La pregunta entonces es: como encontrar la longitud maxima del lado de forma eficiente?

## La Clave: Crecimiento Incremental en Lugar de Busqueda Binaria
El enfoque tipico seria construir la tabla de sumas prefijas y luego hacer busqueda binaria sobre la longitud del lado. Pero hay una observacion mas elegante: mientras recorremos la matriz de arriba-izquierda a abajo-derecha, solo necesitamos verificar si la mejor respuesta actual puede incrementarse en 1.

Pensemoslo asi: si ya encontramos un cuadrado valido de lado `k`, la siguiente pregunta interesante es si existe un cuadrado de lado `k + 1`. No necesitamos volver a verificar tamanos mas pequenos. Cuando visitamos cada celda `(i, j)`, calculamos la suma prefija y luego verificamos si un cuadrado de lado `max_len + 1` que termina en `(i, j)` tiene una suma dentro del umbral. Si la tiene, incrementamos `max_len`.

Esto funciona porque si existe un cuadrado valido de lado `k + 1` en cualquier lugar de la matriz, encontraremos su esquina inferior derecha durante nuestro recorrido, y en ese punto `max_len` sera al menos `k` (ya que el cuadrado de lado `k` ya fue encontrado), asi que probaremos y aceptaremos `k + 1`.

## El Algoritmo
1. Crear una tabla de sumas prefijas `dp` de tamano `(m+1) x (n+1)`, inicializada en cero.
2. Inicializar `max_len = 0`.
3. Para cada celda `(i, j)` desde `(1, 1)` hasta `(m, n)`:
   - Calcular `dp[i][j] = mat[i-1][j-1] + dp[i-1][j] + dp[i][j-1] - dp[i-1][j-1]`.
   - Sea `current_len = max_len + 1`.
   - Si `i >= current_len` y `j >= current_len` (el cuadrado cabe), calcular la suma del cuadrado de lado `current_len` que termina en `(i, j)`.
   - Si esa suma esta dentro del umbral, incrementar `max_len`.
4. Devolver `max_len`.

La belleza de este enfoque es que nunca verificamos mas de un candidato de longitud de lado por celda, por lo que todo el algoritmo se ejecuta en tiempo O(m * n).

### Implementacion en Rust

```rust
impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let mut dp = vec![vec![0; n + 1]; m + 1];

        let mut max_len = 0;

        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = mat[i - 1][j - 1] + dp[i - 1][j] + dp[i][j - 1] - dp[i - 1][j - 1];

                let current_len = max_len + 1;

                if i >= current_len && j >= current_len {
                    let sum = dp[i][j] - dp[i - current_len][j] - dp[i][j - current_len]
                        + dp[i - current_len][j - current_len];

                    if sum <= threshold {
                        max_len += 1;
                    }
                }
            }
        }

        max_len as i32
    }
}
```

## Conclusion
Este problema es un gran ejemplo de como las sumas prefijas pueden transformar una busqueda aparentemente costosa en un recorrido lineal. La observacion clave es que no necesitamos hacer busqueda binaria sobre la longitud del lado en absoluto. Al solo preguntarnos "puedo mejorar en uno?", reducimos el problema a una sola pasada por la matriz. Es una de esas soluciones que se sienten casi demasiado simples una vez que las ves, pero llegar ahi requiere reconocer que la respuesta solo puede crecer de uno en uno.
