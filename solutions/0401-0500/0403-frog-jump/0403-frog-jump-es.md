---
title: "0403 Frog Jump - ES"
problemUrl: "https://leetcode.com/problems/frog-jump/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "hash-map", "hash-set"]
complexity:
  time: "O(N^2) donde N es el numero de piedras"
  space: "O(N^2)"
---

# La Rana que Calcula Antes de Saltar

## El Problema
Una rana esta cruzando un rio. El rio esta dividido en unidades y en algunas de ellas hay piedras. La rana puede saltar sobre una piedra, pero no debe caer al agua. Dada una lista de posiciones de piedras ordenadas en orden ascendente, determinar si la rana puede cruzar el rio aterrizando en la ultima piedra. Inicialmente, la rana esta en la primera piedra y asume que el primer salto debe ser de 1 unidad. Si el ultimo salto de la rana fue de `k` unidades, su proximo salto debe ser de `k - 1`, `k` o `k + 1` unidades. La rana solo puede saltar hacia adelante.

## La Intuicion Inicial

A primera vista, parece un problema de busqueda sobre un grafo de estados, donde cada estado es una combinacion de piedra actual y tamanio del ultimo salto. Podria intentar una busqueda en profundidad o amplitud, pero la cantidad de estados posibles podria explotar. Lo que necesito es una forma de rastrear, para cada piedra, todos los tamanios de salto con los que la rana puede llegar a ella.

Mi primer instinto es usar programacion dinamica. Si para cada piedra almaceno el conjunto de posibles saltos que llevaron a la rana hasta ahi, puedo propagar hacia adelante: desde cada piedra, con cada salto `k` registrado, intento llegar a las posiciones `piedra + k - 1`, `piedra + k` y `piedra + k + 1`. Si alguna de esas posiciones corresponde a una piedra real, registro el nuevo salto en esa piedra destino.

## El HashMap como Columna Vertebral

La clave del enfoque es usar un `HashMap` donde cada piedra mapea a un `HashSet` de tamanios de salto. Inicializo el mapa insertando todas las piedras con conjuntos vacios. Luego coloco un salto de `0` en la primera piedra como semilla -- la rana comienza ahi sin haber saltado.

Este diseno tiene una ventaja crucial: verificar si una posicion destino es una piedra valida se reduce a una consulta O(1) en el mapa. No necesito buscar linealmente ni usar busqueda binaria sobre el arreglo de piedras.

## La Propagacion Hacia Adelante

Recorro las piedras en orden. Para cada piedra, clono su conjunto de saltos (necesario en Rust para evitar conflictos de prestamo) y para cada salto `k` en el conjunto, considero los tres posibles saltos siguientes: `k - 1`, `k` y `k + 1`. Solo propago si el paso es positivo -- un salto de `0` o negativo no tiene sentido porque la rana solo avanza.

Si la posicion destino `current_stone + step` existe como clave en el mapa, inserto `step` en el conjunto de esa piedra destino. Al final, la rana puede cruzar el rio si y solo si el conjunto de saltos de la ultima piedra no esta vacio, lo que significa que al menos un camino valido la alcanzo.

## Por Que Funciona la Complejidad

Cada piedra puede tener a lo sumo O(N) saltos diferentes en su conjunto, ya que los saltos crecen de forma controlada. Para cada piedra, itero sobre sus saltos y hago tres operaciones de busqueda e insercion en el mapa. En el peor caso, el total de operaciones es O(N^2), lo cual es aceptable para las restricciones del problema donde N puede ser hasta 2000.

## Solucion en Rust

```rust
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();

        let mut dp: HashMap<i32, HashSet<i32>> = HashMap::new();

        for &stone in &stones {
            dp.insert(stone, HashSet::new());
        }

        if let Some(start_set) = dp.get_mut(&stones[0]) {
            start_set.insert(0);
        }

        for i in 0..n {
            let current_stone = stones[i];

            let jumps = dp[&current_stone].clone();

            for &k in &jumps {
                for step in k - 1..=k + 1 {
                    if step > 0 {
                        let next_pos = current_stone + step;

                        if let Some(next_set) = dp.get_mut(&next_pos) {
                            next_set.insert(step);
                        }
                    }
                }
            }
        }

        !dp[&stones[n - 1]].is_empty()
    }
}
```

El mapa `dp` se construye primero insertando todas las piedras como claves con conjuntos vacios, garantizando que las consultas de existencia sean directas. La semilla `0` en la primera piedra permite que el primer salto genere los pasos `k - 1 = -1` (descartado por la condicion `step > 0`), `k = 0` (tambien descartado) y `k + 1 = 1` (el unico salto valido inicial). El `.clone()` del conjunto de saltos es necesario porque Rust no permite tener simultaneamente una referencia inmutable para iterar y una mutable para insertar en el mismo mapa. Cada insercion exitosa en `next_set` indica que la rana puede alcanzar esa piedra destino con ese tamanio de paso particular. La verificacion final comprueba si la ultima piedra fue alcanzada por algun camino.

## Conclusion

Frog Jump es un ejemplo elegante de como la programacion dinamica con propagacion hacia adelante puede resolver problemas de alcanzabilidad con restricciones sobre transiciones. La combinacion de `HashMap` y `HashSet` en Rust proporciona las operaciones de busqueda e insercion en tiempo amortizado O(1), manteniendo la solucion dentro de O(N^2) tanto en tiempo como en espacio. Lo que podria parecer un problema de busqueda exponencial se domestica al observar que lo unico que importa en cada piedra no es como llegamos, sino con que saltos llegamos -- y esa informacion compacta es todo lo necesario para decidir hacia donde puede ir la rana a continuacion.
