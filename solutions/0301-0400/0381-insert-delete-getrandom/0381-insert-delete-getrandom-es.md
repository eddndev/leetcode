---
title: "0381 Insert Delete GetRandom O(1) Duplicates Allowed - ES"
problemUrl: "https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["hash-table", "design", "array", "randomized"]
complexity:
  time: "O(1) amortizado por insert, remove y getRandom"
  space: "O(N), donde N es el numero total de elementos en la coleccion"
---

# Malabarismo con Duplicados en Tiempo Constante

## El Problema
Disenar una estructura de datos `RandomizedCollection` que soporte tres operaciones en tiempo O(1) promedio: `insert(val)` que inserta un elemento `val` y retorna `true` si el elemento no estaba presente, `remove(val)` que elimina una instancia de `val` y retorna `true` si el elemento estaba presente, y `getRandom()` que retorna un elemento aleatorio de la coleccion donde la probabilidad de cada elemento es proporcional a la cantidad de veces que aparece.

## La Trampa de los Duplicados

Si este problema solo pidiera valores unicos, la solucion clasica seria un vector mas un HashMap que mapee cada valor a su indice. Pero los duplicados cambian todo. Un mismo valor puede aparecer multiples veces, y necesito rastrear todos sus indices para poder eliminar cualquiera de ellos en O(1). Ademas, `getRandom` debe respetar la frecuencia: si el numero 5 aparece tres veces y el numero 2 aparece una vez, el 5 debe ser tres veces mas probable de ser seleccionado.

## La Estrategia del Vector con Conjuntos de Indices

Mi enfoque combina dos estructuras:

1. **Un vector `nums`** que almacena todos los elementos, incluyendo duplicados. Esto hace que `getRandom` sea trivial: solo elijo un indice aleatorio uniforme.
2. **Un `HashMap<i32, HashSet<usize>>`** que para cada valor almacena el conjunto de todos los indices donde aparece en el vector.

La insercion es directa: agrego el valor al final del vector y registro su nuevo indice en el conjunto correspondiente. El truco esta en la eliminacion.

## La Eliminacion sin Huecos

Para eliminar en O(1), no puedo simplemente quitar un elemento del medio del vector, porque eso desplazaria todos los indices posteriores. En su lugar, uso la tecnica del intercambio con el ultimo elemento:

1. Elijo cualquier indice del valor a eliminar (tomo el primero que me da el iterador del `HashSet`).
2. Lo quito del conjunto de indices de ese valor.
3. Si ese indice no es el ultimo del vector, copio el ultimo elemento del vector en la posicion que quiero eliminar, y actualizo el conjunto de indices del ultimo elemento para reflejar su nueva posicion.
4. Hago `pop` del vector para eliminar la ultima posicion.

Este baile de intercambios garantiza que el vector siempre este compacto, sin huecos, y que cada operacion sea O(1) amortizado.

## Paso a Paso con un Ejemplo

Supongamos las operaciones: `insert(1)`, `insert(1)`, `insert(2)`, `remove(1)`, `getRandom()`.

- **insert(1)**: `nums = [1]`, indices: `{1: {0}}`. Retorna `true` (primera vez).
- **insert(1)**: `nums = [1, 1]`, indices: `{1: {0, 1}}`. Retorna `false` (ya existia).
- **insert(2)**: `nums = [1, 1, 2]`, indices: `{1: {0, 1}, 2: {2}}`. Retorna `true`.
- **remove(1)**: Elijo el indice 0 (o 1, depende del iterador). Supongamos que elijo 0. El ultimo elemento es `2` en indice 2. Copio `2` en posicion 0: `nums = [2, 1]`. Actualizo indices: `{1: {1}, 2: {0}}`. Retorna `true`.
- **getRandom()**: Con `nums = [2, 1]`, hay 50% de probabilidad para cada uno.

## Solucion en Rust

```rust
use rand::Rng;
use std::collections::{HashMap, HashSet};

struct RandomizedCollection {
    nums: Vec<i32>,
    indices: HashMap<i32, HashSet<usize>>,
}

impl RandomizedCollection {
    fn new() -> Self {
        RandomizedCollection {
            nums: Vec::new(),
            indices: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let is_present = self.indices.contains_key(&val) && !self.indices[&val].is_empty();

        self.nums.push(val);

        let new_idx = self.nums.len() - 1;
        self.indices
            .entry(val)
            .or_insert_with(HashSet::new)
            .insert(new_idx);

        !is_present
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(idxs) = self.indices.get_mut(&val) {
            if idxs.is_empty() {
                return false;
            }

            let remove_idx = *idxs.iter().next().unwrap();

            idxs.remove(&remove_idx);

            if idxs.is_empty() {
                self.indices.remove(&val);
            }

            let last_idx = self.nums.len() - 1;

            if remove_idx != last_idx {
                let last_val = self.nums[last_idx];

                self.nums[remove_idx] = last_val;

                if let Some(last_val_idxs) = self.indices.get_mut(&last_val) {
                    last_val_idxs.remove(&last_idx);
                    last_val_idxs.insert(remove_idx);
                }
            }

            self.nums.pop();

            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let random_idx = rng.gen_range(0..self.nums.len());
        self.nums[random_idx]
    }
}
```

La estructura `RandomizedCollection` mantiene un vector `nums` donde los duplicados conviven naturalmente, y un `HashMap` de conjuntos que rastrea donde vive cada valor. En `insert`, primero verifico si el valor ya existe para decidir el retorno booleano, luego lo agrego al final del vector y registro su indice. En `remove`, tomo un indice cualquiera del valor a eliminar, lo quito del conjunto, y si no era el ultimo elemento del vector, muevo el ultimo elemento a esa posicion actualizando sus indices. Finalmente, `get_random` simplemente genera un indice aleatorio uniforme sobre el vector, lo que automaticamente respeta las frecuencias de los duplicados ya que cada instancia ocupa su propia posicion.

## Conclusion

Insert Delete GetRandom O(1) con duplicados es un problema que parece sencillo hasta que los duplicados rompen las suposiciones del enfoque clasico. La clave es reconocer que un `HashSet` de indices por valor, combinado con la tecnica de intercambio con el ultimo elemento, preserva todas las garantias de tiempo constante. El vector nos da `getRandom` gratis con probabilidades correctas, y el mapa de conjuntos nos da la capacidad de localizar y eliminar cualquier instancia sin recorrer toda la estructura. Es un ejercicio elegante de como multiples estructuras de datos pueden colaborar para lograr lo que ninguna podria por si sola.
