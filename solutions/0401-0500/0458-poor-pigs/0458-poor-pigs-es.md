---
title: "0458 Poor Pigs - ES"
problemUrl: "https://leetcode.com/problems/poor-pigs/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["math", "combinatorics"]
complexity:
  time: "O(log(buckets) / log(states)), donde states = (minutesToTest / minutesToDie) + 1"
  space: "O(1)"
---

# Cerditos, Veneno y la Geometria de la Informacion

## El Problema
Hay `buckets` cubetas de liquido, exactamente una de ellas esta envenenada. Para averiguar cual es la cubeta envenenada, se pueden alimentar cerdos con el liquido. Un cerdo muere si bebe del veneno, y tarda exactamente `minutesToDie` minutos en morir. Dado un tiempo total de `minutesToTest` minutos para realizar las pruebas, retornar el numero minimo de cerdos necesarios para identificar con certeza la cubeta envenenada.

## La Ilusion de la Busqueda Lineal

Mi primer instinto fue pensar en esto como un problema de eliminacion. Un cerdo puede probar una cubeta por ronda, asi que con una ronda podria descartar cubetas una por una. Pero eso necesitaria potencialmente tantos cerdos como cubetas, lo cual es absurdo. La clave esta en que los cerdos no estan limitados a probar una sola cubeta: pueden beber de multiples cubetas simultaneamente, y la informacion que proporcionan al morir (o sobrevivir) es mucho mas rica de lo que parece.

## La Dimension del Tiempo

Lo que transforma este problema es darse cuenta de que el tiempo no es un simple limite: es una dimension adicional de informacion. Si tengo `minutesToTest / minutesToDie` rondas disponibles, cada cerdo puede encontrarse en uno de varios estados al final del experimento: murio en la ronda 1, murio en la ronda 2, ..., murio en la ronda R, o sobrevivio. Eso son `R + 1` estados posibles por cerdo.

Con un solo cerdo y `R + 1` estados, puedo distinguir entre `R + 1` cubetas: le doy una cubeta diferente en cada ronda, y el momento en que muere (o el hecho de que sobreviva) me dice cual estaba envenenada.

## Multiplicando Dimensiones

Ahora viene el salto conceptual. Si tengo dos cerdos, cada uno con `R + 1` estados, puedo organizar las cubetas en una cuadricula de `(R+1) x (R+1)`. El primer cerdo identifica la fila y el segundo la columna. En la ronda k, el primer cerdo bebe de todas las cubetas de la fila k, y el segundo de todas las cubetas de la columna k. Sus combinaciones de muerte/supervivencia identifican la celda exacta.

Esto se extiende naturalmente a mas dimensiones. Con `P` cerdos, puedo organizar las cubetas en un hipercubo de `P` dimensiones, cada una de tamano `(R+1)`. La capacidad total es `(R+1)^P`. Solo necesito encontrar el menor `P` tal que `(R+1)^P >= buckets`.

## La Elegancia del Algoritmo

El algoritmo es casi trivialmente simple una vez que se comprende la teoria:

1. Calcular `rounds = minutesToTest / minutesToDie`.
2. Calcular `states = rounds + 1` (los estados posibles por cerdo).
3. Multiplicar `states` por si mismo repetidamente, contando cuantos cerdos se necesitan hasta que la capacidad iguale o supere el numero de cubetas.

No hay busqueda, no hay simulacion, no hay DP. Es pura matematica combinatoria traducida a un bucle de tres lineas.

## Paso a Paso con un Ejemplo

Consideremos `buckets = 1000`, `minutesToDie = 15`, `minutesToTest = 60`.

- Rondas disponibles: `60 / 15 = 4`.
- Estados por cerdo: `4 + 1 = 5`.
- Con 1 cerdo: capacidad = 5. No alcanza.
- Con 2 cerdos: capacidad = 25. No alcanza.
- Con 3 cerdos: capacidad = 125. No alcanza.
- Con 4 cerdos: capacidad = 625. No alcanza.
- Con 5 cerdos: capacidad = 3125. Alcanza y sobra.

Respuesta: 5 cerdos. Cada cerdo representa un eje en un espacio de 5 dimensiones con 5 posiciones por eje. Las 3125 celdas cubren con creces las 1000 cubetas.

## Solucion en Rust

```rust
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let rounds = minutes_to_test / minutes_to_die;
        let states = rounds + 1;
        let mut pigs = 0;
        let mut capacity = 1;

        while capacity < buckets {
            capacity *= states;
            pigs += 1;
        }

        pigs
    }
}
```

La solucion calcula primero cuantas rondas de prueba hay disponibles y de ahi el numero de estados por cerdo. Luego entra en un bucle que multiplica la capacidad por `states` en cada iteracion, incrementando el contador de cerdos. El bucle termina en cuanto la capacidad iguala o supera el numero de cubetas. La operacion es O(log(buckets) / log(states)) en tiempo y O(1) en espacio, lo cual es practicamente instantaneo para cualquier entrada valida.

## Conclusion

Poor Pigs es un problema que disfraza un razonamiento de teoria de la informacion detras de un escenario aparentemente caprichoso. La clave no esta en simular pruebas ni en optimizar asignaciones, sino en reconocer que cada cerdo es un eje de informacion y que el tiempo anade estados a cada eje. La cantidad de cubetas que se pueden distinguir es el producto de los estados de todos los cerdos, y encontrar el numero minimo de cerdos se reduce a una simple comparacion de potencias. Es uno de esos problemas donde la solucion mas elegante es tambien la mas corta.
