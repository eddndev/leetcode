---
title: "0154 Find Minimum in Rotated Sorted Array II - ES"
problemUrl: "https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["array", "binary-search"]
complexity:
  time: "O(N) worst case, O(log N) average"
  space: "O(1)"
---

# Cazando la Aguja en un Pajar Retorcido

## El Problema
Dado un arreglo ordenado de enteros `nums` que ha sido rotado entre 1 y `n` veces, y que **puede contener duplicados**, encontrar el elemento minimo. El arreglo fue originalmente ordenado en orden ascendente y luego rotado en algun pivote desconocido.

## La Primera Impresion

La version clasica de este problema -- sin duplicados -- es un ejercicio de busqueda binaria de libro de texto. Comparas el elemento del medio con el de la derecha, y el lado donde vive el minimo se revela limpiamente. Pero al introducir duplicados, esa particion binaria ordenada se desmorona. Cuando `nums[mid] == nums[right]`, genuinamente no puedes saber cual mitad contiene el minimo. Esa ambiguedad es el alma entera de este problema.

Mi pensamiento inicial fue: ¿puedo aun rescatar la busqueda binaria? La respuesta es si, pero con una advertencia. En el peor caso -- piensa en un arreglo como `[1, 1, 1, 1, 1, 1, 1]` rotado a `[1, 1, 1, 1, 1, 1, 1]` -- ningun algoritmo puede hacerlo mejor que tiempo lineal, porque cada elemento luce identico y no tienes informacion para descartar ninguna porcion del arreglo. Lo mejor que podemos hacer es degradar con gracia: usar busqueda binaria cuando los duplicados nos lo permiten, y encoger la ventana de uno en uno cuando no.

## La Decision de Tres Vias

El algoritmo mantiene dos punteros, `left` y `right`, y examina repetidamente el elemento del medio. Hay exactamente tres casos:

### Caso 1: `nums[mid] > nums[right]`

El minimo debe estar en algun lugar a la derecha de `mid`. Si el medio es mayor que el elemento mas a la derecha, el punto de rotacion -- donde el arreglo pasa de su maximo de vuelta a su minimo -- se encuentra en el intervalo `(mid, right]`. Podemos mover con seguridad `left = mid + 1`.

### Caso 2: `nums[mid] < nums[right]`

El minimo esta en `mid` o a su izquierda. El subarreglo `[mid, right]` esta correctamente ordenado, asi que el valor mas pequeño en ese rango es `nums[mid]` mismo. Establecemos `right = mid`, manteniendo `mid` como candidato.

### Caso 3: `nums[mid] == nums[right]`

Este es el caso complicado. No podemos determinar en que lado esta el minimo. Consideremos `[3, 1, 3, 3, 3]` versus `[3, 3, 3, 1, 3]` -- ambos tienen `nums[mid] == nums[right] == 3`, pero el minimo esta en lados diferentes. El unico movimiento seguro es encoger el espacio de busqueda en uno: `right -= 1`. Perdemos a lo sumo un elemento que es igual a algo que ya vimos en `mid`, asi que no saltaremos accidentalmente el minimo.

### Terminacion

El bucle se ejecuta mientras `left < right`. Cuando se encuentran, `nums[left]` es la respuesta.

## Solucion en Rust

```rust
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] > nums[right] {
                left = mid + 1;
            } else if nums[mid] < nums[right] {
                right = mid;
            } else {
                right -= 1;
            }
        }

        nums[left]
    }
}
```

La implementacion es notablemente compacta. El calculo del punto medio `left + (right - left) / 2` evita el desbordamiento de enteros -- un habito que vale la pena construir incluso en Rust donde `usize` es grande. Las tres ramas se mapean directamente a los tres casos logicos descritos arriba. Notemos que nunca usamos `left = mid` (lo cual arriesgaria bucles infinitos con el redondeo hacia abajo de la division entera) -- el puntero `left` siempre avanza al menos en uno, y `right` siempre decrece al menos en uno, garantizando la convergencia. Como `nums` es un `Vec<i32>`, el indexado se verifica en modo debug, y el `nums[left]` final es seguro porque `left` permanece dentro de los limites durante toda la ejecucion.

## Conclusion

Este problema es una clase magistral en entender los limites de la busqueda binaria. Con elementos unicos, la busqueda binaria te da una garantia limpia de `O(log N)`. Los duplicados erosionan esa garantia al introducir ambiguedad en cada paso donde los valores coinciden. El respaldo `right -= 1` es elegante en su simplicidad -- es la concesion minima a la incertidumbre, preservando la estructura de busqueda binaria para cada paso donde el algoritmo *puede* hacer una division decisiva. El peor caso es `O(N)`, pero en la mayoria de las entradas con duplicados moderados, el algoritmo aun se ejecuta en tiempo logaritmico. Es un recordatorio de que a veces el mejor algoritmo no es uniformemente rapido -- es uno que es rapido cuando puede serlo y graciosamente lineal cuando debe serlo.
