---
title: "0329 Longest Increasing Path in a Matrix - ES"
problemUrl: "https://leetcode.com/problems/longest-increasing-path-in-a-matrix/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "dfs", "memoization", "matrix", "graph"]
complexity:
  time: "O(M * N) donde M y N son las dimensiones de la matriz"
  space: "O(M * N)"
---

# Rios Tallando la Cuadricula

## El Problema
Dada una matriz de enteros de `m x n`, devolver la longitud del camino creciente mas largo. Desde cada celda, se puede mover en cuatro direcciones: izquierda, derecha, arriba o abajo. No se permite moverse diagonalmente ni fuera de los limites. Cada paso debe ir a una celda con un valor estrictamente mayor.

## La Intuicion Inicial

A primera vista, esto parece un problema de grafos donde cada celda es un nodo y las aristas conectan a vecinos con valores estrictamente mayores. Necesito el camino mas largo en este grafo dirigido aciclico. La parte "aciclico" es crucial -- como cada paso debe ir a un valor estrictamente mayor, nunca puedo revisitar una celda, lo que significa que los ciclos son imposibles.

Esta estructura de DAG sugiere inmediatamente DFS con memorizacion. Si ya conozco el camino creciente mas largo que comienza desde alguna celda `(i, j)`, nunca deberia recalcularlo. La respuesta de cada celda depende unicamente de sus vecinos con valores mayores, y esas dependencias forman una estructura limpia tipo arbol sin dependencias circulares.

## Por Que la Memorizacion Funciona Perfectamente

La observacion clave es que la restriccion de "estrictamente creciente" nos da un orden topologico natural. Si `matrix[a][b] < matrix[i][j]`, entonces la respuesta para `(a, b)` nunca puede depender de la respuesta para `(i, j)`, porque no se puede ir de un valor mayor a uno menor. Esto significa que cuando calculo `dfs(i, j)`, todas las llamadas recursivas que hago son para celdas con valores estrictamente mayores, y sus resultados ya estan en cache o se calcularan sin nunca volver a `(i, j)`.

Sin memorizacion, el DFS por fuerza bruta revisitaria celdas exponencialmente muchas veces. Consideremos una cuadricula como:

```
1  2  3
6  5  4
7  8  9
```

El camino desde la celda `1` pasa por la celda `2`, que tambien se explora independientemente. La celda `4` se alcanza desde `3` y desde `5`. La superposicion se propaga en cascada. Pero con memorizacion, cada celda se calcula completamente exactamente una vez, dandonos un trabajo total de `O(M * N)`.

## La Estructura del DFS

Para cada celda `(i, j)`, exploro los cuatro vecinos. Si un vecino `(ni, nj)` esta dentro de los limites y tiene un valor estrictamente mayor, busco recursivamente el camino mas largo que comienza desde ese vecino y le sumo 1. La respuesta para `(i, j)` es el maximo entre todos los vecinos validos, con un caso base de 1 (solo la celda misma, cuando ningun vecino es mayor).

Almaceno los resultados en una matriz `cache` inicializada en ceros. Una entrada distinta de cero significa que la celda ya fue calculada. Este truco de doble proposito evita necesitar un arreglo separado de "visitados" -- como la respuesta de cada celda es al menos 1, un cero significa inequivocamente "aun no calculado".

## El Ciclo Exterior

El camino creciente mas largo podria comenzar desde cualquier celda, asi que lanzo DFS desde cada celda de la matriz, manteniendo el registro del maximo global. Gracias a la memorizacion, la mayoria de estas llamadas retornan instantaneamente del cache. El trabajo total entre todas las llamadas sigue siendo `O(M * N)` porque el cuerpo del DFS de cada celda se ejecuta exactamente una vez.

## Solucion en Rust

```rust
use std::cmp;

impl Solution {
    const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let m = matrix.len();
        let n = matrix[0].len();

        let mut cache = vec![vec![0; n]; m];
        let mut max_len = 0;

        for i in 0..m {
            for j in 0..n {
                max_len = cmp::max(max_len, Self::dfs(&matrix, &mut cache, i, j, m, n));
            }
        }

        max_len
    }

    fn dfs(
        matrix: &Vec<Vec<i32>>,
        cache: &mut Vec<Vec<i32>>,
        i: usize,
        j: usize,
        m: usize,
        n: usize,
    ) -> i32 {
        if cache[i][j] != 0 {
            return cache[i][j];
        }

        let mut current_max = 1;
        let current_val = matrix[i][j];

        for &(di, dj) in &Self::DIRS {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            if ni >= 0 && ni < m as isize && nj >= 0 && nj < n as isize {
                let ni = ni as usize;
                let nj = nj as usize;

                if matrix[ni][nj] > current_val {
                    current_max = cmp::max(current_max, 1 + Self::dfs(matrix, cache, ni, nj, m, n));
                }
            }
        }

        cache[i][j] = current_max;
        current_max
    }
}
```

La implementacion se divide limpiamente en dos funciones. `longest_increasing_path` maneja el ciclo exterior, iterando sobre cada celda y rastreando el maximo global. Inicializa el cache como una matriz llena de ceros y lo pasa de forma mutable al DFS. La funcion `dfs` es el motor principal: primero verifica el cache, luego explora las cuatro direcciones usando la constante `DIRS`. La verificacion de limites convierte los indices a `isize` para aritmetica segura con desplazamientos negativos, y luego los reconvierte a `usize` solo despues de confirmar que el vecino esta dentro de los limites. La variable `current_max` comienza en 1 (la celda misma) y crece a medida que se encuentran caminos mas largos a traves de vecinos validos. Una vez calculado, el resultado se almacena en el cache antes de retornar.

## Conclusion

Longest Increasing Path in a Matrix es un ejemplo de libro de texto de como una restriccion de "estrictamente creciente" transforma lo que podria ser un problema NP-hard de camino mas largo en un elegante DFS con memorizacion. La restriccion estrictamente creciente garantiza un DAG, que a su vez garantiza que la memorizacion es segura y completa. Cada celda se calcula exactamente una vez, dando tiempo lineal en el tamanio de la matriz. La solucion en Rust refleja esta simplicidad: un arreglo de direcciones, una matriz de cache y una funcion recursiva que se lee como una traduccion directa de la relacion de recurrencia.
