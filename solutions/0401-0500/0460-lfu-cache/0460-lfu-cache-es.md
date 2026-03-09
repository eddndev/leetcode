---
title: "0460 LFU Cache - ES"
problemUrl: "https://leetcode.com/problems/lfu-cache/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["hash-table", "linked-list", "design"]
complexity:
  time: "O(1)"
  space: "O(N)"
---

# El Arte de Olvidar lo que Menos Importa

## El Problema
Disenar e implementar una estructura de datos para una cache LFU (Least Frequently Used). Debe soportar las operaciones `get` y `put`, ambas en tiempo O(1). Cuando la cache alcanza su capacidad y se debe insertar una nueva clave, se desaloja la clave usada con menor frecuencia. Si hay un empate en frecuencia, se elimina la clave menos recientemente usada entre ellas.

## La Intuicion Inicial

Cuando me enfrente a este problema por primera vez, pense en su primo mas simple: la cache LRU. En LRU solo importa la recencia, y una unica lista doblemente enlazada junto con un hash map resuelve el asunto. Pero LFU introduce un segundo eje -- la frecuencia -- y eso lo cambia todo. Ahora necesitas saber no solo *cuando* se uso algo, sino *cuantas veces*.

Mi primer instinto fue usar un min-heap para extraer siempre el elemento con la menor frecuencia, pero eso empuja `get` y `put` a O(log N). El problema exige explicitamente O(1). Asi que me pregunte: como podemos rastrear la frecuencia minima sin un heap?

## La Estrategia del Doble Hash Map

La idea clave es mantener **dos hash maps** trabajando en conjunto con **listas doblemente enlazadas**:

1. **`key_map`**: Mapea cada clave directamente a su nodo en memoria. Esto nos da busqueda O(1) para cualquier clave.
2. **`freq_map`**: Mapea cada frecuencia a una lista doblemente enlazada que contiene todos los nodos con esa frecuencia. Dentro de cada lista, los nodos estan ordenados por recencia -- el mas recientemente usado esta en la cabeza.

Tambien mantenemos una unica variable `min_freq` que siempre contiene la frecuencia minima actual en la cache.

### Como Funciona `get`

Cuando llamamos a `get(key)`:
- Buscamos el nodo en `key_map`. Si no esta, retornamos -1.
- Removemos el nodo de su lista de frecuencia actual en `freq_map`.
- Si esa lista quedo vacia *y* su frecuencia era `min_freq`, incrementamos `min_freq`.
- Incrementamos la frecuencia del nodo y lo agregamos a la cabeza de la nueva lista de frecuencia.
- Retornamos el valor.

### Como Funciona `put`

Cuando llamamos a `put(key, value)`:
- Si la clave ya existe, actualizamos su valor y hacemos el mismo incremento de frecuencia que en `get`.
- Si la clave es nueva y estamos al limite de capacidad, encontramos la lista en `min_freq` y removemos el nodo de la cola (el menos recientemente usado entre los menos frecuentemente usados). Eliminamos esa clave de `key_map`.
- Creamos un nuevo nodo con frecuencia 1, lo agregamos a `key_map`, lo insertamos en la lista de frecuencia 1 y establecemos `min_freq = 1`.

La razon por la que siempre podemos establecer `min_freq = 1` al insertar una clave nueva es simple: una clave recien creada se ha usado exactamente una vez, y ninguna clave en la cache puede tener frecuencia menor a 1.

### Por Que Esto Es O(1)

Cada operacion individual -- busqueda en hash map, insercion en hash map, insercion en la cabeza de la lista, remocion de un nodo conocido -- es O(1). La variable `min_freq` elimina la necesidad de cualquier recorrido u ordenamiento. Las listas doblemente enlazadas dentro de cada grupo de frecuencia manejan el desempate por recencia automaticamente: la cola siempre es el mas antiguo, y los nuevos accesos van a la cabeza.

### Un Ejemplo Paso a Paso

Para una cache con capacidad 2:
- `put(1, 1)`: Cache = {1:1 (freq=1)}. `min_freq = 1`
- `put(2, 2)`: Cache = {1:1 (freq=1), 2:2 (freq=1)}. `min_freq = 1`. Lista freq-1: [2, 1]
- `get(1)` -> 1: La frecuencia de la clave 1 sube a 2. Lista freq-1: [2]. Lista freq-2: [1]. `min_freq = 1`
- `put(3, 3)`: Al limite. `min_freq = 1`, desalojamos la cola de la lista freq-1: clave 2. Cache = {1:1 (freq=2), 3:3 (freq=1)}. `min_freq = 1`
- `get(2)` -> -1: La clave 2 fue desalojada.
- `get(3)` -> 3: La frecuencia de la clave 3 sube a 2. `min_freq = 2`
- `put(4, 4)`: Al limite. `min_freq = 2`, ambas claves tienen freq=2. Desalojamos la cola de la lista freq-2: clave 1 (la menos recientemente usada). Cache = {3:3 (freq=2), 4:4 (freq=1)}. `min_freq = 1`

## Solucion en Rust

