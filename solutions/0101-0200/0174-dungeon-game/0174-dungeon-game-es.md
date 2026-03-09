---
title: "0174 Dungeon Game - ES"
problemUrl: "https://leetcode.com/problems/dungeon-game/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "matrix"]
complexity:
  time: "O(M * N)"
  space: "O(N)"
---

# El Caballero que Caminaba Hacia Atras

## El Problema
Los demonios habian capturado a la princesa y la encerraron en la esquina inferior derecha de una mazmorra. La mazmorra consiste en una cuadricula de `M x N` habitaciones. Nuestro valiente caballero se encuentra inicialmente en la habitacion superior izquierda y debe abrirse camino hasta la princesa. El caballero tiene un punto de salud inicial y en cada habitacion gana o pierde puntos de salud segun el valor de esa habitacion (positivo para orbes magicos, negativo para demonios). La salud del caballero debe ser al menos 1 en todo momento. Debemos determinar la salud minima inicial con la que el caballero puede rescatar a la princesa.

## La Trampa del Camino Hacia Adelante

Mi primer impulso fue atacar esto como un problema clasico de DP hacia adelante: empezar en la esquina superior izquierda, y en cada celda calcular el minimo de salud que necesito haber tenido al inicio para llegar hasta ahi. Pero esto lleva a un callejon sin salida sutil. Cuando avanzo celda por celda, necesito dos datos simultaneamente: la salud *acumulada* y la salud *minima* que he alcanzado en el camino. Esos dos valores interactuan de formas que no se pueden optimizar con una sola tabla DP, porque un camino con menor acumulado podria tener un minimo local mas alto, y viceversa. El principio de optimalidad se quiebra.

La solucion es un giro mental elegante: caminar hacia atras. Si empiezo desde la princesa y retrocedo hacia el caballero, en cada celda solo necesito responder una pregunta: ¿cual es la salud minima que necesito tener *al entrar* a esta celda para poder llegar a la princesa desde aqui? Esa pregunta tiene una respuesta unica y se descompone limpiamente.

## La Logica del Retroceso

Definamos `dp[j]` como la salud minima necesaria al entrar a la celda `(i, j)` para poder llegar a la celda `(m-1, n-1)`. Iteramos desde la esquina inferior derecha hacia la superior izquierda.

Para cada celda, el caballero puede moverse solo hacia la derecha o hacia abajo. Por lo tanto, la salud minima que necesitara al entrar depende del minimo entre lo que necesitaria si va a la derecha (`dp[j+1]`) y lo que necesitaria si va abajo (`dp[j]` de la fila anterior, que en nuestra iteracion in-place ya es el valor actual):

```
min_hp_siguiente = min(dp[j], dp[j+1])
necesidad = min_hp_siguiente - dungeon[i][j]
```

Si la celda tiene un orbe positivo grande, la `necesidad` podria caer a cero o menos, lo que significaria que el caballero podria entrar muerto. Pero la regla dice que siempre debe tener al menos 1 punto de salud, asi que aplicamos `max(necesidad, 1)`.

### El Truco de la Inicializacion

Uso un arreglo de tamano `n+1` inicializado con `i32::MAX`. Esto actua como "pared" para las celdas del borde: una celda en la ultima fila no puede ir hacia abajo, y una celda en la ultima columna no puede ir a la derecha. El `MAX` asegura que esas direcciones prohibidas nunca sean elegidas por el `min`. La unica excepcion es `dp[n-1] = 1`, que establece que al llegar a la princesa (antes de entrar a su celda), necesitamos al menos 1 punto de salud viniendo del siguiente paso.

### Un Ejemplo Concreto

Consideremos la mazmorra:
```
[[-2, -3,  3],
 [-5, -10, 1],
 [10,  30, -5]]
```

Empezando desde `(2,2)`: necesito sobrevivir a la celda `-5`, y despues no hay mas celdas. Asi que necesito `1 - (-5) = 6`. Pero espera, como es la ultima celda, solo necesito `max(1 - (-5), 1) = 6` al entrar.

Luego `(2,1)`: la celda vale `30`, y a la derecha necesito 6. Necesidad: `6 - 30 = -24`. Como es negativo, el caballero solo necesita 1 punto de salud al entrar aqui.

Siguiendo el retroceso completo, la respuesta resulta ser **7**: el caballero necesita comenzar con 7 puntos de salud para sobrevivir el camino optimo.

## Solucion en Rust

```rust
use std::cmp;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();

        let mut dp = vec![i32::MAX; n + 1];

        dp[n - 1] = 1;

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let min_hp_next = cmp::min(dp[j], dp[j + 1]);

                let need = min_hp_next - dungeon[i][j];

                dp[j] = if need <= 0 { 1 } else { need };
            }
        }

        dp[0]
    }
}
```

La implementacion en Rust es notablemente compacta para un problema Hard. El vector `dp` de tamano `n + 1` actua como una fila deslizante que se reutiliza de abajo hacia arriba. Cuando iteramos `j` de derecha a izquierda, `dp[j]` todavia contiene el valor de la fila de abajo (el costo de ir hacia abajo), mientras que `dp[j+1]` ya fue actualizado para la fila actual (el costo de ir a la derecha). Esta superposicion es exactamente lo que necesitamos: `cmp::min(dp[j], dp[j+1])` nos da el minimo entre ambas direcciones sin necesidad de mantener dos arreglos. La condicion `if need <= 0 { 1 } else { need }` es la forma idiomatica de expresar `max(need, 1)`, asegurando que el caballero nunca "entre muerto" a ninguna celda.

## Conclusion

Este problema es un recordatorio de que la direccion en la que recorres un espacio de estados importa profundamente. Hacia adelante, el problema es intratable con DP estandar porque el futuro afecta la interpretacion del pasado. Hacia atras, cada celda tiene una respuesta limpia y autocontenida que solo depende de sus vecinos ya calculados. Es la misma mazmorra, los mismos demonios, los mismos orbes -- pero mirada desde la princesa hacia el caballero, la niebla se disipa y el camino se vuelve claro.
