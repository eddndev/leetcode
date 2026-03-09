---
title: "0212 Word Search II - ES"
problemUrl: "https://leetcode.com/problems/word-search-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["trie", "backtracking", "depth-first-search", "matrix"]
complexity:
  time: "O(M * N * 4 * 3^(L-1)) donde L es la longitud maxima de palabra"
  space: "O(W * L) donde W es el numero de palabras"
---

# Cazando Palabras en un Laberinto de Letras

## El Problema
Dada una cuadricula `board` de `m x n` caracteres y una lista de palabras `words`, devolver todas las palabras de la lista que se pueden encontrar en la cuadricula. Cada palabra debe formarse a partir de letras en celdas adyacentes (horizontal o verticalmente), y la misma celda no puede usarse mas de una vez en una misma palabra.

## La Trampa de la Fuerza Bruta

La version ingenua seria tomar cada palabra, intentar formarla desde cada celda de la cuadricula mediante DFS, y verificar si se completa. Si tenemos `W` palabras de longitud maxima `L` en una cuadricula de `M x N`, eso nos da `W * M * N * 4 * 3^(L-1)` operaciones. Con miles de palabras, esto es inaceptable.

Mi primera observacion fue que muchas palabras comparten prefijos. Si "oath" y "oat" estan en la lista, el camino o-a-t se recorre dos veces con el enfoque ingenuo. Lo que necesitaba era una estructura que me permitiera buscar *todas* las palabras simultaneamente mientras recorro la cuadricula. Esa estructura es un **Trie**.

## La Estrategia: Trie + DFS Simultaneo

### El Trie como Mapa de Navegacion

La idea es construir un Trie con todas las palabras de la lista. Luego, para cada celda de la cuadricula, inicio un DFS pero en lugar de buscar una palabra especifica, *desciendo por el Trie en paralelo*. Si la celda actual contiene 'o' y el nodo actual del Trie tiene un hijo 'o', avanzo simultaneamente en la cuadricula y en el Trie. Si no existe ese hijo, podo la rama inmediatamente -- ninguna palabra de la lista puede comenzar con ese prefijo, asi que no hay razon para seguir explorando.

Esto transforma el problema de "buscar W palabras independientemente" a "recorrer la cuadricula una vez guiado por el Trie." Cada celda se explora en el contexto de los prefijos que aun son viables, y las ramas muertas se eliminan al instante.

### Recoleccion y Deduplicacion en un Solo Paso

Cuando el DFS llega a un nodo del Trie que contiene una palabra completa (almacenada directamente en el nodo), la recolecto en el resultado. Pero aqui hay un detalle: la misma palabra podria encontrarse por multiples caminos en la cuadricula. Para evitar duplicados sin usar un `HashSet`, uso `node.word.take()` -- extraigo la palabra del nodo y la reemplazo con `None`. Asi, la primera vez que la encuentro la recolecto, y cualquier intento posterior encuentra el nodo vacio. Deduplicacion gratuita integrada en la estructura.

### Marcado In-Place para Evitar Revisitas

Durante el DFS, necesito marcar las celdas visitadas para no reutilizarlas en la misma palabra. En lugar de mantener una matriz booleana separada, reemplazo temporalmente el caracter de la celda con `'#'`. Al retroceder en el backtracking, restauro el caracter original. Esto ahorra memoria y simplifica el codigo -- solo necesito comprobar `board[r][c] != '#'` para saber si una celda esta disponible.

### Un Ejemplo Concreto

Con `board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]]` y `words = ["oath","pea","eat","rain"]`:

```
Trie construido:
      root
      / \
     o   p    e    r
     |   |    |    |
     a   e    a    a
     |   |    |    |
     t   a    t    i
     |               |
     h               n

DFS desde (0,0)='o': hijo 'o' existe -> avanzar
  (1,0)='e': no hay hijo 'e' desde 'o' -> podar
  (0,1)='a': hijo 'a' existe -> avanzar
    (1,1)='t': hijo 't' existe -> avanzar
      (1,0)='e': no hay hijo 'e' desde 't' -> podar
      (2,1)='h': hijo 'h' existe -> nodo tiene word="oath" -> recolectar!
```

