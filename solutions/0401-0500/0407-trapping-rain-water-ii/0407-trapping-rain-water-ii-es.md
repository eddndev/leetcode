---
title: "0407 Trapping Rain Water II - ES"
problemUrl: "https://leetcode.com/problems/trapping-rain-water-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["heap", "breadth-first-search", "matrix"]
complexity:
  time: "O(M * N * log(M * N)) donde M y N son las dimensiones de la matriz"
  space: "O(M * N)"
---

# Inundando el Terreno desde los Bordes

## El Problema
Dada una matriz `heightMap` de `m x n` enteros no negativos que representan la altura de cada celda en un mapa de elevacion, calcular el volumen de agua que puede quedar atrapado despues de llover.

## La Intuicion Inicial

Este problema es la extension tridimensional del clasico Trapping Rain Water. En la version 1D, la idea era sencilla: para cada barra, el agua que retiene depende de la barra mas alta a su izquierda y a su derecha. Pero en dos dimensiones, el agua en una celda no depende solo de dos direcciones, sino de todo el contorno que la rodea. Una celda puede "escapar" el agua por cualquier camino hacia el borde de la matriz.

Mi primera reaccion fue intentar generalizar los dos punteros, pero rapidamente me di cuenta de que eso no escala a 2D. La pregunta correcta no es "cual es el maximo a la izquierda y a la derecha", sino "cual es el camino mas bajo desde esta celda hasta el borde". Y eso me llevo a pensar en el problema desde afuera hacia adentro.

## Pensando como el Agua

El agua siempre escapa por el punto mas bajo del borde. Si imagino que el terreno es una piscina irregular, el nivel del agua esta determinado por la pared mas baja. Entonces, en lugar de preguntar "cuanta agua cabe aqui?" para cada celda, debo preguntar "que tan alto puede subir el agua antes de derramarse?".

Esto sugiere un enfoque voraz: empezar por los bordes, que son las celdas que no pueden retener agua, y expandirse hacia adentro procesando siempre la celda de menor altura primero. Si una celda interior es mas baja que el nivel actual del borde que la contiene, la diferencia es agua atrapada.

## El Min-Heap como Frontera

La estructura ideal para este enfoque es un min-heap (cola de prioridad minima). Inicio insertando todas las celdas del borde en el heap, marcandolas como visitadas. Estas celdas forman la "pared" inicial de la piscina.

En cada iteracion, extraigo la celda con menor altura del heap. Esta celda representa el punto mas debil del contorno actual. Luego examino sus cuatro vecinos no visitados. Para cada vecino:

1. Si su altura es **menor** que la de la celda actual, la diferencia es agua que queda atrapada ahi. Ademas, inserto el vecino en el heap con la altura de la celda actual (no la suya), porque el agua lo eleva a ese nivel y ahora el es parte del contorno con esa altura efectiva.

2. Si su altura es **mayor o igual**, no se atrapa agua, y el vecino entra al heap con su propia altura.

En ambos casos, la altura que inserto en el heap es `max(altura_actual, altura_vecino)`, lo que captura elegantemente la logica: el nivel efectivo de contorno nunca baja, solo sube.

## Por Que el Min-Heap Garantiza Correctitud

La clave es que al procesar siempre la celda de menor altura primero, simulo como el agua llenaria el terreno en la realidad: primero desborda por los puntos bajos. Cuando proceso una celda del heap, tengo la garantia de que su altura representa el nivel de agua mas bajo posible que puede alcanzar esa region. Cualquier celda interior que descubra despues tendra un contorno al menos tan alto, porque el heap ya consumio todos los puntos mas bajos.

Esto es esencialmente una busqueda BFS por prioridad, similar a Dijkstra pero sobre alturas de terreno.

## El Caso Base

Si la matriz tiene menos de 3 filas o menos de 3 columnas, es imposible atrapar agua: todas las celdas son parte del borde o estan adyacentes al borde sin espacio para una "piscina". La guarda `m < 3 || n < 3` al inicio maneja este caso trivial.

## Solucion en Rust

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();

        if m < 3 || n < 3 {
            return 0;
        }

        let mut heap = BinaryHeap::new();
        let mut visited = vec![vec![false; n]; m];

        for r in 0..m {
            for c in 0..n {
                if r == 0 || r == m - 1 || c == 0 || c == n - 1 {
                    heap.push(Reverse((height_map[r][c], r, c)));
                    visited[r][c] = true;
                }
            }
        }

        let mut total_water = 0;

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some(Reverse((h, r, c))) = heap.pop() {
            for (dr, dc) in directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < m as isize && nc >= 0 && nc < n as isize {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if !visited[nr][nc] {
                        visited[nr][nc] = true;

                        let neighbor_height = height_map[nr][nc];

                        if neighbor_height < h {
                            total_water += h - neighbor_height;
                        }

                        heap.push(Reverse((h.max(neighbor_height), nr, nc)));
                    }
                }
            }
        }

        total_water
    }
}
```

El `BinaryHeap` de Rust es un max-heap por defecto, asi que envuelvo las tuplas en `Reverse` para convertirlo en un min-heap. La tupla `(altura, fila, columna)` se compara lexicograficamente, asegurando que siempre proceso primero la celda mas baja. La matriz `visited` evita procesar una celda mas de una vez, lo cual es esencial tanto para la correctitud como para la eficiencia. La expresion `h.max(neighbor_height)` al insertar en el heap es el corazon del algoritmo: propaga el nivel efectivo del agua hacia el interior, garantizando que cada celda nueva entra al contorno con la altura correcta. Las direcciones se definen como `isize` para manejar limpiamente la aritmetica con signo al verificar limites, y los casts a `usize` solo ocurren despues de confirmar que los indices son validos.

## Conclusion

Trapping Rain Water II transforma un problema intuitivamente complejo -- calcular volumen de agua en un terreno 3D -- en una exploracion elegante desde los bordes hacia el interior. El min-heap actua como una frontera ordenada que simula el comportamiento fisico del agua: siempre desborda primero por el punto mas bajo. La generalizacion de 1D a 2D no fue trivial, pero la idea central es la misma: el agua atrapada en cualquier punto esta determinada por el obstaculo mas bajo entre ese punto y la libertad. En 1D ese obstaculo se encuentra con dos punteros; en 2D, con un heap que erosiona el contorno capa por capa.
