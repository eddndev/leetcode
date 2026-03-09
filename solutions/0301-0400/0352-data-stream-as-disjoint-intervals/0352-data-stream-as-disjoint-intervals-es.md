---
title: "0352 Data Stream as Disjoint Intervals - ES"
problemUrl: "https://leetcode.com/problems/data-stream-as-disjoint-intervals/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["binary-search-tree", "ordered-map", "design", "intervals"]
complexity:
  time: "O(log N) por addNum, O(N) por getIntervals, donde N es el numero de intervalos"
  space: "O(N)"
---

# Tejiendo Intervalos desde el Caos

## El Problema
Disenar una estructura de datos que reciba un flujo de enteros no negativos y los resuma como una lista de intervalos disjuntos. Implementar la clase `SummaryRanges` con dos operaciones: `addNum(value)` que agrega el entero `value` al flujo, y `getIntervals()` que devuelve un resumen de los enteros en el flujo como una lista de intervalos disjuntos `[start, end]`, ordenados por `start`.

## El Desafio del Flujo Continuo

A primera vista, uno podria pensar en mantener un conjunto de todos los numeros vistos y reconstruir los intervalos cada vez que se llama a `getIntervals`. Pero eso es desperdiciar trabajo: si ya tenemos los intervalos formados, agregar un nuevo numero solo deberia afectar localmente, quiza extendiendo un intervalo existente, fusionando dos intervalos vecinos, o creando uno nuevo. La pregunta es como hacer esas operaciones de forma eficiente.

## La Intuicion del BTreeMap

La estructura ideal para este problema es un mapa ordenado donde cada clave es el inicio de un intervalo y su valor es el final. Un `BTreeMap` en Rust nos da exactamente eso: busquedas, inserciones y eliminaciones en O(log N), y la capacidad de buscar el predecesor y sucesor de cualquier clave.

Cuando llega un nuevo valor, necesito responder tres preguntas:

1. **Ya esta contenido?** Si algun intervalo existente ya cubre este valor, no hay nada que hacer.
2. **Es adyacente al intervalo anterior?** Si el intervalo que termina justo en `value - 1` existe, puedo extenderlo.
3. **Es adyacente al intervalo siguiente?** Si hay un intervalo que comienza en `value + 1`, puedo absorberlo.

Si es adyacente a ambos lados, fusiono los dos intervalos en uno solo. Si solo toca un lado, extiendo ese intervalo. Si no toca ninguno, creo un intervalo nuevo `[value, value]`.

## Paso a Paso con un Ejemplo

Supongamos que agregamos los numeros `1, 3, 7, 2, 6`:

- **addNum(1)**: No hay intervalos. Creo `[1, 1]`. Estado: `{[1,1]}`.
- **addNum(3)**: No es adyacente a `[1,1]` (que termina en 1, y 1 != 3-1=2). Creo `[3, 3]`. Estado: `{[1,1], [3,3]}`.
- **addNum(7)**: No es adyacente a nada. Creo `[7, 7]`. Estado: `{[1,1], [3,3], [7,7]}`.
- **addNum(2)**: El predecesor es `[1,1]` que termina en 1 = 2-1, asi que `merge_left = true`. El sucesor es `[3,3]` que comienza en 3 = 2+1, asi que `merge_right = true`. Fusiono ambos: elimino `[3,3]` y actualizo `[1,1]` a `[1,3]`. Estado: `{[1,3], [7,7]}`.
- **addNum(6)**: El predecesor es `[1,3]` que termina en 3, y 3 != 6-1=5, asi que `merge_left = false`. El sucesor es `[7,7]` que comienza en 7 = 6+1, asi que `merge_right = true`. Elimino `[7,7]` e inserto `[6,7]`. Estado: `{[1,3], [6,7]}`.

Resultado de `getIntervals()`: `[[1,3], [6,7]]`.

## Solucion en Rust

```rust
use std::collections::BTreeMap;

struct SummaryRanges {
    intervals: BTreeMap<i32, i32>,
}

impl SummaryRanges {
    fn new() -> Self {
        SummaryRanges {
            intervals: BTreeMap::new(),
        }
    }

    fn add_num(&mut self, value: i32) {
        let prev = self
            .intervals
            .range(..=value)
            .next_back()
            .map(|(&start, &end)| (start, end));

        if let Some((_, end)) = prev {
            if end >= value {
                return;
            }
        }

        let merge_left = prev.map_or(false, |(_, end)| end == value - 1);

        let next = self
            .intervals
            .range((value + 1)..)
            .next()
            .map(|(&start, &end)| (start, end));
        let merge_right = next.map_or(false, |(start, _)| start == value + 1);

        match (merge_left, merge_right) {
            (true, true) => {
                let (prev_start, _) = prev.unwrap();
                let (next_start, next_end) = next.unwrap();

                self.intervals.remove(&next_start);
                self.intervals.insert(prev_start, next_end);
            }
            (true, false) => {
                let (prev_start, _) = prev.unwrap();
                self.intervals.insert(prev_start, value);
            }
            (false, true) => {
                let (next_start, next_end) = next.unwrap();
                self.intervals.remove(&next_start);
                self.intervals.insert(value, next_end);
            }
            (false, false) => {
                self.intervals.insert(value, value);
            }
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals
            .iter()
            .map(|(&start, &end)| vec![start, end])
            .collect()
    }
}
```

La implementacion usa un `BTreeMap<i32, i32>` donde cada entrada `(start, end)` representa el intervalo `[start, end]`. Cuando llega un nuevo valor, `range(..=value).next_back()` encuentra el intervalo con la clave mas grande que no excede `value`, es decir, el candidato a predecesor. Si ese intervalo ya contiene `value` (su `end >= value`), retornamos inmediatamente. Luego determinamos si podemos fusionar a la izquierda (el predecesor termina en `value - 1`) y si podemos fusionar a la derecha (el sucesor comienza en `value + 1`). El `match` sobre las cuatro combinaciones posibles de `(merge_left, merge_right)` maneja cada caso: fusion doble, extension izquierda, extension derecha, o creacion de un intervalo nuevo. La operacion `get_intervals` simplemente itera el mapa ordenado, que ya mantiene los intervalos en el orden correcto.

## Conclusion

Data Stream as Disjoint Intervals es un problema de diseno que premia la eleccion correcta de estructura de datos. El `BTreeMap` nos da el equilibrio perfecto: busquedas logaritmicas para localizar los intervalos vecinos, y un orden natural que hace que la consulta de intervalos sea un simple recorrido. La logica de fusion en cuatro casos es el corazon del algoritmo, y una vez que se entiende que cada numero nuevo solo puede afectar a sus dos vecinos inmediatos en el mapa, la solucion fluye con elegancia. Es un recordatorio de que en problemas de flujos de datos, mantener la estructura actualizada incrementalmente es casi siempre mejor que reconstruirla desde cero.