La palabra "oath" se encuentra en un solo recorrido. "eat" se encuentra comenzando desde (1,1) o (0,2). "pea" y "rain" no se encuentran en la cuadricula.

## La Navegacion por Wrapping

Un detalle interesante de esta implementacion es como maneja los limites de la cuadricula. En lugar de verificar `new_r >= 0` por separado (lo cual requeriria indices con signo), uso `wrapping_add` para sumar las direcciones. Si `r = 0` y `dr = -1`, entonces `0_usize.wrapping_add(-1_isize as usize)` produce `usize::MAX`, que siempre fallara la comparacion `new_r < board.len()`. Esto evita la necesidad de casteos o verificaciones adicionales -- los limites se comprueban con una sola comparacion por dimension.

## Solucion en Rust

```rust
use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    word: Option<String>,
}

impl TrieNode {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.word = Some(word);
    }
}
impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = TrieNode::new();

        for word in words {
            root.insert(word);
        }

        let rows = board.len();
        let cols = board[0].len();
        let mut result = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if let Some(next_node) = &mut root.children[char_to_idx(board[r][c])] {
                    dfs(&mut board, r, c, next_node, &mut result);
                }
            }
        }

        result
    }
}

#[inline(always)]
fn char_to_idx(c: char) -> usize {
    (c as u8 - b'a') as usize
}

fn dfs(
    board: &mut Vec<Vec<char>>,
    r: usize,
    c: usize,
    node: &mut TrieNode,
    result: &mut Vec<String>,
) {
    if let Some(w) = node.word.take() {
        result.push(w);
    }

    let original_char = board[r][c];
    board[r][c] = '#';

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dr, dc) in directions {
        let new_r = r.wrapping_add(dr as usize);
        let new_c = c.wrapping_add(dc as usize);

        if new_r < board.len() && new_c < board[0].len() {
            let next_char = board[new_r][new_c];
            if next_char != '#' {
                if let Some(next_node) = &mut node.children[char_to_idx(next_char)] {
                    dfs(board, new_r, new_c, next_node, result);
                }
            }
        }
    }
    board[r][c] = original_char;
}
```

La implementacion en Rust aprovecha el sistema de ownership de forma elegante. El Trie se construye con `Box<TrieNode>` para los nodos hijos, lo cual da estabilidad de punteros en el heap. El metodo `insert` navega el Trie con `get_or_insert_with`, creando nodos solo cuando son necesarios. La funcion `dfs` toma `&mut TrieNode` como referencia mutable, lo que permite usar `node.word.take()` para extraer la palabra encontrada sin necesidad de clonacion ni estructuras auxiliares -- `take()` reemplaza el `Option<String>` con `None` en el mismo nodo, deduplicando resultados de forma natural. El array fijo `children: [Option<Box<TrieNode>>; 26]` es mas eficiente que un `HashMap` para el alfabeto limitado de letras minusculas, ya que cada acceso es `O(1)` sin overhead de hashing. La anotacion `#[inline(always)]` en `char_to_idx` asegura que la conversion de caracter a indice se resuelve sin coste de llamada a funcion.

## Conclusion

Word Search II demuestra como una estructura de datos auxiliar puede transformar un problema de busqueda multiple en un recorrido unificado. El Trie actua como filtro de prefijos en tiempo real: cada paso del DFS que no corresponde a ningun prefijo viable se descarta inmediatamente, reduciendo drasticamente el espacio de busqueda. La combinacion de marcado in-place, extraccion destructiva con `take()`, y navegacion por wrapping produce una solucion que es tanto eficiente como concisa -- sin estructuras auxiliares para deduplicacion, sin matrices booleanas para el marcado, y sin casteos para el manejo de bordes.
