---
title: "0327 Count of Range Sum - ES"
problemUrl: "https://leetcode.com/problems/count-of-range-sum/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["merge-sort", "divide-and-conquer", "prefix-sum"]
complexity:
  time: "O(N log N) donde N es la longitud del arreglo"
  space: "O(N) para el arreglo de sumas prefijo y el buffer de mezcla"
---

# Cazando Sumas Entre las Lineas

## El Problema
Dado un arreglo de enteros `nums` y dos enteros `lower` y `upper`, devolver la cantidad de sumas de rango que se encuentran en `[lower, upper]` inclusive. Una suma de rango `S(i, j)` se define como la suma de los elementos en `nums` entre los indices `i` y `j` inclusive, donde `i <= j`.

## La Trampa de la Fuerza Bruta

La solucion ingenua itera sobre todos los pares `(i, j)` con `i <= j`, calcula la suma del subarreglo y verifica si cae dentro de `[lower, upper]`. Este enfoque O(N^2) (u O(N^3) si las sumas se recalculan desde cero cada vez) se desmorona rapidamente cuando `N` alcanza decenas de miles. Podemos optimizar el calculo de sumas usando sumas prefijo -- donde `S(i, j) = prefix[j+1] - prefix[i]` -- pero la enumeracion de pares en si sigue siendo cuadratica. Necesitamos una forma de contar pares validos sin examinar cada uno individualmente.

## La Estrategia: Merge Sort sobre Sumas Prefijo

### Construyendo la Base con Sumas Prefijo

Primero construyo un arreglo de sumas prefijo `sums` de longitud `N + 1`, donde `sums[0] = 0` y `sums[i+1] = sums[i] + nums[i]`. Con este arreglo, cualquier suma de rango `S(i, j)` es igual a `sums[j+1] - sums[i]`. El problema entonces se transforma en: contar la cantidad de pares `(i, j)` con `i < j` tales que `lower <= sums[j] - sums[i] <= upper`. Este es un problema de conteo estructurado sobre todos los pares de sumas prefijo.

### Por Que el Merge Sort Encaja Perfectamente

Me di cuenta de que el merge sort proporciona exactamente el marco necesario. Durante el paso de mezcla, cuando tengo dos mitades ordenadas, puedo contar eficientemente cuantos elementos en la mitad derecha satisfacen la restriccion de rango relativa a cada elemento en la mitad izquierda. Especificamente, para cada `sums[i]` en la mitad izquierda, quiero contar cuantos `sums[j]` en la mitad derecha satisfacen `lower <= sums[j] - sums[i] <= upper`, lo cual es equivalente a `sums[i] + lower <= sums[j] <= sums[i] + upper`.

Como la mitad derecha esta ordenada, puedo usar dos punteros `k` y `m` para encontrar la ventana de valores validos. Para cada `sums[i]` en la mitad izquierda, `k` avanza hasta la primera posicion donde `sums[k] - sums[i] >= lower`, y `m` avanza hasta la primera posicion donde `sums[m] - sums[i] > upper`. La cantidad de pares validos para este `sums[i]` es `m - k`. De forma crucial, a medida que `sums[i]` aumenta (ya que la mitad izquierda esta ordenada), tanto `k` como `m` solo pueden avanzar hacia adelante, por lo que el trabajo total de conteo a traves de todos los elementos en la mitad izquierda es lineal.

### El Conteo y la Mezcla en Tandem

Despues de contar, realizo la mezcla estandar para combinar las dos mitades en una sola secuencia ordenada. Esto es esencial: los niveles futuros de recursion necesitan el arreglo ordenado para que la tecnica de conteo con dos punteros siga siendo valida. El conteo y la mezcla son fases separadas dentro de cada paso de mezcla -- primero cuento los pares validos, luego mezclo los elementos.

### Un Ejemplo Concreto

Con `nums = [-2, 5, -1]`, `lower = -2`, `upper = 2`:

```
Sumas prefijo: [0, -2, 3, 2]

Division: [0, -2] y [3, 2]

Sub-mezcla izquierda: [0, -2]
  Division: [0] y [-2]
  Conteo: para sums[i]=0, buscar sums[j] en [-2+0, 2+0] = [-2, 2]
           sums[j]=-2, que esta en [-2, 2], count += 1
  Mezcla: [-2, 0]     conteo acumulado = 1

Sub-mezcla derecha: [3, 2]
  Division: [3] y [2]
  Conteo: para sums[i]=3, buscar sums[j] en [3+(-2), 3+2] = [1, 5]
           sums[j]=2, que esta en [1, 5], count += 1
  Mezcla: [2, 3]      conteo acumulado = 2

Mezcla final: [-2, 0] y [2, 3]
  Para sums[i]=-2: buscar sums[j] en [-2+(-2), -2+2] = [-4, 0]
    Ni 2 ni 3 estan en [-4, 0], count += 0
  Para sums[i]=0: buscar sums[j] en [0+(-2), 0+2] = [-2, 2]
    sums[j]=2 esta en [-2, 2], count += 1
  Mezcla: [-2, 0, 2, 3]    conteo acumulado = 3
```

