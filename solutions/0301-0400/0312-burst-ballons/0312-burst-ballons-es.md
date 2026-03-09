---
title: "0312 Burst Balloons - ES"
problemUrl: "https://leetcode.com/problems/burst-balloons/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "interval-dp"]
complexity:
  time: "O(N^3)"
  space: "O(N^2)"
---

# El Ultimo Globo en Reventar

## El Problema
Dado un arreglo de enteros `nums` que representa globos, donde cada globo tiene un numero pintado. Se nos pide reventar todos los globos. Al reventar el globo `i`, se obtienen `nums[i-1] * nums[i] * nums[i+1]` monedas. Si `i-1` o `i+1` estan fuera de los limites del arreglo, se tratan como si tuvieran un `1`. Encontrar el maximo de monedas que se pueden obtener reventando todos los globos.

## La Intuicion Inicial

Mi primer instinto fue pensar en esto como un problema de backtracking: probar todos los ordenes posibles de reventar globos y quedarnos con el que da mas monedas. Pero con hasta 300 globos, eso es N! permutaciones, algo completamente inviable.

Lo que hace este problema particularmente complicado es que reventar un globo cambia los vecinos de los globos restantes. Si reviento el globo del medio en `[3, 1, 5]`, ahora el 3 y el 5 se vuelven vecinos. Esa dependencia hacia adelante hace que sea dificil pensar en subproblemas.

Y aqui esta el giro mental que lo cambia todo: **en lugar de pensar en cual globo reventar primero, pensemos en cual reventar ultimo**.

## La Inversion del Pensamiento

Si decido que el globo `k` sera el ultimo en reventarse dentro de un rango `(left, right)`, entonces se que cuando lo reviente, sus vecinos seran exactamente `left` y `right` (los bordes del rango, que aun no han sido reventados). Las monedas que obtengo son `nums[left] * nums[k] * nums[right]`.

Y lo mejor: los globos a la izquierda de `k` y los globos a la derecha de `k` forman subproblemas completamente independientes. Los que estan a la izquierda no saben nada de los que estan a la derecha, porque `k` todavia esta ahi separandolos. Esto es exactamente lo que necesitamos para programacion dinamica.

Para facilitar el manejo de los bordes, envolvemos el arreglo con dos `1`s virtuales: uno al inicio y otro al final. Asi, un arreglo como `[3, 1, 5, 8]` se convierte en `[1, 3, 1, 5, 8, 1]`. Ahora el problema es encontrar el maximo de monedas al reventar todos los globos entre el indice `0` y `len-1` (los dos `1`s virtuales nunca se revientan).

Definimos `dp[left][right]` como el maximo de monedas obtenible reventando todos los globos estrictamente entre `left` y `right`. Iteramos por tamanio de ventana: empezamos con ventanas de tamanio 2 (donde no hay globos en medio, asi que `dp = 0`), y vamos creciendo. Para cada ventana `(left, right)`, probamos cada globo `k` entre ellos como el ultimo en reventarse:

```
dp[left][right] = max(dp[left][k] + dp[k][right] + nums[left] * nums[k] * nums[right])
```

La respuesta final esta en `dp[0][len-1]`.

### Un Ejemplo Paso a Paso

Para `nums = [3, 1, 5, 8]`, el arreglo extendido es `[1, 3, 1, 5, 8, 1]`:

- Ventanas de tamanio 2 (sin globos en medio): todos `dp = 0`
- Ventana `(0, 2)`: solo `k=1` (globo 3). Monedas = `1*3*1 = 3`. `dp[0][2] = 3`
- Ventana `(1, 3)`: solo `k=2` (globo 1). Monedas = `3*1*5 = 15`. `dp[1][3] = 15`
- Ventana `(2, 4)`: solo `k=3` (globo 5). Monedas = `1*5*8 = 40`. `dp[2][4] = 40`
- Ventana `(3, 5)`: solo `k=4` (globo 8). Monedas = `5*8*1 = 40`. `dp[3][5] = 40`
- Ventanas mas grandes combinan resultados previos, y asi sucesivamente...
- La respuesta final en `dp[0][5] = 167`.

## Solucion en Rust

```rust
impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut padded_nums = Vec::with_capacity(n + 2);
        padded_nums.push(1);
        padded_nums.extend(nums);
        padded_nums.push(1);

        let len = padded_nums.len();

        let mut dp = vec![vec![0; len]; len];

        for window in 2..len {
            for left in 0..len - window {
                let right = left + window;

                for k in (left + 1)..right {
                    let coins = dp[left][k]
                        + dp[k][right]
                        + (padded_nums[left] * padded_nums[k] * padded_nums[right]);

                    if coins > dp[left][right] {
                        dp[left][right] = coins;
                    }
                }
            }
        }

        dp[0][len - 1]
    }
}
```

La implementacion en Rust refleja la idea de manera directa. Primero construimos `padded_nums` con los dos `1`s centinela usando `with_capacity` para una sola alocacion. La tabla `dp` es una matriz cuadrada de tamanio `len x len`, y el triple bucle anidado hace todo el trabajo: el externo controla el tamanio de la ventana, el medio la posicion de la ventana, y el interno prueba cada candidato `k` como ultimo globo. La comparacion manual `if coins > dp[left][right]` evita la dependencia de `std::cmp::max` y es igualmente legible. Al final, `dp[0][len - 1]` contiene la respuesta: el maximo de monedas posible reventando todos los globos originales.

## Conclusion

Burst Balloons es un problema que parece de backtracking puro hasta que se hace la inversion mental correcta. Pensar en el ultimo globo en reventarse en lugar del primero transforma un problema con dependencias enredadas en subproblemas independientes perfectos para DP de intervalos. La tecnica de agregar centinelas en los bordes simplifica el manejo de casos especiales de forma elegante. Es uno de esos problemas donde la dificultad no esta en la implementacion, sino en encontrar la forma correcta de descomponerlo.
