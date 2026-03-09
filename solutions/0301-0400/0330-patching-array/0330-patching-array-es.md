---
title: "0330 Patching Array - ES"
problemUrl: "https://leetcode.com/problems/patching-array/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["greedy", "array", "number-theory"]
complexity:
  time: "O(M + log N) donde M es la longitud de nums y N es el objetivo"
  space: "O(1)"
---

# Llenando los Huecos en la Recta Numerica

## El Problema
Dado un arreglo ordenado de enteros `nums` y un entero `n`, agregar el minimo numero de parches (elementos) al arreglo de manera que cualquier numero en el rango `[1, n]` pueda formarse como la suma de algunos elementos del arreglo. Devolver el numero minimo de parches requeridos.

## La Intuicion Inicial

Este problema esconde una hermosa intuicion greedy detras de lo que inicialmente parece una pesadilla combinatoria. Si tengo que garantizar que cada entero del 1 al `n` pueda representarse como una suma de elementos, el enfoque de fuerza bruta de verificar subconjuntos es claramente inviable. En cambio, necesito pensar en cobertura -- que rango de sumas puedo representar actualmente, y que pasa cuando encuentro un hueco?

El concepto clave es una variable que llamo `miss`: el numero mas pequenio que aun no puedo formar como suma de los elementos que he procesado hasta ahora. Si puedo formar todo numero en `[1, miss)`, entonces mi cobertura actual se extiende hasta `miss - 1`. La pregunta se convierte en: como extiendo esta cobertura eficientemente?

## Por Que la Estrategia Greedy Funciona

Supongamos que puedo formar actualmente toda suma en `[1, miss)`. Si el siguiente numero en mi arreglo ordenado es menor o igual a `miss`, puedo absorberlo: al sumarlo a cada suma que ya podia formar, ahora cubro `[1, miss + nums[i])`. Esto es porque cualquier suma en `[miss, miss + nums[i])` puede crearse tomando el nuevo elemento mas algun subconjunto que sume el resto, y ese resto esta garantizado de estar en `[1, miss)` que ya cubro.

Pero si el siguiente numero excede `miss`, hay un hueco. Ningun elemento existente puede ayudarme a alcanzar `miss` en si. El parche optimo en este caso es agregar el propio `miss`. Por que? Porque agregar `miss` duplica mi cobertura a `[1, 2 * miss)`. Cualquier parche mas pequenio extenderia menos la cobertura. Cualquier parche mas grande dejaria `miss` inalcanzable. Agregar `miss` es demostrablemente optimo -- es el unico numero que extiende maximamente el rango de sumas representables.

## Recorriendo un Ejemplo

Consideremos `nums = [1, 5, 10]` y `n = 20`.

Empezando con `miss = 1`. El primer elemento es `1`, que es `<= miss`, asi que lo absorbo: `miss` se convierte en `1 + 1 = 2`. Ahora cubro `[1, 2)`.

El siguiente elemento es `5`, pero `5 > miss = 2`. No puedo formar `2` todavia, asi que parcho agregando `2` mismo. Ahora `miss = 2 + 2 = 4`, cubriendo `[1, 4)`. Contador de parches: 1.

Sigo mirando `5`, y ahora `5 > miss = 4`. Parcho de nuevo con `4`. Ahora `miss = 4 + 4 = 8`, cubriendo `[1, 8)`. Contador de parches: 2.

Ahora `5 <= miss = 8`, asi que lo absorbo: `miss = 8 + 5 = 13`, cubriendo `[1, 13)`.

El siguiente elemento es `10`, y `10 <= 13`, asi que lo absorbo: `miss = 13 + 10 = 23`, cubriendo `[1, 23)`.

Como `23 > 20 = n`, he terminado. Respuesta: 2 parches.

## El Argumento de Duplicacion

La razon por la que este algoritmo es tan eficiente es el comportamiento de duplicacion al parchar. Cada vez que parcho, `miss` se duplica. Empezando desde 1, despues de `k` parches (en el peor caso sin elementos del arreglo para absorber), `miss` alcanza `2^k`. Para cubrir hasta `n`, necesito como maximo `log2(n)` parches. Combinado con el recorrido unico del arreglo, el tiempo total es `O(M + log N)`.

Esto es notablemente elegante: un problema que parece requerir busqueda exponencial sobre subconjuntos se reduce a un escaneo lineal con logaritmicamente pocos parches.

## Solucion en Rust

```rust
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut patches = 0;
        let mut miss: i64 = 1;
        let mut i = 0;
        let limit = n as i64;

        while miss <= limit {
            if i < nums.len() && (nums[i] as i64) <= miss {
                miss += nums[i] as i64;
                i += 1;
            } else {
                miss += miss;
                patches += 1;
            }
        }

        patches
    }
}
```

La implementacion es sorprendentemente concisa para un problema Hard. La variable `miss` se declara como `i64` para evitar desbordamiento -- dado que `n` puede ser hasta `2^31 - 1`, duplicar `miss` podria exceder el rango de `i32` durante calculos intermedios. El ciclo principal continua mientras `miss <= limit`, lo que significa que todavia hay valores en `[1, n]` que necesitan ser alcanzables. Dentro del ciclo, hay exactamente dos casos: o bien el elemento actual del arreglo cabe dentro de la frontera de cobertura y la extiende aditivamente, o existe un hueco y parcho duplicando `miss`. El indice `i` solo avanza cuando un elemento del arreglo es absorbido, manejando naturalmente el caso donde se agotan los elementos del arreglo -- en ese punto, cada iteracion parcha, y la duplicacion asegura convergencia rapida hacia `n`.

## Conclusion

Patching Array es uno de esos problemas donde la solucion es enganiosamente simple una vez que ves el invariante correcto. La variable `miss` -- rastreando la suma inalcanzable mas pequenia -- transforma un problema de cobertura aparentemente intratable en un algoritmo greedy limpio. La prueba de optimalidad se sigue del hecho de que parchar con el propio `miss` es siempre la mejor opcion: extiende maximamente la cobertura al duplicarla. El resultado es un algoritmo `O(M + log N)` con espacio constante, codificado en apenas una docena de lineas de Rust. Es un recordatorio de que los problemas mas dificiles a veces tienen las soluciones mas cortas.
