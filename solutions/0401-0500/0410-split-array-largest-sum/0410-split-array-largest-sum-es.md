---
title: "0410 Split Array Largest Sum - ES"
problemUrl: "https://leetcode.com/problems/split-array-largest-sum/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["binary-search", "greedy"]
complexity:
  time: "O(N * log(S)) donde N es la longitud del arreglo y S es la suma de todos los elementos"
  space: "O(1)"
---

# Repartiendo la Carga sin Romper la Balanza

## El Problema
Dado un arreglo de enteros `nums` y un entero `k`, dividir `nums` en `k` subarreglos contiguos no vacios de tal forma que la suma mas grande entre estos subarreglos sea minimizada. Devolver la suma mas grande minimizada.

## La Intuicion Inicial

Cuando me encontre por primera vez con este problema, mi mente salto a programacion dinamica -- probar todas las formas posibles de dividir el arreglo en `k` partes y rastrear el minimo del maximo de las sumas de los subarreglos. Ese enfoque funciona, pero su complejidad O(N^2 * k) se siente pesada. Entonces una perspectiva diferente hizo clic: realmente no estoy buscando como dividir. Estoy buscando un umbral -- el valor mas pequeno tal que el arreglo pueda dividirse en a lo sumo `k` subarreglos contiguos donde ninguno exceda ese umbral.

Una vez que formulo la pregunta como "puedo dividir el arreglo en a lo sumo `k` partes, cada una sumando a lo sumo `mid`?", el problema se transforma en una busqueda binaria sobre el espacio de respuestas. La respuesta debe estar entre el elemento maximo individual (porque cada subarreglo contiene al menos un elemento) y la suma total del arreglo (que es la suma cuando `k = 1`). Entre esos dos limites, puedo hacer busqueda binaria para encontrar el umbral mas ajustado que aun permita una division valida.

## La Configuracion de la Busqueda Binaria

Establezco `low` como el elemento maximo de `nums` y `high` como la suma total. Cualquier respuesta valida debe caer en este rango. Si pruebo un candidato `mid`, pregunto: "Es posible dividir `nums` en a lo sumo `k` subarreglos contiguos, cada uno con suma a lo sumo `mid`?" Si la respuesta es si, intento un umbral mas pequeno estableciendo `high = mid`. Si no, el umbral es demasiado ajustado y lo elevo con `low = mid + 1`.

Este es un patron clasico de busqueda binaria sobre la respuesta. La propiedad monotona es clara: si puedo dividir exitosamente con umbral `T`, entonces ciertamente puedo dividir con cualquier umbral mayor que `T`. Inversamente, si no puedo dividir con umbral `T`, no puedo dividir con nada menor tampoco.

## La Verificacion Greedy de Factibilidad

La funcion `can_split` es el corazon de la solucion. Dado el arreglo, el numero de subarreglos permitidos `k` y un candidato `limit`, construyo subarreglos de forma greedy de izquierda a derecha. Mantengo una suma acumulada y sigo agregando elementos al subarreglo actual. En el momento en que agregar el siguiente elemento empujaria la suma mas alla del limite, cierro el subarreglo actual y comienzo uno nuevo con ese elemento. Si en algun punto he abierto mas de `k` subarreglos, el limite es infactible y devuelvo `false`.

Esta estrategia greedy funciona porque siempre quiero empacar tantos elementos como sea posible en cada subarreglo antes de comenzar uno nuevo. No hay beneficio en cerrar un subarreglo tempranamente -- hacerlo solo forzaria mas subarreglos, nunca menos.

## Por Que Todo Encaja

La busqueda binaria hace O(log(S)) llamadas a `can_split`, donde S es la suma del arreglo. Cada llamada a `can_split` se ejecuta en O(N), realizando un unico recorrido lineal por el arreglo. Esto da una complejidad total de O(N * log(S)), que es notablemente eficiente para un problema que inicialmente podria parecer requerir busqueda exponencial. La complejidad espacial es O(1) ya que solo uso un punado de variables -- sin estructuras de datos auxiliares, sin tablas de memorizacion.

## Solucion en Rust

```rust
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut low = 0;
        let mut high = 0;

        for &num in nums.iter() {
            low = low.max(num);
            high += num;
        }

        while low < high {
            let mid = low + (high - low) / 2;

            if Self::can_split(&nums, k, mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        low
    }

    fn can_split(nums: &[i32], k: i32, limit: i32) -> bool {
        let mut count = 1;
        let mut current_sum = 0;

        for &num in nums {
            if current_sum + num > limit {
                count += 1;
                current_sum = num;

                if count > k {
                    return false;
                }
            } else {
                current_sum += num;
            }
        }

        true
    }
}
```

El bucle inicial calcula ambos limites simultaneamente: `low` acumula el elemento maximo y `high` acumula la suma total. La busqueda binaria usa `low + (high - low) / 2` en vez de `(low + high) / 2` para evitar desbordamiento de enteros. En `can_split`, el contador comienza en `1` porque el primer subarreglo siempre esta abierto desde el inicio. Cuando `current_sum + num` excede el limite, un nuevo subarreglo comienza con `num` como su primer elemento, y el contador se incrementa. El retorno temprano cuando `count > k` es una optimizacion que evita recorrer el resto del arreglo cuando la respuesta ya esta determinada. Cuando la busqueda binaria converge, `low` iguala a `high` y contiene la suma mas grande minimizada.

## Conclusion

Split Array Largest Sum es un ejemplo de libro de texto de busqueda binaria sobre el espacio de respuestas. La percepcion que transforma esto de un problema de combinatoria de particiones a una elegante busqueda binaria es reconocer que la pregunta "cual es la minima suma mas grande posible?" puede reformularse como "cual es el umbral mas pequeno que permite una division greedy valida?" La verificacion greedy de factibilidad proporciona el predicado monotono que la busqueda binaria requiere, y juntos entregan una solucion O(N * log(S)) que maneja las restricciones del problema con soltura. Lo que al principio parece exigir una enumeracion exhaustiva de particiones cede con gracia ante una simple busqueda sobre una recta numerica.
