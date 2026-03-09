---
title: "0446 Arithmetic Slices II - Subsequence - ES"
problemUrl: "https://leetcode.com/problems/arithmetic-slices-ii-subsequence/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "hash-table", "subsequence"]
complexity:
  time: "O(N^2), donde N es la longitud del arreglo"
  space: "O(N^2), por los hash maps almacenados en cada indice"
---

# Contando Fantasmas en la Secuencia

## El Problema
Dado un arreglo de enteros `nums`, retornar la cantidad de todas las subsecuencias aritmeticas de `nums`. Una secuencia es aritmetica si tiene al menos tres elementos y la diferencia entre elementos consecutivos es la misma. Una subsecuencia es una secuencia derivada del arreglo eliminando algunos o ningun elemento sin cambiar el orden de los restantes.

## La Trampa de la Fuerza Bruta

Mi primera reaccion fue enumerar cada subsecuencia posible y verificar si forma una progresion aritmetica. Pero un arreglo de longitud N tiene 2^N subsecuencias, lo cual es catastroficamente lento. Incluso restringiendose a subsecuencias de longitud 3 o mas, la cantidad de combinaciones hace que la fuerza bruta sea imposible para las restricciones dadas (N hasta 1000). Necesito una forma de contar sin construir.

## Pensar en Pares, No en Tripletas

La idea clave que desbloquea este problema es pensar en pares en lugar de subsecuencias completas. Toda subsecuencia aritmetica de longitud k se puede descomponer en una subsecuencia aritmetica de longitud k-1 que termina en el penultimo elemento, mas el ultimo elemento. Esto significa que si se, para cada elemento `nums[j]`, cuantas subsecuencias aritmeticas con una diferencia comun `d` terminan en `j`, puedo extender cada una de ellas agregando un nuevo elemento `nums[i]` donde `nums[i] - nums[j] == d`.

Esta es la formulacion DP: sea `dp[i]` un HashMap donde `dp[i][d]` almacena la cantidad de subsecuencias aritmeticas "debiles" (longitud 2 o mas) que terminan en el indice `i` con diferencia comun `d`. Las llamo "debiles" porque un par de dos elementos todavia no es una subsecuencia aritmetica valida (necesitamos al menos 3), pero es un prefijo potencial de una.

## Como se Acumula el Conteo

Para cada par `(j, i)` donde `j < i`, calculo `diff = nums[i] - nums[j]`. Entonces:

1. Busco `dp[j][diff]`, que me dice cuantas subsecuencias aritmeticas debiles terminan en `j` con esta diferencia. Cada una de estas tiene longitud al menos 2, asi que extenderlas con `nums[i]` produce subsecuencias de longitud al menos 3 -- validas. Sumo este conteo al total.

2. Actualizo `dp[i][diff]` sumando `dp[j][diff] + 1`. El `+1` representa el nuevo par `(nums[j], nums[i])` en si mismo, que es una subsecuencia debil de longitud 2 que podria extenderse despues.

La belleza esta en que nunca rastreo explicitamente la longitud de ninguna subsecuencia. El `+1` crea las semillas (pares), y la propagacion de `dp[j][diff]` cuenta todas las extensiones validas. Cada subsecuencia aritmetica de longitud 3 o mas se cuenta exactamente una vez, en el momento en que se agrega su ultimo elemento.

## Paso a Paso con un Ejemplo

Consideremos `nums = [2, 4, 6, 8, 10]`.

- **i = 1 (nums[i] = 4)**: Par con j=0: diff=2, dp[0][2]=0. Total += 0. dp[1][2] = 0 + 1 = 1.
- **i = 2 (nums[i] = 6)**: Par con j=0: diff=4, dp[2][4] = 1. Par con j=1: diff=2, dp[1][2]=1. Total += 1. dp[2][2] = 1 + 1 = 2.
- **i = 3 (nums[i] = 8)**: Par con j=0: diff=6, dp[3][6] = 1. Par con j=1: diff=4, dp[1][4]=0, dp[3][4] = 1. Par con j=2: diff=2, dp[2][2]=2. Total += 2. dp[3][2] = 2 + 1 = 3.
- **i = 4 (nums[i] = 10)**: Par con j=0: diff=8, dp[4][8]=1. Par con j=1: diff=6, dp[4][6]=1. Par con j=2: diff=4, dp[2][4]=1, total += 1, dp[4][4]=2. Par con j=3: diff=2, dp[3][2]=3, total += 3, dp[4][2] = 3 + 1 = 4.

Total final = 0 + 1 + 2 + 1 + 3 = 7. Las siete subsecuencias aritmeticas son: [2,4,6], [4,6,8], [6,8,10], [2,4,6,8], [4,6,8,10], [2,6,10], [2,4,6,8,10].

## Por Que i64 para la Diferencia

Un detalle sutil: el problema permite valores hasta 2^31 - 1 y tan bajos como -2^31. La diferencia entre dos de estos valores puede desbordar un entero de 32 bits. Al convertir a `i64` antes de restar, evito esta trampa por completo.

## Solucion en Rust

```rust
use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut total = 0;
        let mut dp: Vec<HashMap<i64, i32>> = vec![HashMap::new(); n];

        for i in 0..n {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64;
                let count = *dp[j].get(&diff).unwrap_or(&0);

                total += count;

                *dp[i].entry(diff).or_insert(0) += count + 1;
            }
        }

        total
    }
}
```

La solucion asigna un vector de HashMaps, uno por indice. Para cada par `(j, i)`, calcula la diferencia como `i64`, obtiene el conteo de subsecuencias debiles existentes en `j` con esa diferencia, lo suma al total acumulado, y luego actualiza `dp[i]` para incluir tanto las subsecuencias extendidas como el nuevo par. La API `entry` hace que la actualizacion del HashMap sea limpia: si la clave no existe, inserta 0 antes de sumar. Todo el computo se ejecuta en O(N^2) en tiempo y espacio, lo cual maneja comodamente arreglos hasta el limite de 1000 elementos.

## Conclusion

Arithmetic Slices II - Subsequence es un problema que premia un cambio de perspectiva. En lugar de intentar enumerar subsecuencias directamente, las cuento construyendo desde pares. Cada par es una semilla que, al extenderse, se convierte en una subsecuencia aritmetica valida. El DP de HashMap-por-indice rastrea cada diferencia comun posible simultaneamente, y la logica de acumulacion asegura que cada subsecuencia valida de longitud 3 o mas se cuente exactamente una vez en el momento en que se completa. Es un ejemplo de libro de texto de como la formulacion DP correcta puede convertir un problema exponencial en uno cuadratico.
