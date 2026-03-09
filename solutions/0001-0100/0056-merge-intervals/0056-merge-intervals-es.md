---
title: "0056 Merge Intervals - ES"
problemUrl: "https://leetcode.com/problems/merge-intervals/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["array", "sorting"]
complexity:
  time: "O(n log n)"
  space: "O(n)"
---

# Merge Intervals: Ordenar Primero, Preguntar Despues

## El Problema
Dado un arreglo de `intervals` donde `intervals[i] = [start_i, end_i]`, fusionar todos los intervalos que se solapan y devolver un arreglo de intervalos no solapados que cubran todos los intervalos de la entrada.

A primera vista parece un problema de grafos donde hay que encontrar componentes conectados. Pero el truco esta en que ordenar lo transforma en algo mucho mas simple.

## La Intuicion: Ordenar lo Cambia Todo

Si los intervalos estan desordenados, detectar solapamientos requiere comparar cada par, lo cual es $O(n^2)$. Pero una vez que los ordenamos por su valor de inicio, los intervalos que se solapan quedan adyacentes. Esa unica observacion reduce el problema a un recorrido lineal.

Despues de ordenar, iteramos por los intervalos y mantenemos un intervalo "actual" en ejecucion. Para cada nuevo intervalo, verificamos: se solapa con el actual? Si es asi (el fin del intervalo actual es mayor o igual al inicio del nuevo), extendemos el fin del intervalo actual para cubrir ambos. Si no, empujamos el intervalo actual al resultado y comenzamos uno nuevo.

El detalle clave es que al extender, tomamos el **maximo** de ambos finales, no simplemente el final del nuevo intervalo. Esto maneja el caso donde un intervalo esta completamente contenido dentro de otro, como `[1, 10]` y `[2, 5]`.

## Solucion en Rust

El `sort_unstable_by` de Rust es perfecto aqui ya que no necesitamos preservar el orden relativo de elementos iguales, y evita el overhead de un sort estable. El `match` sobre `result.last_mut()` es idiomatico: nos da una referencia mutable al ultimo elemento sin overhead de verificacion de limites en el camino feliz.

```rust
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut result: Vec<Vec<i32>> = Vec::new();

        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        for interval in intervals {
            match result.last_mut() {
                Some(last) if last[1] >= interval[0] => {
                    last[1] = last[1].max(interval[1]);
                }

                _ => {
                    result.push(interval);
                }
            }
        }

        result
    }
}
```

## Conclusion

El cuello de botella es el ordenamiento en $O(n \log n)$; el paso de fusion en si es solo $O(n)$. El espacio es $O(n)$ para la salida. Este es uno de esos problemas donde el preprocesamiento correcto (ordenar) hace que la logica real sea casi trivial. El patron de "ordenar, luego recorrido lineal con un estado en ejecucion" aparece sorprendentemente seguido en problemas de intervalos.
