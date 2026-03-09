---
title: "0220 Contains Duplicate III - ES"
problemUrl: "https://leetcode.com/problems/contains-duplicate-iii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["sliding-window", "bucket-sort", "hash-map"]
complexity:
  time: "O(N) donde N es la longitud del arreglo"
  space: "O(min(N, indexDiff)) para el mapa de cubetas"
---

# Cazando Casi-Duplicados Con Cubetas Invisibles

## El Problema
Dado un arreglo de enteros `nums` y dos enteros `indexDiff` y `valueDiff`, devolver `true` si existen dos indices distintos `i` y `j` tales que `abs(i - j) <= indexDiff` y `abs(nums[i] - nums[j]) <= valueDiff`.

## La Trampa de la Fuerza Bruta

El enfoque ingenuo verifica cada par de elementos dentro de la ventana de indices -- para cada elemento, escanear hasta `indexDiff` posiciones hacia adelante y comparar valores. Esto corre en O(N * indexDiff) tiempo, que puede degradarse a O(N^2) cuando `indexDiff` es grande. Una estructura ordenada como un `BTreeSet` puede mejorar esto a O(N log(indexDiff)) manteniendo una ventana deslizante de elementos ordenados y realizando consultas de rango. Pero, podemos hacerlo mejor?

El verdadero desafio es que necesitamos satisfacer simultaneamente dos restricciones: los indices deben ser cercanos *y* los valores deben ser cercanos. La restriccion de indices sugiere naturalmente una ventana deslizante, pero la restriccion de valores requiere algo mas ingenioso que la comparacion por fuerza bruta dentro de esa ventana.

## La Estrategia: Ordenamiento por Cubetas en una Ventana Deslizante

### La Intuicion de las Cubetas

Mi idea clave fue tomar prestado del ordenamiento por cubetas. Si divido la recta numerica en cubetas de ancho `valueDiff + 1`, entonces dos numeros que caen en la *misma cubeta* tienen garantizado diferir por a lo sumo `valueDiff`. Dos numeros en *cubetas adyacentes* tambien podrian satisfacer la condicion, pero solo necesito verificar esos dos vecinos -- no la ventana entera.

Por ejemplo, con `valueDiff = 3`, creo cubetas de ancho 4: la cubeta 0 contiene valores [0, 3], la cubeta 1 contiene [4, 7], la cubeta 2 contiene [8, 11], y asi sucesivamente. Si dos valores caen en la misma cubeta, su diferencia es a lo sumo 3. Si caen en cubetas adyacentes, calculo la diferencia real y verifico.

### Manejo de Negativos

Los numeros negativos complican la asignacion de cubetas. Una division ingenua `val / w` no produce cubetas uniformemente espaciadas para negativos porque la division entera redondea hacia cero. Mi solucion desplaza los valores negativos: para un `val` negativo, el id de cubeta es `(val + 1) / w - 1`. Esto desplaza las cubetas de modo que [-4, -1] se mapea a la cubeta -1, [-8, -5] se mapea a la cubeta -2, y asi sucesivamente -- cada cubeta cubre exactamente `w` enteros consecutivos.

### La Ventana Deslizante

Para hacer cumplir la restriccion de indices, mantengo el mapa de cubetas como una ventana deslizante de tamano `indexDiff`. Al procesar cada elemento en la posicion `i`:

1. **Calcular el id de cubeta** para el valor actual.
2. **Verificar la misma cubeta**: si ya contiene un elemento, retornar `true` inmediatamente. Como la ventana solo contiene elementos dentro de `indexDiff` posiciones, la restriccion de indices se satisface automaticamente.
3. **Verificar cubetas adyacentes**: si la cubeta vecina existe y su valor difiere por menos de `w` del valor actual, retornar `true`.
4. **Insertar** el valor actual en su cubeta.
5. **Desalojar** el elemento que sale de la ventana: cuando `i >= indexDiff`, remover el elemento en la posicion `i - indexDiff` de su cubeta.

Dado que cada cubeta puede contener a lo sumo un elemento a la vez (si dos elementos se mapearan a la misma cubeta, ya habriamos retornado `true`), el mapa nunca crece mas alla de `indexDiff + 1` entradas.

### Un Ejemplo Concreto

Con `nums = [1, 5, 9, 1, 5, 9]`, `indexDiff = 2`, `valueDiff = 3`:

