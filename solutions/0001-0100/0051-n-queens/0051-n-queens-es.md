---
title: "0051 N-Queens - ES"
problemUrl: "https://leetcode.com/problems/n-queens/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["backtracking", "bitmask", "recursion"]
complexity:
  time: "O(N!)"
  space: "O(N)"
---

# Coronando Reinas con Bits

## El Problema
Dado un entero `n`, encontrar todas las formas de colocar `n` reinas en un tablero de ajedrez de `n x n` tal que ninguna reina ataque a otra. Dos reinas se atacan si comparten la misma fila, columna o diagonal. Devolver todas las configuraciones validas.

## El Backtracking Clasico y su Talon de Aquiles

El enfoque natural para N-Queens es colocar reinas fila por fila, y en cada fila probar cada columna. Antes de colocar una reina, verificamos que no haya conflicto con las reinas ya colocadas. Eso implica, en la implementacion ingenua, recorrer todas las reinas previas para cada candidato, verificando columnas y ambas diagonales.

Funciona, pero cada verificacion es lineal en el numero de reinas ya colocadas. Lo que yo queria era una forma de saber en tiempo constante si una posicion es segura. Y la respuesta, como en tantos problemas de backtracking, esta en los **bitmasks**.

## La Idea: Tres Mascaras, Cero Bucles de Verificacion

La clave es representar las restricciones con tres enteros: `cols`, `ld` (diagonal izquierda) y `rd` (diagonal derecha). Cada bit encendido indica una posicion atacada.

- `cols` registra que columnas ya tienen una reina.
- `ld` registra las diagonales superiores-izquierdas ocupadas. Cuando bajamos una fila, las amenazas de diagonal izquierda se desplazan un bit a la izquierda (`<< 1`).
- `rd` registra las diagonales superiores-derechas ocupadas. Al bajar una fila, se desplazan un bit a la derecha (`>> 1`).

Con estas tres mascaras, las posiciones disponibles en la fila actual se calculan con una sola operacion:

```
posibilidades = NOT(cols OR ld OR rd) AND limit
```

Donde `limit = (1 << n) - 1` es una mascara con los `n` bits inferiores encendidos, que nos asegura quedarnos solo dentro del tablero. Sin bucles, sin recorridos: una operacion bitwise y tengo exactamente donde puedo colocar la siguiente reina.

## Extrayendo Posiciones con Bit Tricks

Una vez que tengo la mascara de posibilidades, necesito iterar sobre cada bit encendido para probar cada posicion. Aqui uso el truco clasico de aislar el bit menos significativo:

```
bit = posibilidades & -posibilidades   // Aisla el bit mas bajo encendido
col_idx = bit.trailing_zeros()          // Obtiene el indice de la columna
posibilidades ^= bit                   // Apaga ese bit para la siguiente iteracion
```

Cada `bit` aislado representa una columna segura. Lo empujo al estado actual, hago la llamada recursiva con las mascaras actualizadas, y al volver simplemente lo saco. El backtracking es limpio y sin copias.

## Solucion en Rust

```rust
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut results = Vec::new();
        let limit = (1 << n) - 1;
        let mut current_board: Vec<usize> = Vec::with_capacity(n as usize);

        Self::backtrack(0, 0, 0, limit, n, &mut current_board, &mut results);

        results
    }

    fn backtrack(
        cols: i32,
        ld: i32,
        rd: i32,
        limit: i32,
        n: i32,
        current_board: &mut Vec<usize>,
        results: &mut Vec<Vec<String>>,
    ) {
        if cols == limit {
            results.push(Self::format_board(current_board, n));
            return;
        }

        let mut possibilities = !(cols | ld | rd) & limit;

        while possibilities > 0 {
            let bit = possibilities & -possibilities;

            let col_idx = bit.trailing_zeros() as usize;

            current_board.push(col_idx);

            Self::backtrack(
                cols | bit,
                (ld | bit) << 1,
                (rd | bit) >> 1,
                limit,
                n,
                current_board,
                results,
            );

            current_board.pop();

            possibilities ^= bit;
        }
    }

    fn format_board(indices: &Vec<usize>, n: i32) -> Vec<String> {
        let mut board = Vec::with_capacity(n as usize);
        for &col in indices {
            let mut row_str = String::with_capacity(n as usize);
            for i in 0..n {
                if i == col as i32 {
                    row_str.push('Q');
                } else {
                    row_str.push('.');
                }
            }
            board.push(row_str);
        }
        board
    }
}
```

Lo que mas me gusta de esta implementacion es la elegancia de la condicion base: `cols == limit`. Cuando todas las columnas estan ocupadas, sabemos que hemos colocado exactamente `n` reinas sin conflictos. No necesitamos un contador de filas separado; la mascara de columnas nos dice todo.

La propagacion de restricciones diagonales es la parte mas ingeniosa. Al hacer `(ld | bit) << 1` y `(rd | bit) >> 1`, las amenazas diagonales "caen" naturalmente a la siguiente fila. Cada nivel de recursion recibe las restricciones ya desplazadas, listas para usarse sin ningun calculo adicional.

El `current_board` almacena solo los indices de columna de cada reina, lo que hace que la reconstruccion del tablero en `format_board` sea trivial: para cada fila, la reina esta en el indice guardado y el resto son puntos. Todo el estado mutable se reduce a un vector de indices y tres enteros.

## Conclusion

El problema de las N-Queens es un clasico que parece pedir backtracking puro, pero la combinacion con bitmasks lo transforma por completo. Las tres mascaras eliminan toda verificacion de conflictos, el desplazamiento de bits propaga las restricciones diagonales de forma natural, y el truco de aislar el bit menos significativo nos da las posiciones disponibles una por una sin recorrer el tablero. El resultado es un algoritmo que, aunque sigue siendo exponencial en el peor caso, poda el espacio de busqueda de manera agresiva y ejecuta cada paso en tiempo constante.
