---
title: "0042 Trapping Rain Water - ES"
problemUrl: "https://leetcode.com/problems/trapping-rain-water/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["array", "two-pointers"]
complexity:
  time: "O(N)"
  space: "O(1)"
---

# Atrapando la Lluvia entre Muros

## El Problema
Dado un arreglo de enteros no negativos `height` donde cada elemento representa la altura de una barra de ancho 1, calcular cuanta agua puede quedar atrapada entre las barras despues de llover.

## La Intuicion Inicial

Cuando vi este problema por primera vez, mi mente fue directamente a la fuerza bruta: para cada posicion, mirar cual es la barra mas alta a su izquierda y la mas alta a su derecha, y el agua que cabe ahi es el minimo de esas dos alturas menos la altura de la barra actual. Es correcto, pero requiere recorrer el arreglo completo para cada posicion, lo cual da O(N^2).

El siguiente paso natural es precomputar esos maximos con dos arreglos auxiliares: uno que guarde el maximo desde la izquierda y otro desde la derecha. Eso reduce el tiempo a O(N), pero usa O(N) de espacio extra. Me pregunte: se puede hacer sin esos arreglos auxiliares? Y la respuesta es si, con **dos punteros**.

## La Estrategia de Dos Punteros

La observacion clave es esta: no necesitamos conocer ambos maximos al mismo tiempo. Solo necesitamos saber que el lado opuesto tiene una barra lo suficientemente alta como para "sostener" el agua.

Colocamos un puntero `left` al inicio y otro `right` al final del arreglo. Tambien mantenemos dos variables: `left_max` y `right_max`, que rastrean la barra mas alta vista desde cada extremo.

En cada paso, comparamos `height[left]` con `height[right]`:

1. **Si `height[left] < height[right]`:** Sabemos que el lado derecho tiene al menos una barra tan alta como `height[right]`, que es mayor que `height[left]`. Por lo tanto, el agua en la posicion `left` esta determinada unicamente por `left_max`. Si `height[left] >= left_max`, actualizamos `left_max`. Si no, la diferencia `left_max - height[left]` es agua atrapada. Avanzamos `left`.

2. **Si `height[left] >= height[right]`:** El razonamiento es simetrico. El lado izquierdo garantiza que hay una barra lo suficientemente alta, asi que el agua en `right` depende solo de `right_max`. Si `height[right] >= right_max`, actualizamos `right_max`. Si no, sumamos `right_max - height[right]`. Retrocedemos `right`.

Lo elegante es que nunca necesitamos mirar hacia atras ni precomputar nada. El hecho de que uno de los dos lados siempre tenga una barra mas alta que la posicion actual nos da la garantia que necesitamos para acumular agua con confianza.

### Un Ejemplo Paso a Paso

Para `height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]`:
- Iniciamos con `left = 0`, `right = 11`, `left_max = 0`, `right_max = 0`, `water = 0`
- `height[0]=0 < height[11]=1`: `left_max = 0`, no hay agua. `left = 1`
- `height[1]=1 < height[11]=1`? No, son iguales, vamos al else: `right_max = 1`. `right = 10`
- `height[1]=1 < height[10]=2`: `left_max = 1`. `left = 2`
- `height[2]=0 < height[10]=2`: `0 < left_max(1)`, water += 1. `left = 3`
- `height[3]=2 >= height[10]=2`: `right_max = max(1, 2) = 2`. `right = 9`
- `height[3]=2 >= height[9]=1`: `1 < right_max(2)`, water += 1. `right = 8`
- Y asi continua hasta que `left` y `right` se cruzan, acumulando un total de **6** unidades de agua.

## Solucion en Rust

```rust
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }

        let mut left = 0;
        let mut right = height.len() - 1;

        let mut left_max = 0;
        let mut right_max = 0;

        let mut water = 0;

        while left < right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    water += left_max - height[left];
                }
                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    water += right_max - height[right];
                }
                right -= 1;
            }
        }

        water
    }
}
```

La implementacion en Rust es directa y limpia. La guarda inicial `height.len() < 3` es un detalle practico: con menos de tres barras es imposible atrapar agua, y ademas evita un posible underflow en `height.len() - 1` cuando el vector esta vacio (ya que `len()` devuelve `usize`, un tipo sin signo). Los dos punteros se manejan con indices simples, y toda la logica cabe en un solo `while` con una bifurcacion clara. No hay allocaciones, no hay estructuras auxiliares: solo cuatro variables escalares y el arreglo de entrada.

## Conclusion

Este problema es un clasico que demuestra el poder de la tecnica de dos punteros. La clave esta en darse cuenta de que no necesitamos informacion global para tomar decisiones locales: basta con saber que el lado opuesto tiene una pared lo suficientemente alta. Esa observacion es lo que nos permite pasar de O(N) en espacio a O(1), eliminando los arreglos de maximos precalculados sin perder nada de informacion. A veces, la solucion mas eficiente no viene de agregar mas estructura, sino de darse cuenta de que ya tenemos todo lo que necesitamos.
