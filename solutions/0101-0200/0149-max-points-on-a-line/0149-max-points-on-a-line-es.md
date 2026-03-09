---
title: "0149 Max Points on a Line - ES"
problemUrl: "https://leetcode.com/problems/max-points-on-a-line/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["hash-table", "math", "geometry"]
complexity:
  time: "O(N^2)"
  space: "O(N)"
---

# La Conspiracion de los Puntos Colineales

## El Problema
Dado un arreglo de `points` donde `points[i] = [xi, yi]` representa un punto en el plano X-Y, devolver el numero maximo de puntos que se encuentran sobre la misma linea recta.

## La Geometria Escondida en las Pendientes

A primera vista, este problema parece invitar a un enfoque de fuerza bruta triple: para cada par de puntos definir una linea, y luego contar cuantos otros puntos caen sobre ella. Eso seria O(N^3) y funciona, pero hay algo mas elegante esperando debajo de la superficie.

La observacion fundamental es que una linea queda completamente definida por un punto y una pendiente. Si fijo un punto de referencia y calculo la pendiente hacia todos los demas puntos, aquellos que compartan la misma pendiente estan necesariamente sobre la misma linea (la que pasa por mi punto de referencia con esa inclinacion). Asi, el problema se reduce a: para cada punto, encontrar la pendiente mas popular entre las pendientes hacia todos los demas.

## La Trampa de los Numeros Flotantes

Aqui surge la trampa mas peligrosa del problema. Si representamos la pendiente como un numero de punto flotante (`dy / dx`), las imprecisiones de la aritmetica flotante pueden hacer que dos pendientes que deberian ser identicas difieran en el ultimo decimal. Dos puntos que geometricamente estan sobre la misma linea podrian producir pendientes "distintas" al dividir.

La solucion es no dividir nunca. En lugar de almacenar la pendiente como un flotante, la represento como una fraccion reducida `(dy, dx)` donde he dividido ambos componentes por su maximo comun divisor. Esto convierte cada pendiente en una pareja unica de enteros que puedo usar como llave en un HashMap sin ningun riesgo de imprecision.

## Normalizacion: El Detalle que lo Hace Funcionar

Representar la pendiente como `(dy, dx)` no es suficiente por si solo. La pendiente entre el punto A y el punto B podria producir `(2, -3)`, mientras que entre A y otro punto colineal C podria producir `(-2, 3)`. Geometricamente es la misma pendiente, pero las tuplas son diferentes.

Para resolver esto, normalizo con dos reglas:

1. **Divido por el GCD** de los valores absolutos para obtener la fraccion irreducible.
2. **Fuerzo que `dx` sea siempre positivo.** Si `dx` es negativo, multiplico ambos por -1. Esto garantiza que `(2, 3)` y `(-2, -3)` se representen de forma identica.

Los casos especiales tambien necesitan atencion: si `dx == 0`, la linea es vertical y la codifico como `(1, 0)`. Si `dy == 0`, la linea es horizontal y la codifico como `(0, 1)`. Con estas convenciones, cada linea tiene exactamente una representacion canonica.

## El Algoritmo Completo

Para cada punto `i`, creo un HashMap fresco. Luego, para cada punto `j > i`, calculo la pendiente normalizada entre `i` y `j`, e incremento el contador de esa pendiente en el mapa. El maximo valor en el mapa me dice cuantos puntos comparten la pendiente mas comun con el punto `i` -- sumando 1 por el propio punto `i`, obtengo el numero total de puntos colineales a traves de el. El maximo global a traves de todos los puntos de referencia es la respuesta.

### Un Ejemplo Concreto

Con los puntos `[[1,1], [2,2], [3,3]]`:

- Fijando el punto `[1,1]`: pendiente hacia `[2,2]` es `(1,1)`, pendiente hacia `[3,3]` es `(2,2)` que se reduce a `(1,1)`. El HashMap tiene `{(1,1): 2}`. Maximo local: 2 + 1 = 3.

Los tres puntos estan sobre la misma linea, y el algoritmo lo detecta sin jamas calcular un flotante.

## Solucion en Rust

```rust
use std::collections::HashMap;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 2 {
            return n as i32;
        }

        let mut max_points = 1;

        for i in 0..n {
            let p1 = &points[i];
            let mut slopes: HashMap<(i32, i32), i32> = HashMap::new();

            for j in i + 1..n {
                let p2 = &points[j];

                let delta_y = p2[1] - p1[1];
                let delta_x = p2[0] - p1[0];

                let slope = Self::get_normalized_slope(delta_y, delta_x);

                *slopes.entry(slope).or_insert(0) += 1;
            }

            let current_max = slopes.values().max().unwrap_or(&0) + 1;
            max_points = max_points.max(current_max);
        }

        max_points
    }

    fn get_normalized_slope(dy: i32, dx: i32) -> (i32, i32) {
        if dx == 0 {
            return (1, 0);
        }
        if dy == 0 {
            return (0, 1);
        }

        let divisor = Self::gcd(dy.abs(), dx.abs());
        let mut res_dy = dy / divisor;
        let mut res_dx = dx / divisor;

        if res_dx < 0 {
            res_dy = -res_dy;
            res_dx = -res_dx;
        }

        (res_dy, res_dx)
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
```

La implementacion en Rust es particularmente satisfactoria. El `HashMap<(i32, i32), i32>` usa tuplas de enteros como llaves, algo que en muchos lenguajes requeriria implementar hashing personalizado pero que en Rust funciona directamente porque las tuplas de tipos que implementan `Hash` son automaticamente `Hash`. La funcion `get_normalized_slope` encapsula toda la logica de normalizacion: maneja los casos especiales de lineas verticales y horizontales primero, luego reduce la fraccion con el GCD y fuerza el signo positivo en `dx`. El bucle exterior solo necesita iterar `j` desde `i + 1` porque la pendiente de `i` a `j` es identica a la de `j` a `i` -- esto evita trabajo duplicado y el HashMap se reinicia en cada iteracion exterior, manteniendo el espacio en O(N).

## Conclusion

Este problema esconde una leccion importante sobre la representacion de datos. La tentacion natural de usar division flotante para calcular pendientes es precisamente la trampa que lo hace Hard. Al reemplazar un numero real con una fraccion irreducible normalizada, eliminamos toda ambiguedad y convertimos un problema de geometria continua en uno de conteo discreto con hash maps. La complejidad cuadratica es inevitable -- necesitamos comparar cada par de puntos -- pero el factor constante es bajo y la implementacion resulta limpia. A veces, la mejor forma de resolver un problema de geometria es negarse a hacer geometria y pensar en aritmetica de enteros.
