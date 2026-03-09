---
title: "0218 The Skyline Problem - ES"
problemUrl: "https://leetcode.com/problems/the-skyline-problem/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["heap", "sweep-line", "sorting", "divide-and-conquer"]
complexity:
  time: "O(N log N) donde N es el numero de edificios"
  space: "O(N) para el heap y los puntos criticos"
---

# Dibujando Siluetas Contra el Horizonte

## El Problema
Dada una lista de edificios representados como `[left, right, height]`, devolver la linea del horizonte (skyline) formada por estos edificios como una lista de puntos clave `[x, height]`. Cada punto clave marca donde la linea del horizonte cambia de altura. La salida no debe contener entradas consecutivas con la misma altura.

## El Desafio de las Torres Superpuestas

A primera vista, este problema parece una simple cuestion de rastrear cual edificio es el mas alto en cada posicion. Pero los edificios se superponen de formas complejas -- un edificio mas bajo podria estar completamente oculto detras de uno mas alto, o dos edificios de diferentes alturas podrian compartir el mismo borde izquierdo. La dificultad reside en determinar eficientemente *cuando cambia la altura maxima* mientras barremos el eje x.

Un enfoque de fuerza bruta verificaria la altura en cada coordenada x entera, calculando la altura maxima entre todos los edificios activos en cada punto. Pero con coordenadas potencialmente en las decenas de miles, esto es un desperdicio. La mayoria de esas coordenadas x no producen ningun cambio en la linea del horizonte. El skyline solo puede cambiar en los bordes de los edificios -- en sus limites izquierdo y derecho.

## La Estrategia: Linea de Barrido con Max-Heap

### Identificando Puntos Criticos

Mi observacion clave fue que la linea del horizonte solo puede cambiar en coordenadas x donde un edificio comienza o termina. Asi que extraigo todas las coordenadas x unicas de los bordes de los edificios, las ordeno, y las proceso de izquierda a derecha. Estas son las unicas posiciones donde podria ocurrir una transicion de altura.

### El Max-Heap como Rastreador de Alturas

Mientras barro de izquierda a derecha, necesito saber cual es el edificio activo mas alto en cada punto critico. Un **max-heap** (cola de prioridad) es la opcion natural: me da acceso instantaneo a la altura maxima actual.

Para cada coordenada x critica, hago dos cosas:

1. **Activar edificios**: cualquier edificio cuyo borde izquierdo sea igual a la x actual se inserta en el heap como `(height, right_edge)`. Como el `BinaryHeap` de Rust es un max-heap por defecto, almacenar la altura primero significa que el edificio mas alto siempre estara en la cima.

2. **Expirar edificios**: antes de leer la altura actual, saco del tope del heap cualquier edificio cuyo borde derecho sea igual o anterior a la coordenada x actual. Estos edificios han terminado y ya no deben contribuir al skyline.

Despues de estos dos pasos, la cima del heap (si no esta vacio) me da la altura actual del skyline. Si difiere de la ultima altura registrada, he encontrado un nuevo punto clave.

### Un Ejemplo Concreto

Con `buildings = [[2,9,10],[3,7,15],[5,12,12],[15,20,10],[19,24,8]]`:

```
Puntos criticos: [2, 3, 5, 7, 9, 12, 15, 19, 20, 24]

x=2: Activar (10,9). Heap: [(10,9)]. Altura=10 -> registrar [2,10]
x=3: Activar (15,7). Heap: [(15,7),(10,9)]. Altura=15 -> registrar [3,15]
x=5: Activar (12,12). Heap: [(15,7),(12,12),(10,9)]. Altura=15 -> sin cambio
x=7: Expirar (15,7). Heap: [(12,12),(10,9)]. Altura=12 -> registrar [7,12]
x=9: Expirar (10,9). Heap: [(12,12)]. Altura=12 -> sin cambio
x=12: Expirar (12,12). Heap: []. Altura=0 -> registrar [12,0]
x=15: Activar (10,20). Heap: [(10,20)]. Altura=10 -> registrar [15,10]
x=19: Activar (8,24). Heap: [(10,20),(8,24)]. Altura=10 -> sin cambio
x=20: Expirar (10,20). Heap: [(8,24)]. Altura=8 -> registrar [20,8]
x=24: Expirar (8,24). Heap: []. Altura=0 -> registrar [24,0]

Resultado: [[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]]
```

### Por Que Funciona la Eliminacion Perezosa

Un detalle sutil pero importante: cuando "expiro" edificios, solo saco del tope del heap. Los edificios que han terminado pero estan enterrados bajo uno mas alto permanecen en el heap. Esto no afecta la correccion -- nunca influiran en el resultado porque estan ocultos detras del edificio mas alto que tienen encima. Cuando ese edificio mas alto finalmente expire, esas entradas obsoletas estaran en el tope y seran eliminadas en ese momento. Esta eliminacion perezosa evita la necesidad de una estructura de datos mas compleja como un BST balanceado con soporte de eliminacion.

## Solucion en Rust

```rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn get_skyline(mut buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut points = Vec::with_capacity(buildings.len() * 2);
        for b in &buildings {
            points.push(b[0]);
            points.push(b[1]);
        }
        points.sort_unstable();
        points.dedup();

        buildings.sort_unstable_by_key(|b| b[0]);

        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut i = 0;
        let n = buildings.len();

        for &x in &points {
            while i < n && buildings[i][0] == x {
                heap.push((buildings[i][2], buildings[i][1]));
                i += 1;
            }

            while let Some(&(_, right)) = heap.peek() {
                if right <= x {
                    heap.pop();
                } else {
                    break;
                }
            }

            let curr_height = if let Some(&(h, _)) = heap.peek() {
                h
            } else {
                0
            };

            if result.is_empty() || result.last().unwrap()[1] != curr_height {
                result.push(vec![x, curr_height]);
            }
        }

        result
    }
}
```

La implementacion comienza recolectando todos los bordes de edificios en una lista deduplicada y ordenada de coordenadas x criticas. Los edificios se ordenan por su borde izquierdo para poder activarlos en orden con un simple indice `i` que solo avanza hacia adelante -- sin necesidad de busqueda binaria. El `BinaryHeap` almacena tuplas `(height, right_edge)`; como el heap de Rust esta ordenado por maximo de forma predeterminada, el edificio activo mas alto siempre es accesible via `peek()`. El bucle de eliminacion perezosa en cada punto critico solo retira entradas expiradas del tope, dejando las entradas expiradas enterradas para limpieza posterior. La comparacion final `result.last().unwrap()[1] != curr_height` asegura que nunca se emitan puntos clave consecutivos con la misma altura, satisfaciendo la restriccion de salida del problema. La pre-asignacion con `with_capacity` en el vector `points` evita realocaciones, y `sort_unstable` con `dedup` produce eficientemente el conjunto ordenado y unico de coordenadas criticas.

## Conclusion

El Skyline Problem es una aplicacion clasica del paradigma de linea de barrido. Al reconocer que el horizonte solo cambia en los limites de los edificios, reducimos un problema de coordenadas infinitas a un conjunto finito de eventos criticos. El max-heap proporciona insercion en O(log N) y acceso en O(1) a la altura maxima actual, mientras que la eliminacion perezosa mantiene la logica simple sin sacrificar la correccion. El resultado es una solucion limpia en O(N log N) que procesa cada borde de edificio exactamente una vez y deja que los edificios expirados caigan naturalmente.
