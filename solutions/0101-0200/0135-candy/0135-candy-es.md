---
title: "0135 Candy - ES"
problemUrl: "https://leetcode.com/problems/candy/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["array", "greedy"]
complexity:
  time: "O(N)"
  space: "O(N)"
---

# Repartiendo Dulces con Justicia

## El Problema
Hay `n` ninos parados en fila. A cada nino se le asigna un valor de calificacion dado en el arreglo entero `ratings`. Queremos distribuir dulces a estos ninos siguiendo dos reglas: cada nino debe recibir al menos un dulce, y un nino con una calificacion mayor que la de un vecino inmediato debe recibir mas dulces que ese vecino. Devolver el numero minimo total de dulces necesarios.

## La Trampa de Querer Verlo Todo a la Vez

Cuando me encontre con este problema por primera vez, mi instinto fue intentar resolver todo en un solo recorrido: comparar cada nino con ambos vecinos simultaneamente y asignar dulces de inmediato. Pero eso no funciona. El conflicto es que la cantidad correcta de dulces para un nino depende de lo que pase tanto a su izquierda como a su derecha, y esas dos perspectivas pueden estar en tension. Un nino podria necesitar 3 dulces por su relacion con el vecino izquierdo pero 5 por su relacion con el derecho. No se pueden reconciliar ambas restricciones en un solo paso.

La clave estuvo en descomponer el problema en dos subproblemas independientes: primero satisfacer todas las restricciones mirando solo hacia la izquierda, y despues satisfacer todas las restricciones mirando solo hacia la derecha. Al final, combinar ambos resultados.

## Dos Pasadas: Izquierda y Derecha

La estrategia funciona asi:

1. **Inicializar:** Crear un arreglo `candies` del mismo tamano que `ratings`, con todos los valores en 1. Esto satisface la primera regla: cada nino recibe al menos un dulce.

2. **Pasada izquierda a derecha:** Recorrer desde el segundo nino hasta el ultimo. Si `ratings[i] > ratings[i - 1]`, entonces el nino `i` merece mas dulces que el nino `i - 1`, asi que asignamos `candies[i] = candies[i - 1] + 1`. Si la calificacion no es mayor, dejamos el valor en 1. Despues de esta pasada, todas las restricciones del tipo "un nino con mejor calificacion que su vecino izquierdo tiene mas dulces" quedan satisfechas.

3. **Pasada derecha a izquierda:** Recorrer desde el penultimo nino hasta el primero. Si `ratings[i] > ratings[i + 1]`, el nino `i` necesita mas dulces que el nino `i + 1`. Pero no podemos simplemente asignar `candies[i + 1] + 1`, porque eso podria violar la restriccion izquierda que ya satisficimos. La solucion es tomar el maximo: `candies[i] = max(candies[i], candies[i + 1] + 1)`. Asi preservamos lo que la primera pasada establecio y al mismo tiempo cumplimos la nueva restriccion.

4. **Sumar:** El total de dulces es la suma del arreglo.

### Un Ejemplo Concreto

Para `ratings = [1, 0, 2]`:
- Inicializamos: `candies = [1, 1, 1]`
- Pasada izquierda: `ratings[1]=0` no es mayor que `ratings[0]=1`, se queda. `ratings[2]=2 > ratings[1]=0`, asi que `candies[2] = candies[1] + 1 = 2`. Resultado: `[1, 1, 2]`
- Pasada derecha: `ratings[0]=1 > ratings[1]=0`, asi que `candies[0] = max(1, 1 + 1) = 2`. `ratings[1]=0` no es mayor que `ratings[2]=2`, se queda. Resultado: `[2, 1, 2]`
- Total: **5** dulces.

El nino del medio con calificacion 0 recibe solo 1 dulce, y ambos vecinos con calificaciones mayores reciben 2 cada uno. Las dos reglas quedan satisfechas con el minimo posible.

### Por que el Maximo es Correcto

El momento critico es el `max` en la segunda pasada. Sin el, podriamos destruir una restriccion ya satisfecha. Supongamos que despues de la pasada izquierda un nino tiene 4 dulces porque hay una secuencia creciente larga a su izquierda. Si en la pasada derecha descubrimos que necesita al menos 2 por su vecino derecho, no debemos bajar a 2; debemos quedarnos en 4. El maximo garantiza que ambas restricciones se cumplen simultaneamente: es la operacion que convierte dos soluciones parciales en una solucion global.

## Solucion en Rust

```rust
use std::cmp;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut candies = vec![1; n];

        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }

        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = cmp::max(candies[i], candies[i + 1] + 1);
            }
        }

        candies.iter().sum()
    }
}
```

La implementacion en Rust es concisa y directa. El `vec![1; n]` inicializa todos los dulces en 1 de forma idiomatica. La primera pasada recorre con `1..n`, y la segunda usa `(0..n - 1).rev()` para iterar en reversa, un patron elegante que Rust permite sin costo adicional gracias a los iteradores lazy. El uso de `cmp::max` en la segunda pasada es el unico punto donde ambas restricciones se encuentran y se reconcilian. Finalmente, `candies.iter().sum()` colapsa el arreglo en el resultado final aprovechando los traits de iteradores de Rust. No hay allocaciones innecesarias ni complejidad oculta: solo dos recorridos lineales y una suma.

## Conclusion

Este problema parece complicado porque cada nino depende de dos vecinos, creando un sistema de restricciones bidireccional. Pero la observacion fundamental es que las restricciones son separables por direccion. Al procesar primero todas las restricciones izquierdas y despues todas las derechas, transformamos un problema aparentemente global en dos problemas locales que se resuelven con recorridos lineales simples. El `max` al final actua como un operador de union que fusiona ambas soluciones parciales sin violar ninguna. Es una leccion clasica de la programacion greedy: cuando las restricciones son dificiles de satisfacer juntas, a veces basta con satisfacerlas por separado y combinarlas.
