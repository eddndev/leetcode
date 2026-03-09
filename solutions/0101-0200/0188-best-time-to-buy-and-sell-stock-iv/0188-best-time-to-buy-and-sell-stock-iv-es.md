---
title: "0188 Best Time to Buy and Sell Stock IV - ES"
problemUrl: "https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["array", "dynamic-programming"]
complexity:
  time: "O(N * K)"
  space: "O(K)"
---

# K Asaltos en el Ring del Trading: Exprimiendo la Ganancia Maxima con Transacciones Limitadas

## El Problema
Se te da un entero `k` y un arreglo `prices` donde `prices[i]` es el precio de una accion en el dia `i`. Encuentra la ganancia maxima que puedes obtener. Puedes completar como maximo `k` transacciones (una transaccion es una compra seguida de una venta). Nota: No puedes participar en multiples transacciones simultaneamente (es decir, debes vender la accion antes de comprar otra).

## La Primera Impresion

Este es la forma generalizada de la familia de problemas de compra-venta de acciones. Cuando `k = 1` es el clasico problema de una sola operacion; cuando `k = 2` es la conocida variante de dos transacciones. Pero aqui `k` puede ser cualquier valor, asi que necesitamos un marco que escale con gracia segun el numero de operaciones permitidas.

Un enfoque de fuerza bruta -- enumerar todas las combinaciones posibles de hasta `k` pares de compra-venta que no se superpongan -- explota combinatoriamente. La observacion clave es que en cualquier dia dado, nuestro estado optimo depende unicamente de cuantas transacciones hemos completado hasta ahora y de si actualmente tenemos una accion en mano. Esta es una configuracion clasica para **programacion dinamica sobre una maquina de estados**: para cada ranura de transaccion `j` de `1` a `k`, mantenemos dos valores -- la mejor posicion si actualmente tenemos una accion despues de comprar en nuestra `j`-esima operacion (`buy[j]`), y la mejor ganancia despues de haber vendido en nuestra `j`-esima operacion (`sell[j]`).

Hay un atajo critico que evita que la solucion exceda el tiempo limite en casos extremos: si `k >= n / 2` (donde `n` es el numero de dias), entonces efectivamente tenemos transacciones ilimitadas. En ese escenario, cada movimiento ascendente de precio puede capturarse de forma voraz, y colapsamos todo el problema a un simple recorrido lineal sumando todas las diferencias positivas entre dias consecutivos.

## La Maquina de Estados con K Capas

Mantenemos dos arreglos de longitud `k + 1`:

- **`buy[j]`**: el valor maximo que podemos tener si estamos sosteniendo una accion y hemos entrado en nuestra `j`-esima transaccion. Se inicializa con un numero negativo grande (representando la imposibilidad de tener una accion antes de que se haya observado algun dia).
- **`sell[j]`**: la ganancia maxima despues de completar la `j`-esima venta. Se inicializa en cero -- no hacer nada siempre es valido.

### Las transiciones

Para cada precio en el arreglo, y para cada ranura de transaccion `j` de `1` a `k`:

1. `buy[j] = max(buy[j], sell[j - 1] - price)` -- ¿deberiamos iniciar la `j`-esima operacion comprando al precio de hoy? El costo efectivo es la ganancia acumulada de las primeras `j - 1` operaciones menos el precio de hoy.
2. `sell[j] = max(sell[j], buy[j] + price)` -- ¿deberiamos cerrar la `j`-esima operacion vendiendo hoy? La ganancia es nuestra posicion actual mas el precio de hoy.

Como `sell[j - 1]` alimenta a `buy[j]`, la ganancia de operaciones anteriores fluye naturalmente hacia las siguientes. Y como cada actualizacion es un `max`, solo mejoramos cada estado -- el orden de las actualizaciones dentro de un mismo dia no causa interferencias.

### El atajo de transacciones ilimitadas

Cuando `k >= n / 2`, ningun calendario de operaciones podria usar mas de `n / 2` transacciones (ya que cada operacion requiere al menos dos dias distintos). En este regimen cambiamos a una estrategia voraz: recorremos los precios con una ventana deslizante de tamaño dos, y cada vez que el precio de manana supera al de hoy, nos quedamos con la diferencia. Esto se ejecuta en O(N) y evita por completo el bucle O(N * K), lo cual es critico porque `k` puede ser tan grande como 10^9.

### Por que esto maneja los casos limite

Si existen menos de `k` operaciones rentables, las ranuras de transaccion sobrantes simplemente permanecen con ganancia cero -- nunca degradan la respuesta. Si el arreglo de precios tiene menos de dos elementos, o `k` es cero, no hay nada que operar y retornamos cero inmediatamente. El algoritmo maneja todas estas situaciones sin ramas especiales en el bucle principal.

## Solucion en Rust

```rust
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 || k == 0 {
            return 0;
        }
        let k = k as usize;

        if k >= n / 2 {
            return prices.windows(2).map(|w| (w[1] - w[0]).max(0)).sum();
        }

        let mut buy = vec![-1_000_000_000; k + 1];
        let mut sell = vec![0; k + 1];

        for price in prices {
            for j in 1..=k {
                buy[j] = buy[j].max(sell[j - 1] - price);

                sell[j] = sell[j].max(buy[j] + price);
            }
        }

        sell[k]
    }
}
```

La implementacion en Rust es compacta y expresiva. Los retornos tempranos manejan las entradas degeneradas -- menos de dos precios o cero transacciones permitidas. La verificacion `k >= n / 2` luego despacha el caso de transacciones ilimitadas usando `prices.windows(2)`, una forma bellamente idiomatica de iterar sobre pares consecutivos, sumando solo las diferencias positivas con `.max(0)`.

Para el caso general, `buy` y `sell` son vectores alojados en el heap de tamaño `k + 1`. El valor centinela `-1_000_000_000` sirve como un infinito negativo practico -- lo suficientemente grande para ser dominado por cualquier diferencia de precios real, evitando al mismo tiempo el desbordamiento cuando sumamos un precio (ya que los precios son como maximo 1000 y `k` esta acotado). El bucle anidado itera sobre cada precio y cada ranura de transaccion, realizando exactamente dos operaciones `max` por cada par `(price, j)`. Al final, `sell[k]` contiene la ganancia maxima alcanzable con como maximo `k` operaciones completas. La complejidad total es O(N * K) en tiempo y O(K) en espacio -- lo mejor que podemos lograr para el caso general sin recurrir a estructuras de datos mas exoticas.

## Conclusion

El problema 188 es la piedra angular de la serie de trading de acciones, pidiendonos generalizar desde conteos fijos de transacciones hasta un `k` arbitrario. La solucion es una extension natural del enfoque de maquina de estados: en lugar de cuatro variables fijas, usamos dos arreglos de longitud `k` y propagamos las ganancias de cada operacion completada hacia la siguiente compra. La optimizacion critica -- detectar cuando `k` es lo suficientemente grande para permitir transacciones ilimitadas y cambiar a un recorrido lineal voraz -- evita que el algoritmo se ahogue en entradas patologicas donde `k` supera ampliamente el numero de dias. Es una demostracion satisfactoria de que la abstraccion limpia (la maquina de estados) y los atajos pragmaticos (el respaldo voraz) pueden coexistir en la misma solucion, cada uno cubriendo la debilidad del otro.
