---
title: "0123 Best Time to Buy and Sell Stock III - ES"
problemUrl: "https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["array", "dynamic-programming"]
complexity:
  time: "O(N)"
  space: "O(1)"
---

# Dos Oportunidades en el Mercado: Maximizando Ganancias con un Par de Operaciones

## El Problema
Se te da un arreglo `prices` donde `prices[i]` es el precio de una accion en el dia `i`. Encuentra la ganancia maxima que puedes obtener. Puedes completar como maximo dos transacciones. Nota: No puedes participar en multiples transacciones simultaneamente (es decir, debes vender la accion antes de comprar otra).

## La Primera Impresion

La version de una sola transaccion de este problema es un calentamiento clasico: rastrea el precio minimo visto hasta ahora y actualiza la mejor ganancia de forma voraz. Pero en el momento en que permitimos *dos* transacciones, el panorama cambia dramaticamente. Ahora estamos eligiendo dos intervalos de compra-venta que no se superponen en la linea de tiempo, y la ganancia de la primera operacion puede efectivamente subsidiar el costo de la segunda. Un enfoque ingenuo probaria cada par posible de intervalos -- O(N^2) en el mejor caso -- pero eso ignora la estructura elegante que se esconde bajo la superficie.

La idea clave es pensar en el problema como una **maquina de estados**. En cualquier momento, estamos en uno de cuatro estados: sosteniendo nuestra primera accion, habiendo vendido nuestra primera accion, sosteniendo nuestra segunda accion, o habiendo vendido nuestra segunda accion. Cada transicion de estado cuesta dinero (comprar) o genera dinero (vender), y queremos maximizar el valor del estado final. Esto transforma una pesadilla combinatoria en un unico recorrido de izquierda a derecha.

## Cuatro Variables, Una Pasada

En lugar de construir tablas o dividir el arreglo, rastreamos cuatro valores mientras recorremos los precios:

- **`buy1`**: el mejor (menos negativo) costo de comprar la primera accion hasta ahora. Lo inicializamos en infinito negativo porque antes de ver cualquier precio, no se ha realizado ninguna compra.
- **`sell1`**: la mejor ganancia despues de completar la primera venta. Comienza en cero -- no hacer nada siempre es una opcion.
- **`buy2`**: el mejor costo efectivo de comprar la segunda accion, *despues* de embolsar la ganancia de la primera operacion. Tambien comienza en infinito negativo.
- **`sell2`**: la mejor ganancia despues de completar ambas operaciones. Comienza en cero.

### Las transiciones

Para cada precio, actualizamos los cuatro estados en orden:

1. `buy1 = max(buy1, -price)` -- ¿deberiamos comprar aqui por primera vez? El costo es `-price`.
2. `sell1 = max(sell1, buy1 + price)` -- ¿deberiamos vender nuestra primera accion aqui? La ganancia es lo que pagamos (`buy1`, un numero negativo) mas el precio actual.
3. `buy2 = max(buy2, sell1 - price)` -- ¿deberiamos iniciar la segunda operacion aqui? El costo efectivo es la ganancia de la primera operacion menos el precio actual.
4. `sell2 = max(sell2, buy2 + price)` -- ¿deberiamos completar la segunda operacion aqui?

La belleza es que estas cuatro actualizaciones no interfieren entre si dentro de la misma iteracion. Aunque `buy1` podria cambiar antes de que se calcule `sell1`, la operacion `max` asegura que solo mejoramos nuestro mejor estado conocido. Y como `buy2` se construye sobre `sell1`, la ganancia de la primera transaccion fluye naturalmente hacia la segunda.

### Por que esto maneja los casos limite

Si solo existe una operacion rentable, `buy2` y `sell2` efectivamente replicaran la primera operacion o añadiran cero ganancia, asi que `sell2` sigue dando la respuesta correcta. Si no existe ninguna operacion rentable, todo se mantiene en cero. El algoritmo se degrada con gracia sin necesidad de ramas para casos especiales.

## Solucion en Rust

```rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy1 = i32::MIN;
        let mut sell1 = 0;

        let mut buy2 = i32::MIN;
        let mut sell2 = 0;

        for price in prices {
            buy1 = buy1.max(-price);

            sell1 = sell1.max(buy1 + price);

            buy2 = buy2.max(sell1 - price);

            sell2 = sell2.max(buy2 + price);
        }

        sell2
    }
}
```

La implementacion en Rust refleja la maquina de estados con una pureza casi matematica. Inicializamos ambos estados de compra en `i32::MIN` -- el equivalente en Rust de infinito negativo para enteros de 32 bits -- para representar la imposibilidad de haber comprado antes de ver cualquier precio. El metodo `max` sobre `i32` mantiene cada actualizacion limpia y sin bifurcaciones. Observa que no hay necesidad de sentencias `if`, variables temporales, ni siquiera indexacion en el arreglo -- el iterador `for price in prices` consume el vector directamente. La solucion completa se ejecuta en una sola pasada con cuatro variables enteras, logrando O(N) en tiempo y O(1) en espacio -- tan eficiente como un algoritmo puede llegar a ser.

## Conclusion

Este problema es una clase magistral en pensamiento de maquina de estados aplicado a la programacion dinamica. Lo que inicialmente parece exigir dividir arreglos o anidar bucles se reduce a cuatro operaciones `max` cuidadosamente ordenadas por elemento. La idea clave -- que la segunda compra puede absorber la ganancia de la primera venta como un descuento -- es lo que hace posible el enfoque de una sola pasada. Es un patron que generaliza hermosamente: para como maximo *k* transacciones, mantendrias *2k* variables y aplicarias la misma logica en cascada. Pero para `k = 2`, el resultado es especialmente satisfactorio -- cuatro variables, un bucle, y una respuesta dificil de superar.
