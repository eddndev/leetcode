---
title: "1895 Largest Magic Square - ES"
problemUrl: "https://leetcode.com/problems/largest-magic-square/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["array", "matrix", "prefix-sum"]
complexity:
  time: "O(m * n * min(m, n))"
  space: "O(m * n)"
---

# Largest Magic Square: Sumas Prefijas en Cuatro Direcciones

## El Problema
Dada una cuadricula `grid` de `m x n` enteros, encontrar la submatriz cuadrada mas grande que forme un cuadrado magico. Un cuadrado magico es un cuadrado donde la suma de cada fila, cada columna y ambas diagonales son iguales. Devolver la longitud del lado del cuadrado mas grande. Cada submatriz de 1x1 es trivialmente un cuadrado magico, asi que la respuesta es al menos 1.

El enfoque por fuerza bruta recalcularia la suma de cada fila, columna y diagonal desde cero para cada cuadrado candidato. Eso agregaria un factor extra de `k` por verificacion. Con sumas prefijas, podemos verificar cualquier candidato en tiempo constante.

## La Intuicion: Precalcular Todo

La idea central es que verificar si una submatriz es un cuadrado magico se reduce a comparar sumas: `k` filas, `k` columnas y 2 diagonales. Si precalculamos las sumas prefijas en las cuatro direcciones -- filas, columnas, diagonal principal y anti-diagonal -- entonces cada una de esas sumas se convierte en una consulta O(1).

Para filas y columnas, son sumas prefijas 1D directas. Para la diagonal principal (de arriba-izquierda a abajo-derecha), definimos `d1[r+1][c+1] = d1[r][c] + grid[r][c]`, de modo que la suma a lo largo de cualquier segmento diagonal de longitud `k` que comienza en `(r, c)` es `d1[r+k][c+k] - d1[r][c]`. La anti-diagonal (de arriba-derecha a abajo-izquierda) necesita un desplazamiento ligeramente diferente: `d2[r+1][c+1] = d2[r][c+2] + grid[r][c]`, que acumula valores moviendose hacia abajo-izquierda. La suma de la anti-diagonal de un cuadrado de `k x k` que comienza en `(r, c)` es `d2[r+k][c+1] - d2[r][c+k+1]`.

Una vez construidos los cuatro arreglos de prefijos, iteramos desde la longitud de lado mas grande posible hacia abajo hasta 2. Para cada tamano candidato `k`, probamos cada esquina superior izquierda `(r, c)` y verificamos si la submatriz es magica. En el momento en que encontramos una valida, devolvemos `k` inmediatamente, ya que estamos buscando de mayor a menor. Si ningun cuadrado de tamano 2 o mayor funciona, la respuesta es 1.

## La Verificacion

La funcion `is_magic` toma un cuadrado candidato en la posicion `(r, c)` con lado `k` y verifica:

1. **Filas** -- La suma de la primera fila se convierte en el objetivo. Cada fila siguiente debe coincidir.
2. **Columnas** -- Todas las `k` columnas deben sumar el objetivo.
3. **Diagonal principal** -- Debe ser igual al objetivo.
4. **Anti-diagonal** -- Debe ser igual al objetivo.

Cada verificacion es una sola resta en el arreglo de prefijos correspondiente. Cualquier discrepancia activa un retorno anticipado.

## Solucion en Rust

Los cuatro arreglos de prefijos `rows`, `cols`, `d1` y `d2` se asignan con relleno extra para evitar verificaciones de limites. El `#[inline(always)]` en `is_magic` incentiva al compilador a eliminar la sobrecarga de la llamada a funcion en el bucle interno. Iterar `k` en orden inverso con `.rev()` significa que devolvemos el primer resultado valido, que esta garantizado ser el mas grande.

```rust
impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut rows = vec![vec![0; n + 1]; m];
        let mut cols = vec![vec![0; m + 1]; n];
        let mut d1 = vec![vec![0; n + 2]; m + 2];
        let mut d2 = vec![vec![0; n + 2]; m + 2];

        for r in 0..m {
            for c in 0..n {
                let val = grid[r][c];
                rows[r][c + 1] = rows[r][c] + val;
                cols[c][r + 1] = cols[c][r] + val;
                d1[r + 1][c + 1] = d1[r][c] + val;
                d2[r + 1][c + 1] = d2[r][c + 2] + val;
            }
        }

        for k in (2..=m.min(n)).rev() {
            for r in 0..=(m - k) {
                for c in 0..=(n - k) {
                    if Self::is_magic(r, c, k, &grid, &rows, &cols, &d1, &d2) {
                        return k as i32;
                    }
                }
            }
        }

        1
    }

    #[inline(always)]
    fn is_magic(
        r: usize,
        c: usize,
        k: usize,
        grid: &Vec<Vec<i32>>,
        rows: &Vec<Vec<i32>>,
        cols: &Vec<Vec<i32>>,
        d1: &Vec<Vec<i32>>,
        d2: &Vec<Vec<i32>>,
    ) -> bool {
        let target = rows[r][c + k] - rows[r][c];

        for i in 1..k {
            if rows[r + i][c + k] - rows[r + i][c] != target {
                return false;
            }
        }

        for j in 0..k {
            if cols[c + j][r + k] - cols[c + j][r] != target {
                return false;
            }
        }

        if d1[r + k][c + k] - d1[r][c] != target {
            return false;
        }

        if d2[r + k][c + 1] - d2[r][c + k + 1] != target {
            return false;
        }

        true
    }
}
```

## Conclusion

La complejidad temporal es $O(m \times n \times \min(m, n))$: para cada longitud de lado candidata `k` (hasta `min(m, n)` valores), verificamos hasta $O(m \times n)$ posiciones, y cada verificacion toma tiempo O(k) por los bucles de filas y columnas, pero en el peor caso recorremos todas las posiciones para cada `k`. La complejidad espacial es $O(m \times n)$ por los cuatro arreglos de sumas prefijas. La conclusion clave es que las sumas prefijas en cuatro direcciones convierten lo que seria una suma costosa por elemento en consultas de rango en tiempo constante, y buscar de mayor a menor nos permite detenernos en cuanto encontramos un cuadrado magico valido.