```rust
use std::cell::RefCell;
use std::collections::HashMap;
use std::ptr;
use std::rc::Rc;

struct Node {
    key: i32,
    val: i32,
    freq: i32,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new(key: i32, val: i32) -> *mut Node {
        Box::into_raw(Box::new(Node {
            key,
            val,
            freq: 1,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }
}

struct DList {
    head: *mut Node,
    tail: *mut Node,
    size: i32,
}

impl DList {
    fn new() -> Self {
        let head = Node::new(0, 0);
        let tail = Node::new(0, 0);
        unsafe {
            (*head).next = tail;
            (*tail).prev = head;
        }
        DList {
            head,
            tail,
            size: 0,
        }
    }

    fn add_to_head(&mut self, node: *mut Node) {
        unsafe {
            let next = (*self.head).next;
            (*node).prev = self.head;
            (*node).next = next;
            (*self.head).next = node;
            (*next).prev = node;
            self.size += 1;
        }
    }

    fn remove_node(&mut self, node: *mut Node) {
        unsafe {
            let prev = (*node).prev;
            let next = (*node).next;
            (*prev).next = next;
            (*next).prev = prev;
            self.size -= 1;
        }
    }

    fn remove_last(&mut self) -> *mut Node {
        if self.size == 0 {
            return ptr::null_mut();
        }
        unsafe {
            let last = (*self.tail).prev;
            self.remove_node(last);
            last
        }
    }
}

struct CacheCtx {
    cap: i32,
    min_freq: i32,
    key_map: HashMap<i32, *mut Node>,
    freq_map: HashMap<i32, DList>,
}

pub struct LFUCache {
    ctx: RefCell<CacheCtx>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            ctx: RefCell::new(CacheCtx {
                cap: capacity,
                min_freq: 0,
                key_map: HashMap::new(),
                freq_map: HashMap::new(),
            }),
        }
    }

    fn get(&self, key: i32) -> i32 {
        let mut ctx = self.ctx.borrow_mut();
        if !ctx.key_map.contains_key(&key) {
            return -1;
        }
        unsafe {
            let node = *ctx.key_map.get(&key).unwrap();
            Self::update(&mut ctx, node);
            (*node).val
        }
    }

    fn put(&self, key: i32, value: i32) {
        let mut ctx = self.ctx.borrow_mut();
        if ctx.cap == 0 {
            return;
        }

        unsafe {
            if let Some(&node) = ctx.key_map.get(&key) {
                (*node).val = value;
                Self::update(&mut ctx, node);
            } else {
                if ctx.key_map.len() as i32 == ctx.cap {
                    // CORRECCIÓN AQUÍ: Copiar min_freq antes de pedir el préstamo mutable
                    let min_freq = ctx.min_freq;
                    let min_list = ctx.freq_map.get_mut(&min_freq).unwrap();
                    let to_del = min_list.remove_last();

                    ctx.key_map.remove(&(*to_del).key);
                    // Reclamar memoria (opcional en CP pero correcto en Rust)
                    let _ = Box::from_raw(to_del);
                }
                let new_node = Node::new(key, value);
                ctx.key_map.insert(key, new_node);
                ctx.min_freq = 1;
                ctx.freq_map
                    .entry(1)
                    .or_insert(DList::new())
                    .add_to_head(new_node);
            }
        }
    }

    fn update(ctx: &mut CacheCtx, node: *mut Node) {
        unsafe {
            let freq = (*node).freq;
            let list = ctx.freq_map.get_mut(&freq).unwrap();
            list.remove_node(node);

            if list.size == 0 && freq == ctx.min_freq {
                ctx.min_freq += 1;
            }

            (*node).freq += 1;
            let new_freq = (*node).freq;
            ctx.freq_map
                .entry(new_freq)
                .or_insert(DList::new())
                .add_to_head(node);
        }
    }
}
```

La implementacion en Rust es donde las cosas se ponen interesantes. El modelo de ownership de Rust no acomoda naturalmente las listas doblemente enlazadas con acceso mutable compartido, asi que esta solucion se sumerge en territorio `unsafe` con punteros crudos (`*mut Node`). Los nodos se alocan en el heap via `Box::into_raw` y se liberan manualmente con `Box::from_raw` durante el desalojo. La estructura `DList` usa nodos centinela de cabeza y cola para simplificar la insercion y remocion en los extremos -- una tecnica clasica que elimina casos borde. Todo el estado de la cache vive dentro de un `RefCell<CacheCtx>` para permitir mutabilidad interior, lo cual es necesario porque `get` modifica logicamente la cache (incrementa la frecuencia) a pesar de tener una interfaz conceptualmente de solo lectura. La funcion `update` es el corazon del diseno: remueve el nodo de su lista de frecuencia anterior, verifica si `min_freq` necesita ajuste, y reinserta el nodo en el siguiente grupo de frecuencia -- todo en tiempo constante.

## Conclusion

La cache LFU es uno de esos problemas donde el enfoque ingenuo parece casi imposible de optimizar, pero la combinacion correcta de estructuras de datos hace que todo encaje. La arquitectura de doble hash map -- uno para acceso directo por clave, otro para grupos de frecuencia -- es la base. Las listas doblemente enlazadas dentro de cada grupo manejan el desempate por recencia. Y la variable `min_freq` es el atajo elegante que evita cualquier tipo de busqueda para el candidato a desalojo. Lo que mas me satisface de este diseno es como cada pieza existe por exactamente una razon, y eliminar cualquiera de ellas romperia la garantia de O(1). Es un recordatorio de que a veces los requisitos mas complejos no se resuelven con un unico truco ingenioso, sino con la orquestacion cuidadosa de componentes simples y bien entendidos.
