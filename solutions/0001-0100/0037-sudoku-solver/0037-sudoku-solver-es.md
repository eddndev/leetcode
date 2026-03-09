---
title: "0037 Sudoku Solver - ES"
problemUrl: "https://leetcode.com/problems/sudoku-solver/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["backtracking", "bitmask", "constraint-propagation"]
complexity:
  time: "O(9^M) donde M es el numero de celdas vacias"
  space: "O(M)"
---

# Resolviendo Sudoku con Bits y Backtracking

## El Problema
Dado un tablero de Sudoku parcialmente lleno de 9x9, llenarlo de forma que cada fila, cada columna y cada subcuadro de 3x3 contenga los digitos del 1 al 9 exactamente una vez. Se garantiza que el puzzle tiene exactamente una solucion.

## La Tentacion del Brute Force

Cuando uno se enfrenta a un Sudoku programaticamente, el primer impulso es simple: encontrar una celda vacia, probar los numeros del 1 al 9, verificar si es valido, y si no funciona, retroceder. El clasico backtracking. Y funciona, pero es dolorosamente lento. Cada verificacion de validez recorre filas, columnas y cajas, y el arbol de busqueda crece sin piedad.

Lo que necesitaba era una forma de saber instantaneamente que numeros son validos para una celda dada, sin recorrer nada. Y ahi es donde los **bitmasks** entran en juego.

## La Idea: Restricciones como Bits

Imagine que cada fila, columna y caja de 3x3 tiene una "mascara" de 9 bits. Cada bit representa si un digito (del 1 al 9) ya esta presente. Si el bit 0 esta encendido, el 1 ya fue usado. Si el bit 4 esta encendido, el 5 ya fue usado.

Con tres arreglos de mascaras (`rows`, `cols`, `boxes`), puedo calcular los candidatos validos para cualquier celda `(r, c)` con una sola operacion:

```
candidatos = NOT(rows[r] OR cols[c] OR boxes[b]) AND 0x1FF
```

El `OR` combina todas las restricciones, el `NOT` las invierte para obtener los numeros disponibles, y el `AND 0x1FF` asegura que solo nos quedemos con los 9 bits inferiores. En una sola linea, sin bucles, tengo exactamente los digitos que puedo colocar.

## La Heuristica: MRV (Minimum Remaining Values)

El backtracking puro elige la primera celda vacia que encuentra. Pero no todas las celdas vacias son iguales. Si una celda solo tiene un candidato posible, deberiamos llenarla primero: no hay decision que tomar, y reducimos el espacio de busqueda inmediatamente. Si una celda tiene cero candidatos, sabemos que llegamos a un callejon sin salida antes de desperdiciar mas trabajo.

Esta es la heuristica **MRV**: en cada paso, buscamos la celda vacia con menos candidatos. Si `count_ones(candidatos)` es 1, es una decision forzada. Si es 0, podamos la rama al instante. Este simple cambio transforma un backtracking exponencial en algo que, en la practica, resuelve cualquier Sudoku valido en microsegundos.

## Iterando sobre los Candidatos con Bit Tricks

Una vez que tengo la mascara de candidatos, necesito iterar sobre cada bit encendido. Aqui uso un truco clasico de manipulacion de bits:

```
bit = candidates & !(candidates - 1)  // Aisla el bit menos significativo
val = bit.trailing_zeros()              // Obtiene el indice del digito
candidates &= !bit                     // Apaga ese bit para la siguiente iteracion
```

Esto me permite recorrer solo los candidatos validos sin ningun bucle innecesario del 1 al 9. Cada iteracion extrae un candidato, lo prueba, y si el backtracking falla, lo deshace limpiamente con XOR.

## Solucion en Rust

```rust
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];
        let mut empty_count = 0;

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] != '.' {
                    let bit = 1 << (board[r][c] as u8 - b'1');
                    rows[r] |= bit;
                    cols[c] |= bit;
                    boxes[(r / 3) * 3 + (c / 3)] |= bit;
                } else {
                    empty_count += 1;
                }
            }
        }

        Self::backtrack(board, &mut rows, &mut cols, &mut boxes, empty_count);
    }

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        rows: &mut [u16; 9],
        cols: &mut [u16; 9],
        boxes: &mut [u16; 9],
        count: i32,
    ) -> bool {
        if count == 0 {
            return true;
        }

        let mut min_candidates = 10;
        let mut best_r = 0;
        let mut best_c = 0;
        let mut best_mask = 0;

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    let b = (r / 3) * 3 + (c / 3);
                    let mask = !(rows[r] | cols[c] | boxes[b]) & 0x1FF;
                    let candidates_count = mask.count_ones();

                    if candidates_count < min_candidates {
                        min_candidates = candidates_count;
                        best_r = r;
                        best_c = c;
                        best_mask = mask;
                        if min_candidates == 1 {
                            break;
                        }
                    }
                }
            }
            if min_candidates == 1 {
                break;
            }
        }

        if min_candidates == 0 {
            return false;
        }

        let r = best_r;
        let c = best_c;
        let b = (r / 3) * 3 + (c / 3);
        let mut candidates = best_mask;

        while candidates > 0 {
            let bit = candidates & !(candidates - 1);
            let val_idx = bit.trailing_zeros();

            board[r][c] = (val_idx as u8 + b'1') as char;
            rows[r] |= bit;
            cols[c] |= bit;
            boxes[b] |= bit;

            if Self::backtrack(board, rows, cols, boxes, count - 1) {
                return true;
            }

            rows[r] ^= bit;
            cols[c] ^= bit;
            boxes[b] ^= bit;
            board[r][c] = '.';

            candidates &= !bit;
        }

        false
    }
}
```

La fase de inicializacion recorre el tablero una sola vez para construir las tres mascaras y contar las celdas vacias. A partir de ahi, `backtrack` hace todo el trabajo pesado. Lo que mas me gusta de esta implementacion es que el estado se modifica in-place con operaciones de bits (OR para colocar, XOR para deshacer), sin necesidad de copiar el tablero ni crear estructuras auxiliares. Todo vive en tres arreglos de 9 enteros de 16 bits.

El uso de `count_ones()` y `trailing_zeros()` en Rust es particularmente elegante porque el compilador los traduce directamente a instrucciones de hardware (`POPCNT` y `TZCNT`), haciendo que estas operaciones sean literalmente de un solo ciclo de CPU.

## Conclusion

Este problema es un ejemplo perfecto de como las tecnicas de manipulacion de bits pueden transformar un algoritmo de backtracking ingenuo en algo extremadamente eficiente. Las mascaras de bits comprimen el estado de restricciones en enteros simples, la heuristica MRV poda el arbol de busqueda de forma agresiva, y los bit tricks permiten iterar sobre candidatos sin desperdicio. El resultado es un solver que, a pesar de tener una complejidad teorica exponencial, resuelve tableros de Sudoku en tiempo practicamente instantaneo.
