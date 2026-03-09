---
title: "0315 Count of Smaller Numbers After Self - ES"
problemUrl: "https://leetcode.com/problems/count-of-smaller-numbers-after-self/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["merge-sort", "divide-and-conquer", "inversion-count"]
complexity:
  time: "O(N log N) donde N es la longitud del arreglo"
  space: "O(N) para el buffer temporal de mezcla y el rastreo de indices"
---

# Contando Inversiones por el Espejo Retrovisor

## El Problema
Dado un arreglo de enteros `nums`, devolver un nuevo arreglo `counts` donde `counts[i]` es la cantidad de elementos a la derecha de `nums[i]` que son estrictamente menores que `nums[i]`.

## La Tentacion de la Fuerza Bruta

El enfoque directo verifica, para cada elemento, todos los elementos a su derecha y cuenta cuantos son menores. Este doble bucle tiene un tiempo de ejecucion O(N^2), lo cual esta bien para entradas pequenas pero colapsa bajo la restriccion de hasta 100,000 elementos. Esencialmente estamos contando un tipo especifico de inversion: pares `(i, j)` donde `i < j` pero `nums[i] > nums[j]`. El detalle es que necesitamos atribuir cada inversion al elemento *izquierdo* del par y registrar esos conteos por posicion.

## La Estrategia: Merge Sort con Rastreo de Inversiones

### La Observacion Clave

Me di cuenta de que el merge sort revela inversiones de forma natural. Al mezclar dos mitades ordenadas, cada vez que un elemento de la mitad derecha se coloca antes que un elemento de la mitad izquierda, ese elemento derecho es menor que *todos los elementos restantes en la mitad izquierda*. Pero aqui necesito la perspectiva inversa: para cada elemento de la mitad izquierda, cuantos elementos de la mitad derecha fueron colocados antes que el? Ese conteo es exactamente el numero de elementos de la mitad derecha ya mezclados cuando el elemento de la mitad izquierda toma su turno.

La observacion fundamental es esta: durante el paso de mezcla, mantengo un contador `right_counter` que se incrementa cada vez que selecciono un elemento de la mitad derecha. Cuando finalmente selecciono un elemento de la mitad izquierda, `right_counter` me dice cuantos elementos de la mitad derecha son menores que el. Como ambas mitades estan ordenadas y todos los elementos de la mitad derecha aparecian originalmente a la derecha de todos los elementos de la mitad izquierda en el subarreglo actual, este conteo es precisamente el numero de elementos menores despues de si mismo para esa posicion.

### Por Que Funciona a Traves de los Niveles de Recursion

En cada nivel de recursion, el paso de mezcla solo cuenta inversiones entre las mitades izquierda y derecha -- no recuenta inversiones *dentro* de cada mitad, porque esas ya fueron contadas (y acumuladas) en llamadas recursivas mas profundas. Los conteos se acumulan aditivamente: el conteo de cada elemento crece a medida que se comparan elementos mas distantes en niveles superiores del arbol de recursion.

### Preservando los Indices Originales

Hay una complicacion practica: el merge sort reordena los elementos, pero necesito registrar los conteos en las posiciones *originales*. Resuelvo esto emparejando cada valor con su indice original, creando tuplas `(valor, indice_original)`. A medida que los elementos se mueven durante la ordenacion, sus indices originales viajan con ellos. Cuando agrego `right_counter` y lo atribuyo a un elemento de la mitad izquierda, escribo el conteo en `counts[indice_original]`, no en la posicion actual del elemento en el arreglo.

### Un Ejemplo Concreto

Con `nums = [5, 2, 6, 1]`:

```
Inicial: [(5,0), (2,1), (6,2), (1,3)]

Division: [(5,0), (2,1)] y [(6,2), (1,3)]

Sub-mezcla izquierda: [(5,0), (2,1)]
  Division: [(5,0)] y [(2,1)]
  Mezcla: 2 < 5, tomar (2,1), right_counter=1
          Restante izquierdo: (5,0) recibe right_counter=1 -> counts[0] += 1
  Resultado: [(2,1), (5,0)]      counts = [1, 0, 0, 0]

Sub-mezcla derecha: [(6,2), (1,3)]
  Division: [(6,2)] y [(1,3)]
  Mezcla: 1 < 6, tomar (1,3), right_counter=1
          Restante izquierdo: (6,2) recibe right_counter=1 -> counts[2] += 1
  Resultado: [(1,3), (6,2)]      counts = [1, 0, 1, 0]

Mezcla final: [(2,1), (5,0)] y [(1,3), (6,2)]
  i=0(izq), j=0(der): arr[j]=(1,3) < arr[i]=(2,1) -> tomar (1,3), right_counter=1
  arr[j]=(6,2) >= arr[i]=(2,1) -> tomar (2,1), counts[1] += 1 -> counts = [1, 1, 1, 0]
  arr[j]=(6,2) >= arr[i]=(5,0) -> tomar (5,0), counts[0] += 1 -> counts = [2, 1, 1, 0]
  Izquierda agotada, tomar (6,2).

Resultado: [(1,3), (2,1), (5,0), (6,2)]     counts = [2, 1, 1, 0]
```

La respuesta final `[2, 1, 1, 0]` coincide: a la derecha de 5 hay dos elementos menores (2 y 1), a la derecha de 2 hay uno (1), a la derecha de 6 hay uno (1), y a la derecha de 1 no hay ninguno.

## Solucion en Rust

```rust
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return vec![];
        }

        let mut indexed_nums: Vec<(i32, usize)> = nums
            .into_iter()
            .enumerate()
            .map(|(i, val)| (val, i))
            .collect();

        let mut counts = vec![0; n];

        Self::merge_sort(&mut indexed_nums, &mut counts);

        counts
    }

    fn merge_sort(arr: &mut [(i32, usize)], counts: &mut [i32]) {
        let mid = arr.len() / 2;
        if mid == 0 {
            return;
        }

        Self::merge_sort(&mut arr[0..mid], counts);
        Self::merge_sort(&mut arr[mid..], counts);

        Self::merge(arr, mid, counts);
    }

    fn merge(arr: &mut [(i32, usize)], mid: usize, counts: &mut [i32]) {
        let mut temp = Vec::with_capacity(arr.len());

        let mut i = 0;
        let mut j = mid;
        let mut right_counter = 0;

        while i < mid && j < arr.len() {
            if arr[j].0 < arr[i].0 {
                temp.push(arr[j]);
                right_counter += 1;
                j += 1;
            } else {
                counts[arr[i].1] += right_counter;
                temp.push(arr[i]);
                i += 1;
            }
        }

        while i < mid {
            counts[arr[i].1] += right_counter;
            temp.push(arr[i]);
            i += 1;
        }

        while j < arr.len() {
            temp.push(arr[j]);
            j += 1;
        }

        arr.copy_from_slice(&temp);
    }
}
```

La implementacion empareja cada valor con su indice original como tuplas `(i32, usize)`, luego realiza un merge sort in-place sobre el slice. La funcion `merge` asigna un buffer temporal del tamano del subarreglo actual y mezcla en el, luego copia de vuelta con `copy_from_slice`. La variable `right_counter` rastrea cuantos elementos de la mitad derecha han sido colocados hasta el momento; cuando un elemento de la mitad izquierda es finalmente colocado, `right_counter` se agrega a `counts[indice_original]`. Despues del bucle principal de mezcla, cualquier elemento restante de la mitad izquierda tambien recibe el `right_counter` completo, ya que todos los elementos de la mitad derecha (que son menores) ya fueron colocados. Los elementos restantes de la mitad derecha no necesitan actualizacion de conteo porque son mayores que todos los elementos de la mitad izquierda. El caso base `mid == 0` captura slices de un solo elemento y slices vacios, deteniendo la recursion. El uso de slices de Rust (`&mut arr[0..mid]`) evita asignar nuevos vectores en cada nivel de recursion, manteniendo la sobrecarga de espacio en O(N) total a traves de todas las operaciones de mezcla.

## Conclusion

Count of Smaller Numbers After Self es un problema clasico de conteo de inversiones disfrazado con ropas ligeramente diferentes. Al aumentar el merge sort para llevar los indices originales junto con los valores y rastrear cuantos elementos de la mitad derecha cruzan sobre cada elemento de la mitad izquierda durante la mezcla, contamos todas las inversiones relevantes en tiempo O(N log N) sin recurrir jamas a bucles anidados. La estructura de dividir y conquistar asegura que cada par se compare exactamente una vez, y la acumulacion aditiva de conteos a traves de los niveles de recursion produce la respuesta correcta por posicion.
