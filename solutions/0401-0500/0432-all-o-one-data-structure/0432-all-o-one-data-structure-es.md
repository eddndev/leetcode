---
title: "0432 All O`one Data Structure - ES"
problemUrl: "https://leetcode.com/problems/all-oone-data-structure/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["hash-table", "linked-list", "design", "doubly-linked-list"]
complexity:
  time: "O(1) por cada operacion inc, dec, getMaxKey y getMinKey"
  space: "O(N), donde N es el numero de claves distintas en la estructura"
---

# El Libro de Cuentas que Nunca Pierde el Hilo

## El Problema
Disenar una estructura de datos `AllOne` que soporte cuatro operaciones, todas en tiempo O(1): `inc(key)` que incrementa el conteo de la clave `key` en 1 (o la inserta con conteo 1 si no existe), `dec(key)` que decrementa el conteo de `key` en 1 (eliminandola si llega a 0), `getMaxKey()` que retorna cualquier clave con el conteo maximo, y `getMinKey()` que retorna cualquier clave con el conteo minimo.

## Por Que un HashMap Solo No Alcanza

Mi primer instinto fue usar un simple `HashMap<String, i32>` para rastrear los conteos. Incrementar y decrementar seria O(1), perfecto. Pero el problema exige que `getMaxKey` y `getMinKey` tambien sean O(1), y encontrar el maximo o minimo en un HashMap requiere recorrer todos los valores. Necesito una estructura adicional que mantenga los conteos ordenados y me de acceso instantaneo a los extremos.

## La Lista Doblemente Enlazada de Frecuencias

La idea central es combinar un `HashMap` con una lista doblemente enlazada donde cada nodo representa un conteo especifico y almacena el conjunto de claves que tienen ese conteo. Los nodos estan ordenados de menor a mayor conteo, y la lista tiene centinelas en ambos extremos: un nodo `head` con conteo 0 y un nodo `tail` con conteo `i32::MAX`. Esto elimina los casos borde al insertar o eliminar nodos.

Con esta estructura:
- `getMinKey` simplemente mira el primer nodo real despues de `head`.
- `getMaxKey` mira el ultimo nodo real antes de `tail`.
- Ambas operaciones son O(1) porque solo sigo punteros.

## El Baile de Incrementar y Decrementar

Cuando llamo a `inc(key)`, hay dos escenarios:

1. **La clave es nueva**: Busco el nodo con conteo 1 justo despues de `head`. Si no existe, lo creo. Agrego la clave a ese nodo y la registro en el HashMap apuntando a ese nodo.
2. **La clave ya existe**: La encuentro en su nodo actual con conteo `c`. Busco si el siguiente nodo tiene conteo `c + 1`. Si no, creo un nuevo nodo entre el actual y el siguiente. Muevo la clave al nodo de conteo `c + 1`. Si el nodo de conteo `c` queda vacio, lo elimino.

`dec(key)` es el espejo: si el conteo baja a 0, simplemente elimino la clave del HashMap. Si no, busco o creo el nodo con conteo `c - 1` justo antes del nodo actual. Muevo la clave y limpio el nodo viejo si queda vacio.

Cada una de estas operaciones toca a lo sumo un par de nodos adyacentes en la lista, por lo que todo es O(1).

## Paso a Paso con un Ejemplo

Supongamos las operaciones: `inc("a")`, `inc("b")`, `inc("a")`, `getMinKey()`, `getMaxKey()`, `dec("a")`.

- **inc("a")**: Creo nodo con conteo 1, agrego "a". Lista: `head <-> [1: {a}] <-> tail`.
- **inc("b")**: El nodo con conteo 1 ya existe, agrego "b". Lista: `head <-> [1: {a, b}] <-> tail`.
- **inc("a")**: Muevo "a" del nodo 1 al nodo 2 (que creo). Lista: `head <-> [1: {b}] <-> [2: {a}] <-> tail`.
- **getMinKey()**: El nodo despues de `head` tiene conteo 1, retorno "b".
- **getMaxKey()**: El nodo antes de `tail` tiene conteo 2, retorno "a".
- **dec("a")**: Muevo "a" del nodo 2 al nodo 1. El nodo 2 queda vacio, lo elimino. Lista: `head <-> [1: {b, a}] <-> tail`.

## Solucion en Rust