```
Ancho de cubeta w = 4

i=0, val=1:  cubeta=0. Sin coincidencia. Insertar {0: 1}.
i=1, val=5:  cubeta=1. Verificar vecina 0: |5-1|=4 >= 4, no. Insertar {0: 1, 1: 5}.
i=2, val=9:  cubeta=2. Verificar vecina 1: |9-5|=4 >= 4, no. Insertar {0: 1, 1: 5, 2: 9}.
             Desalojar i=0 (val=1, cubeta=0). Mapa: {1: 5, 2: 9}.
i=3, val=1:  cubeta=0. Verificar vecina 1: |1-5|=4 >= 4, no. Insertar {0: 1, 1: 5, 2: 9}.
             Desalojar i=1 (val=5, cubeta=1). Mapa: {0: 1, 2: 9}.
i=4, val=5:  cubeta=1. Verificar vecina 0: |5-1|=4 >= 4, no.
                        Verificar vecina 2: |5-9|=4 >= 4, no. Insertar {0: 1, 1: 5, 2: 9}.
             Desalojar i=2 (val=9, cubeta=2). Mapa: {0: 1, 1: 5}.
i=5, val=9:  cubeta=2. Verificar vecina 1: |9-5|=4 >= 4, no. Insertar {0: 1, 1: 5, 2: 9}.
             Desalojar i=3 (val=1, cubeta=0). Mapa: {1: 5, 2: 9}.

Resultado: false
```

Ningun par satisface ambas restricciones simultaneamente.

### Por Que Solo Importan Tres Cubetas

Esta es la parte elegante. Para cualquier valor dado, la respuesta solo puede venir de elementos en tres cubetas: la misma (coincidencia garantizada), o las dos vecinas inmediatas (necesitan verificacion de distancia). Los elementos en cubetas a dos o mas de distancia tienen garantizado diferir por mas de `valueDiff`. Esto reduce cada busqueda a O(1) -- tres consultas al mapa hash sin importar el tamano de la ventana o el rango de valores.

## Solucion en Rust

```rust
use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        if value_diff < 0 {
            return false;
        }

        let mut buckets: HashMap<i64, i64> = HashMap::new();
        let w = value_diff as i64 + 1;

        let get_bucket_id = |val: i64| -> i64 {
            if val >= 0 {
                val / w
            } else {
                (val + 1) / w - 1
            }
        };

        for (i, &num) in nums.iter().enumerate() {
            let val = num as i64;
            let bucket_id = get_bucket_id(val);

            if buckets.contains_key(&bucket_id) {
                return true;
            }

            if let Some(&neighbor) = buckets.get(&(bucket_id - 1)) {
                if (val - neighbor).abs() < w {
                    return true;
                }
            }

            if let Some(&neighbor) = buckets.get(&(bucket_id + 1)) {
                if (val - neighbor).abs() < w {
                    return true;
                }
            }

            buckets.insert(bucket_id, val);

            if i as i32 >= index_diff {
                let old_val = nums[i - index_diff as usize] as i64;
                let old_bucket = get_bucket_id(old_val);
                buckets.remove(&old_bucket);
            }
        }

        false
    }
}
```

La implementacion usa `i64` en todo momento para evitar desbordamiento al calcular diferencias entre valores `i32` -- un par como `(i32::MIN, i32::MAX)` desbordaria una resta en `i32`. El ancho de cubeta `w` es `value_diff + 1` porque queremos que valores que difieren por *exactamente* `valueDiff` caigan en la misma cubeta. El closure `get_bucket_id` maneja valores negativos con la formula `(val + 1) / w - 1`, asegurando tamanos de cubeta uniformes a traves del limite positivo-negativo. El retorno temprano `if value_diff < 0` protege contra una restriccion imposible. La logica de desalojo al final remueve el elemento que acaba de salir de la ventana, manteniendo el invariante de que el mapa solo contiene elementos dentro de `indexDiff` posiciones del indice actual.

## Conclusion

Contains Duplicate III es un problema que recompensa pensar en la proximidad de valores en terminos estructurales en lugar de numericos. Al particionar la recta numerica en cubetas de ancho `valueDiff + 1`, transformamos una consulta de rango en una busqueda de tiempo constante en un mapa hash. La ventana deslizante mantiene la restriccion de indices con simple insercion y desalojo. El resultado es un algoritmo limpio en O(N) que procesa cada elemento exactamente una vez con tres busquedas hash por paso -- sin arboles, sin ordenamiento, solo la abstraccion correcta aplicada a la restriccion correcta.