La respuesta es 3, correspondiente a las sumas de rango: `S(0,0) = -2`, `S(2,2) = -1`, y `S(0,2) = 2`, todas dentro de `[-2, 2]`.

### Por Que la Recursion No Cuenta Doble

Cada nivel de recursion cuenta unicamente pares `(i, j)` donde `i` pertenece a la mitad izquierda y `j` pertenece a la mitad derecha del subarreglo actual. Los pares donde ambos indices estan en la mitad izquierda fueron contados en un nivel de recursion mas profundo, y de manera similar para la mitad derecha. La particion disjunta garantiza que cada par valido se cuente exactamente una vez a lo largo de todo el arbol de recursion.

## Solucion en Rust

```rust
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let n = nums.len();
        let mut sums = vec![0i64; n + 1];
        for i in 0..n {
            sums[i + 1] = sums[i] + nums[i] as i64;
        }

        let mut cache = vec![0i64; n + 1];

        Self::merge_sort_recursive(&mut sums, &mut cache, 0, n + 1, lower as i64, upper as i64)
    }

    fn merge_sort_recursive(
        sums: &mut [i64],
        cache: &mut [i64],
        left: usize,
        right: usize,
        lower: i64,
        upper: i64,
    ) -> i32 {
        if right - left <= 1 {
            return 0;
        }

        let mid = left + (right - left) / 2;

        let mut count = Self::merge_sort_recursive(sums, cache, left, mid, lower, upper)
            + Self::merge_sort_recursive(sums, cache, mid, right, lower, upper);

        let mut k = mid;
        let mut m = mid;

        for i in left..mid {
            while k < right && sums[k] - sums[i] < lower {
                k += 1;
            }
            while m < right && sums[m] - sums[i] <= upper {
                m += 1;
            }
            count += (m - k) as i32;
        }

        let mut i = left;
        let mut j = mid;
        let mut idx = 0;

        while i < mid && j < right {
            if sums[i] < sums[j] {
                cache[left + idx] = sums[i];
                i += 1;
            } else {
                cache[left + idx] = sums[j];
                j += 1;
            }
            idx += 1;
        }

        while i < mid {
            cache[left + idx] = sums[i];
            i += 1;
            idx += 1;
        }
        while j < right {
            cache[left + idx] = sums[j];
            j += 1;
            idx += 1;
        }

        sums[left..right].copy_from_slice(&cache[left..right]);

        count
    }
}
```

La implementacion comienza construyendo un arreglo de sumas prefijo usando `i64` para evitar desbordamiento por la acumulacion de valores `i32`. Un buffer `cache` del mismo tamano se asigna una sola vez y se reutiliza en todos los niveles de recursion, manteniendo el uso de espacio en O(N). La funcion recursiva `merge_sort_recursive` opera sobre el rango de indices `[left, right)`. El caso base retorna 0 cuando el rango contiene a lo sumo un elemento. Para cada elemento de la mitad izquierda, dos punteros `k` y `m` recorren la mitad derecha para encontrar la ventana donde `lower <= sums[j] - sums[i] <= upper`. Como ambas mitades estan ordenadas y los elementos de la mitad izquierda se iteran en orden, ambos punteros solo avanzan hacia adelante, haciendo que el paso de conteo sea O(N) por nivel de mezcla. Despues de contar, una mezcla estandar combina las dos mitades en orden a traves del buffer cache, y luego `copy_from_slice` escribe el resultado de vuelta en `sums`. El trabajo total es O(N log N) a traves de todos los niveles de recursion.

## Conclusion

Count of Range Sum es un hermoso ejemplo de como las sumas prefijo y el merge sort pueden combinarse para resolver lo que inicialmente parece un problema cuadratico intratable. Al reformular las sumas de rango como diferencias de sumas prefijo y aprovechar el orden que el merge sort proporciona, la tecnica de conteo con dos punteros reduce el trabajo en cada nivel de mezcla a tiempo lineal. La descomposicion recursiva asegura que cada par se cuente exactamente una vez, y el uso cuidadoso de aritmetica `i64` previene las trampas de desbordamiento que los valores de entrada `i32` podrian causar. El resultado es una solucion elegante O(N log N) que maneja el espacio completo de restricciones con facilidad.
