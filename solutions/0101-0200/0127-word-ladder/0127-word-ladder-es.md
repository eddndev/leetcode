---
title: "0127 Word Ladder - ES"
problemUrl: "https://leetcode.com/problems/word-ladder/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["breadth-first-search", "hash-table", "string"]
complexity:
  time: "O(M^2 * N)"
  space: "O(M^2 * N)"
---

# Cruzando el Puente Una Letra a la Vez

## El Problema
Dadas dos palabras, `beginWord` y `endWord`, y un diccionario `wordList`, devolver el numero de palabras en la **secuencia de transformacion mas corta** desde `beginWord` hasta `endWord`, tal que cada par adyacente de palabras difiere en exactamente una letra, y cada palabra intermedia debe existir en `wordList`. Devolver `0` si no existe tal secuencia. Notar que `beginWord` no necesita estar en `wordList`.

## El Grafo Oculto

A primera vista, esto parece un rompecabezas de manipulacion de cadenas, pero hay un grafo escondido bajo la superficie. Cada palabra del diccionario es un nodo, y dos nodos estan conectados por una arista si y solo si difieren en exactamente un caracter. El problema entonces se convierte en: encontrar el camino mas corto desde `beginWord` hasta `endWord` en este grafo implicito.

En el momento en que reconoci esto como un problema de camino mas corto en un grafo sin pesos, el algoritmo se volvio obvio: **Busqueda en Anchura (BFS)**. BFS garantiza que la primera vez que alcanzamos un nodo, hemos encontrado el camino mas corto hacia el. No hay necesidad de Dijkstra, no hay necesidad de programacion dinamica -- solo una cola simple y expansion nivel por nivel.

## La Estrategia de Intercambio de Caracteres

La forma ingenua de encontrar vecinos seria comparar cada palabra contra todas las demas en el diccionario, verificando si difieren en exactamente una letra. Eso es O(N * M) por palabra, donde N es el tamano del diccionario y M es la longitud de la palabra. Para un diccionario grande, esto se vuelve costoso.

En su lugar, genero vecinos al vuelo: para cada posicion en la palabra actual, pruebo las 26 letras minusculas. Si la palabra resultante existe en el diccionario, es un vecino valido. Esto es O(26 * M) por palabra, que efectivamente es O(M) -- independiente del tamano del diccionario. El HashSet hace que la busqueda sea O(1) amortizado.

Hay un detalle sutil pero critico: una vez que una palabra se descubre como vecina, la **elimino del conjunto inmediatamente**, antes de agregarla a la cola. Esto cumple el mismo proposito que un conjunto de "visitados" pero es mas barato -- no necesitamos una estructura de datos separada, y evitamos que la misma palabra sea encolada multiples veces en diferentes niveles. Funciona porque BFS procesa nodos nivel por nivel, asi que la primera vez que encontramos una palabra esta garantizado que es a la distancia mas corta.

## Recorriendo un Ejemplo

Consideremos `beginWord = "hit"`, `endWord = "cog"`, `wordList = ["hot", "dot", "dog", "lot", "log", "cog"]`.

- **Nivel 1:** Comenzamos con `"hit"`. Probamos todos los cambios de un caracter. `"hot"` esta en el conjunto. La eliminamos, encolamos `("hot", 2)`.
- **Nivel 2:** Procesamos `"hot"`. Cambiar caracteres produce `"dot"` y `"lot"`. Ambas estan en el conjunto. Las eliminamos y encolamos ambas en nivel 3.
- **Nivel 3:** Procesamos `"dot"`. Produce `"dog"`. Encolamos en nivel 4. Procesamos `"lot"`. Produce `"log"`. Encolamos en nivel 4.
- **Nivel 4:** Procesamos `"dog"`. Produce `"cog"`. Coincide con `endWord`. Retornamos 5.

La respuesta es **5**: la secuencia `"hit" -> "hot" -> "dot" -> "dog" -> "cog"`.

### ¿Por que Eliminar en Vez de Marcar como Visitado?

Usar el conjunto de palabras como diccionario y rastreador de visitados a la vez es elegante por dos razones. Primero, elimina la necesidad de un HashSet separado de palabras visitadas, ahorrando memoria. Segundo, previene un error sutil: si solo verificamos pertenencia sin eliminar, multiples palabras en el mismo nivel de BFS podrian descubrir independientemente el mismo vecino, llevando a operaciones de encolado duplicadas y trabajo desperdiciado. Eliminar al descubrir asegura que cada palabra se procese exactamente una vez.

## Solucion en Rust

```rust
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // Convert the vector to a hashset for search
        let mut word_set: HashSet<String> = word_list.into_iter().collect();

        // If the final word doesn't exist, return 0
        if !word_set.contains(&end_word) {
            return 0;
        }

        // Init the queue for BFS
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        queue.push_back((begin_word, 1));

        while let Some((current_word, level)) = queue.pop_front() {
            // If end_word reached, return's the current level
            if current_word == end_word {
                return level;
            }

            let mut current_bytes = current_word.into_bytes();
            let len = current_bytes.len();

            for i in 0..len {
                let original_char = current_bytes[i];

                for c in b'a'..=b'z' {
                    if c == original_char {
                        continue;
                    }

                    current_bytes[i] = c;

                    if let Ok(next_word_str) = std::str::from_utf8(&current_bytes) {
                        if word_set.contains(next_word_str) {
                            word_set.remove(next_word_str);

                            queue.push_back((next_word_str.to_string(), level + 1));
                        }
                    }
                }

                current_bytes[i] = original_char;
            }
        }

        0
    }
}
```

La implementacion en Rust toma varias decisiones que se sienten naturales en el lenguaje. Convertir la palabra en un arreglo de bytes con `into_bytes()` nos permite manipular caracteres individuales como valores `u8`, iterando a traves de `b'a'..=b'z'` sin ninguna ceremonia de casteo. La verificacion con `std::str::from_utf8` es Rust siendo cauteloso -- dado que solo sustituimos letras ASCII minusculas en una cadena UTF-8 originalmente valida, esto siempre tendra exito, pero el sistema de tipos de Rust insiste en la validacion. El `word_set` funciona como diccionario y conjunto de visitados a la vez: cada palabra se elimina en el instante en que se descubre, lo cual es eficiente en memoria y correcto. El patron `while let Some(...)` drena el VecDeque de forma idiomatica, y el retorno anticipado cuando `current_word == end_word` cortocircuita la busqueda en el momento en que se encuentra el objetivo.

## Conclusion

Word Ladder es uno de esos problemas que te ensenan a ver grafos donde no los hay. Las palabras son nodos, las ediciones de un solo caracter son aristas, y la secuencia de transformacion mas corta es simplemente BFS en este grafo implicito. La idea clave de ingenieria es la estrategia de generacion de vecinos -- intercambiar cada posicion a traves del alfabeto y verificar contra un HashSet en lugar de comparar contra cada palabra del diccionario. Combinado con el truco de eliminar palabras del conjunto al descubrirlas para evitar revisitas, toda la solucion corre en O(M^2 * N), donde M es la longitud de la palabra y N es el tamano del diccionario, con BFS garantizando optimalidad por construccion.
