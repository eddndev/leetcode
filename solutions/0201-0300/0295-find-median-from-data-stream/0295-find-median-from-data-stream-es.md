---
title: "0295 Find Median from Data Stream - ES"
problemUrl: "https://leetcode.com/problems/find-median-from-data-stream/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["heap", "two-heaps", "design", "data-stream"]
complexity:
  time: "O(log N) por insercion, O(1) por consulta de mediana"
  space: "O(N) donde N es el numero total de elementos insertados"
---

# El Equilibrista entre Dos Montanas

## El Problema
Disenar una estructura de datos que soporte dos operaciones sobre un flujo de enteros: agregar un numero al flujo (`addNum`) y encontrar la mediana de todos los elementos vistos hasta el momento (`findMedian`). La mediana es el valor central cuando la lista esta ordenada; si la cantidad de elementos es par, es el promedio de los dos valores centrales.

## La Trampa de la Fuerza Bruta

La solucion mas directa seria mantener una lista ordenada e insertar cada nuevo elemento en su posicion correcta. Con busqueda binaria, encontrar la posicion correcta toma O(log N), pero desplazar los elementos para hacer espacio toma O(N). Consultar la mediana seria O(1) -- simplemente acceder al indice del medio -- pero con miles o millones de inserciones, el costo acumulado de O(N) por insercion se vuelve prohibitivo.

Otra idea seria no mantener el orden y simplemente ordenar al momento de consultar. Pero si `findMedian` se llama frecuentemente, estariamos ordenando repetidamente una lista que crece, lo cual es aun peor.

Lo que realmente necesitamos es una estructura que nos de acceso inmediato al "centro" del flujo sin necesidad de mantener todos los elementos ordenados.

## La Estrategia: Dos Heaps en Equilibrio

### La Observacion Clave

Me di cuenta de que para encontrar la mediana no necesito conocer el orden completo de todos los elementos. Solo necesito saber dos cosas: cual es el mayor de la mitad inferior y cual es el menor de la mitad superior. Si puedo mantener esos dos valores accesibles en O(1), la mediana se calcula trivialmente.

Esto me lleva a usar **dos heaps**:
- `small`: un max-heap que contiene la mitad inferior de los elementos. Su tope es el mayor de los "pequenos".
- `large`: un min-heap que contiene la mitad superior de los elementos. Su tope es el menor de los "grandes".

### El Protocolo de Insercion

Para cada nuevo numero, sigo estos pasos:

1. **Insertar en `small`**: empujo el nuevo numero al max-heap. Esto garantiza que el numero sera considerado como candidato de la mitad inferior.

2. **Transferir el maximo de `small` a `large`**: saco el tope de `small` (el mayor de la mitad inferior) y lo empujo a `large`. Esto asegura que todo elemento en `large` es mayor o igual a todo elemento en `small`.

3. **Rebalancear si es necesario**: si `large` tiene mas elementos que `small`, saco el minimo de `large` y lo devuelvo a `small`. Esto mantiene la invariante de que `small` tiene igual cantidad o uno mas que `large`.

Despues de estas tres operaciones, la invariante queda restaurada: `small.len() >= large.len()` y `small.len() - large.len() <= 1`.

### Consultar la Mediana

- Si `small` tiene mas elementos que `large`, la mediana es simplemente el tope de `small`.
- Si ambos tienen la misma cantidad, la mediana es el promedio del tope de `small` y el tope de `large`.

### Un Ejemplo Concreto

Insertando los numeros `[6, 10, 2, 6, 5, 0]`:

```
addNum(6):  small=[6], large=[]          -> mediana = 6.0
addNum(10): small=[6], large=[10]        -> mediana = (6+10)/2 = 8.0
addNum(2):  small=[6,2], large=[10]      -> mediana = 6.0
addNum(6):  small=[6,2], large=[6,10]    -> mediana = (6+6)/2 = 6.0
addNum(5):  small=[6,5,2], large=[6,10]  -> mediana = 6.0
addNum(0):  small=[5,2,0], large=[6,6,10] -> mediana = (5+6)/2 = 5.5
```

En cada paso, los heaps se rebalancean automaticamente, y la mediana siempre esta disponible consultando uno o dos topes.

### Por Que Cada Operacion Es Logaritmica

Cada insercion involucra como maximo tres operaciones de heap: un push en `small`, un pop de `small` seguido de un push en `large`, y posiblemente un pop de `large` seguido de un push en `small`. Cada operacion de heap es O(log N), asi que la insercion total es O(log N). La consulta de mediana solo accede a los topes de los heaps, lo cual es O(1).

## Solucion en Rust

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    small: BinaryHeap<i32>,
    large: BinaryHeap<Reverse<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        MedianFinder {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.small.push(num);

        if let Some(max_of_small) = self.small.pop() {
            self.large.push(Reverse(max_of_small));
        }


        if self.large.len() > self.small.len() {
            if let Some(Reverse(min_of_large)) = self.large.pop() {
                self.small.push(min_of_large);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.small.len() > self.large.len() {
            return *self.small.peek().unwrap() as f64;
        }

        let s = *self.small.peek().unwrap();
        let l = self.large.peek().unwrap().0;

        (s as f64 + l as f64) / 2.0
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
```

La implementacion en Rust aprovecha el `BinaryHeap` de la biblioteca estandar, que es un max-heap por defecto. Para simular un min-heap, utilizo el wrapper `Reverse` de `std::cmp`, que invierte el orden de comparacion. El campo `small` es un max-heap directo -- su `peek()` devuelve el mayor elemento de la mitad inferior. El campo `large` usa `BinaryHeap<Reverse<i32>>`, convirtiendo el max-heap nativo en un min-heap efectivo cuyo `peek()` devuelve el menor de la mitad superior. El protocolo de insercion siempre pasa el elemento por `small` primero, luego transfiere el maximo a `large`, y finalmente rebalancea si `large` crece demasiado. Los `unwrap()` en `find_median` son seguros porque esta funcion solo se llama despues de al menos una insercion, garantizando que `small` nunca esta vacio.

## Conclusion

Find Median from Data Stream es un problema clasico de diseno de estructuras de datos. La idea de dividir los elementos en dos mitades usando un par de heaps complementarios -- un max-heap para la mitad inferior y un min-heap para la superior -- transforma lo que parece requerir una lista ordenada completa en una operacion elegante de O(log N) por insercion y O(1) por consulta. Los heaps actuan como dos montanas opuestas cuyas cimas siempre apuntan al centro del flujo, y mantenerlos en equilibrio es todo lo que se necesita para que la mediana este siempre al alcance de la mano.