```rust
use std::collections::{HashMap, HashSet};
use std::ptr;

struct Node {
    cnt: i32,
    keys: HashSet<String>,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new(cnt: i32) -> *mut Node {
        Box::into_raw(Box::new(Node {
            cnt,
            keys: HashSet::new(),
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }
}

pub struct AllOne {
    map: HashMap<String, *mut Node>,
    head: *mut Node,
    tail: *mut Node,
}

impl AllOne {
    fn new() -> Self {
        let head = Node::new(0);
        let tail = Node::new(i32::MAX);
        unsafe {
            (*head).next = tail;
            (*tail).prev = head;
        }
        AllOne {
            map: HashMap::new(),
            head,
            tail,
        }
    }

    unsafe fn add_node(&self, prev: *mut Node, cnt: i32) -> *mut Node {
        let node = Node::new(cnt);
        let next = (*prev).next;
        (*node).prev = prev;
        (*node).next = next;
        (*prev).next = node;
        (*next).prev = node;
        node
    }

    unsafe fn remove_node(&self, node: *mut Node) {
        let prev = (*node).prev;
        let next = (*node).next;
        (*prev).next = next;
        (*next).prev = prev;
        let _ = Box::from_raw(node);
    }

    fn inc(&mut self, key: String) {
        unsafe {
            if let Some(&cur) = self.map.get(&key) {
                let cnt = (*cur).cnt;
                let next = (*cur).next;
                let target = if (*next).cnt == cnt + 1 {
                    next
                } else {
                    self.add_node(cur, cnt + 1)
                };
                (*target).keys.insert(key.clone());
                self.map.insert(key.clone(), target);
                (*cur).keys.remove(&key);
                if (*cur).keys.is_empty() {
                    self.remove_node(cur);
                }
            } else {
                let first = (*self.head).next;
                let target = if (*first).cnt == 1 {
                    first
                } else {
                    self.add_node(self.head, 1)
                };
                (*target).keys.insert(key.clone());
                self.map.insert(key, target);
            }
        }
    }

    fn dec(&mut self, key: String) {
        unsafe {
            if let Some(&cur) = self.map.get(&key) {
                let cnt = (*cur).cnt;
                if cnt > 1 {
                    let prev = (*cur).prev;
                    let target = if (*prev).cnt == cnt - 1 {
                        prev
                    } else {
                        self.add_node((*cur).prev, cnt - 1)
                    };
                    (*target).keys.insert(key.clone());
                    self.map.insert(key.clone(), target);
                } else {
                    self.map.remove(&key);
                }
                (*cur).keys.remove(&key);
                if (*cur).keys.is_empty() {
                    self.remove_node(cur);
                }
            }
        }
    }

    fn get_max_key(&self) -> String {
        unsafe {
            let last = (*self.tail).prev;
            if last == self.head {
                return "".to_string();
            }
            (*last).keys.iter().next().cloned().unwrap_or_default()
        }
    }

    fn get_min_key(&self) -> String {
        unsafe {
            let first = (*self.head).next;
            if first == self.tail {
                return "".to_string();
            }
            (*first).keys.iter().next().cloned().unwrap_or_default()
        }
    }
}
```

La implementacion usa punteros crudos (`*mut Node`) para construir la lista doblemente enlazada, lo cual en Rust requiere bloques `unsafe`. Cada nodo almacena su conteo, un `HashSet` de claves, y punteros al nodo anterior y siguiente. Los centinelas `head` y `tail` simplifican toda la logica de insercion y eliminacion de nodos, eliminando la necesidad de verificar si estamos en los bordes de la lista. El `HashMap` mapea cada clave directamente al nodo donde vive, lo que permite localizar cualquier clave en O(1) y moverla al nodo adyacente en la misma complejidad. Cuando un nodo se queda sin claves, `remove_node` lo desenlaza de la lista y libera su memoria con `Box::from_raw`.

## Conclusion

All O`one Data Structure es un problema que pone a prueba la capacidad de disenar estructuras compuestas donde cada pieza tiene un proposito preciso. El HashMap da acceso O(1) a la ubicacion de cada clave, la lista enlazada mantiene los conteos ordenados con acceso O(1) a los extremos, y los conjuntos dentro de cada nodo agrupan claves con el mismo conteo para que mover una clave entre frecuencias adyacentes sea una operacion constante. Es la orquestacion perfecta entre tres estructuras de datos que, trabajando juntas, logran lo que individualmente seria imposible: cuatro operaciones en O(1) sobre un diccionario de frecuencias dinamico.
