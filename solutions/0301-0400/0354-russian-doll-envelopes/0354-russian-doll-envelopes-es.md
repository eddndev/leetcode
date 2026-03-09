---
title: "0354 Russian Doll Envelopes - ES"
problemUrl: "https://leetcode.com/problems/russian-doll-envelopes/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "binary-search", "sorting", "greedy"]
complexity:
  time: "O(N log N) donde N es el numero de sobres"
  space: "O(N)"
---

# Munecas Rusas en Papel

## El Problema
Dado un arreglo 2D de enteros `envelopes` donde `envelopes[i] = [wi, hi]` representa el ancho y alto de un sobre, devolver el numero maximo de sobres que se pueden anidar estilo muneca rusa. Un sobre cabe dentro de otro si y solo si tanto su ancho como su alto son estrictamente menores que los del otro sobre. No se permite rotar los sobres.

## La Intuicion Inicial

Este problema parece ser de anidamiento multidimensional, pero tiene una reduccion elegante a un problema clasico unidimensional. Si pudiera de alguna manera fijar una dimension, el problema se reduciria a encontrar la subsecuencia creciente mas larga (LIS) en la otra dimension. La clave es encontrar un ordenamiento que haga esto valido.

Si ordeno los sobres por ancho de menor a mayor, entonces solo necesito preocuparme por las alturas: cualquier subsecuencia creciente en alturas automaticamente satisface la restriccion de ancho, porque los sobres ya estan en orden de ancho. Pero hay una trampa -- sobres con el mismo ancho no pueden anidarse entre si, y un LIS ingenuo sobre las alturas podria seleccionar multiples sobres con el mismo ancho.

## El Truco del Ordenamiento

La solucion a la trampa del mismo ancho es sutil pero poderosa. Cuando dos sobres tienen el mismo ancho, ordeno sus alturas en orden descendente. Por que funciona esto? Consideremos sobres con ancho 5 y alturas 3, 5, 7. Si los ordeno como `[5,7], [5,5], [5,3]`, entonces el algoritmo LIS sobre las alturas nunca seleccionara mas de uno de estos, porque las alturas van en orden decreciente -- no se puede construir una subsecuencia creciente a partir de una secuencia decreciente. Si estuvieran en orden ascendente `[5,3], [5,5], [5,7]`, el LIS podria tomar los tres, violando la restriccion de que el ancho debe ser estrictamente menor.

Asi que la regla de ordenamiento es: ancho ascendente como criterio primario, alto descendente como criterio de desempate. Esto transforma el problema 2D en un LIS puro sobre las alturas.

## De LIS Cuadratico a LIS con Busqueda Binaria

El algoritmo clasico de LIS en O(N^2) compararia cada par de elementos. Para una entrada de hasta 10^5 sobres, esto seria demasiado lento. Necesito el algoritmo de LIS con busqueda binaria que corre en O(N log N).

La idea es mantener un arreglo `tails` donde `tails[i]` es el menor valor final posible entre todas las subsecuencias crecientes de longitud `i + 1` encontradas hasta ahora. Este arreglo siempre esta ordenado, lo que permite busqueda binaria. Para cada nueva altura, busco la posicion donde deberia insertarse. Si es mayor que todos los elementos en `tails`, extiende la subsecuencia mas larga. Si no, reemplaza el elemento en la posicion encontrada, manteniendo la posibilidad de futuras extensiones con valores mas bajos.

## Por Que el Arreglo de Colas Funciona

El arreglo `tails` no almacena una subsecuencia valida real -- almacena la mejor cola posible para cada longitud. Cuando reemplazo `tails[idx]` con un valor menor, no estoy rompiendo ninguna subsecuencia existente. Estoy registrando que ahora existe una subsecuencia de longitud `idx + 1` que termina en un valor menor, lo cual solo puede ser mejor o igual para futuras extensiones.

La longitud de `tails` al final del proceso es exactamente la longitud del LIS, que es la respuesta a nuestro problema original.

## Solucion en Rust

```rust
impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let mut tails: Vec<i32> = Vec::new();

        for env in envelopes {
            let height = env[1];

            match tails.binary_search(&height) {
                Ok(_) => {}
                Err(idx) => {
                    if idx == tails.len() {
                        tails.push(height);
                    } else {
                        tails[idx] = height;
                    }
                }
            }
        }

        tails.len() as i32
    }
}
```

La implementacion comienza con un ordenamiento inestable por rendimiento, usando un comparador personalizado que ordena por ancho ascendente y por alto descendente cuando los anchos son iguales. Luego itera sobre cada sobre, extrayendo solo la altura ya que el ancho esta manejado por el ordenamiento. La llamada a `binary_search` de Rust devuelve `Ok(pos)` si el valor ya existe en `tails` -- en ese caso no hacemos nada, porque un duplicado no extiende ni mejora ninguna subsecuencia. Si devuelve `Err(idx)`, obtenemos el punto de insercion: si `idx` iguala la longitud de `tails`, el valor es mayor que todos los elementos existentes y extiende la subsecuencia mas larga; de lo contrario, reemplaza `tails[idx]` para mantener la cola optima para esa longitud.

## Conclusion

Russian Doll Envelopes es un ejemplo elegante de reduccion de dimensionalidad. Lo que parece un problema de anidamiento bidimensional se transforma, mediante un ordenamiento inteligente, en un LIS unidimensional clasico. El truco de ordenar por alto descendente dentro del mismo ancho neutraliza la posibilidad de seleccionar sobres invalidos. La busqueda binaria sobre el arreglo de colas lleva la complejidad de O(N^2) a O(N log N), haciendo la solucion eficiente incluso para entradas grandes. La implementacion en Rust es notablemente concisa: un ordenamiento, un ciclo y una busqueda binaria son todo lo que se necesita para resolver un problema de dificultad Hard.
