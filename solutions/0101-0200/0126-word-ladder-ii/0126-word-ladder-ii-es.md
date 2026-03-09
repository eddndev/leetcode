---
title: "0126 Word Ladder II - ES"
problemUrl: "https://leetcode.com/problems/word-ladder-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["breadth-first-search", "hash-table", "string", "backtracking"]
complexity:
  time: "O(N * M * 26)"
  space: "O(N * M)"
---

# Cartografiando Todos los Atajos entre Palabras

## El Problema
Dada una palabra inicial `beginWord`, una palabra final `endWord`, y una lista de palabras `wordList`, devolver todas las secuencias de transformacion mas cortas desde `beginWord` hasta `endWord`, donde cada paso cambia exactamente una letra y cada palabra intermedia debe existir en `wordList`. Si no existe ninguna transformacion posible, devolver una lista vacia.

## La Complejidad Oculta

Este problema es la version despiadada de Word Ladder I. En el primer problema solo necesitamos la *longitud* de la transformacion mas corta. Aqui necesitamos *todas* las secuencias que alcanzan esa longitud minima. Eso cambia todo.

Mi primer impulso fue hacer un BFS clasico y registrar los caminos completos en la cola. Pero almacenar caminos enteros en cada nodo del BFS es un desastre de memoria -- cada camino crece con cada nivel, y el numero de caminos puede ser exponencial. La clave esta en separar la exploracion de la reconstruccion: primero usar BFS para mapear la estructura del grafo mas corto, y luego usar backtracking para recorrerlo.

## Dos Fases: BFS para Descubrir, Backtracking para Reconstruir

### Fase 1: BFS por Capas

La idea fundamental es realizar un BFS nivel por nivel, no palabra por palabra. En cada nivel, procesamos *todas* las palabras de la capa actual antes de avanzar. Esto tiene una consecuencia crucial: si una palabra aparece en el nivel 3, sabemos que su distancia desde `beginWord` es exactamente 3. Si la encontramos de nuevo en el nivel 4, ya no nos interesa -- eso no producira un camino mas corto.

Para cada palabra nueva que descubrimos, en lugar de registrar "de donde vine" como un solo padre, registramos *todos* los padres del nivel actual. Si las palabras "hot" y "dot" ambas pueden llegar a "dog" en el mismo nivel, entonces "dog" tiene dos padres. Esa es la informacion que necesitamos para reconstruir multiples caminos.

Un detalle sutil: eliminamos las palabras del `word_set` *al inicio* de cada nivel, no cuando las descubrimos individualmente. Si eliminaramos "dog" al encontrarlo desde "hot", entonces "dot" nunca lo registraria como hijo. Al eliminar por capas completas, permitimos que todas las palabras del nivel actual contribuyan sus conexiones antes de cerrar la puerta.

### Fase 2: Backtracking desde el Final

Una vez que el BFS encuentra `endWord`, tenemos un mapa de padres que codifica un DAG (grafo aciclico dirigido) de caminos minimos. Reconstruimos los caminos caminando hacia atras desde `endWord` hasta `beginWord` usando el mapa de padres, acumulando cada camino y revertiendolo al llegar al origen.

### Un Ejemplo Concreto

Con `beginWord = "hit"`, `endWord = "cog"`, `wordList = ["hot","dot","dog","lot","log","cog"]`:

```
Nivel 0: {hit}
Nivel 1: {hot}          padres: hot <- [hit]
Nivel 2: {dot, lot}     padres: dot <- [hot], lot <- [hot]
Nivel 3: {dog, log}     padres: dog <- [dot], log <- [dot, lot]
Nivel 4: {cog}          padres: cog <- [dog, log]
```

El backtracking desde "cog" reconstruye:
- cog -> dog -> dot -> hot -> hit
- cog -> log -> dot -> hot -> hit
- cog -> log -> lot -> hot -> hit

Cada uno se revierte para dar el camino final de izquierda a derecha.

## ¿Por que Eliminar por Capas y no Individualmente?

Este es el punto mas delicado del algoritmo. Si eliminamos cada palabra del conjunto global en el momento en que la descubrimos, impedimos que otras palabras *del mismo nivel* la descubran tambien. Pero dos caminos minimos pueden pasar por la misma palabra en el mismo nivel. Al procesar la eliminacion por capas completas, garantizamos que todas las aristas del grafo de caminos minimos se capturen en el mapa de padres.

## Solucion en Rust

```rust
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut word_set: HashSet<String> = word_list.into_iter().collect();

        if !word_set.contains(&end_word) {
            return vec![];
        }

        let mut layer: HashSet<String> = HashSet::new();
        layer.insert(begin_word.clone());

        let mut parents: HashMap<String, Vec<String>> = HashMap::new();

        let mut found = false;

        while !layer.is_empty() && !found {
            for w in &layer {
                word_set.remove(w);
            }

            let mut next_layer: HashSet<String> = HashSet::new();

            for word in &layer {
                let mut chars: Vec<char> = word.chars().collect();

                for i in 0..chars.len() {
                    let old_char = chars[i];

                    for c in 'a'..='z' {
                        if c == old_char {
                            continue;
                        }

                        chars[i] = c;
                        let new_word: String = chars.iter().collect();

                        if word_set.contains(&new_word) {
                            if new_word == end_word {
                                found = true;
                            }

                            next_layer.insert(new_word.clone());

                            parents
                                .entry(new_word)
                                .or_insert(Vec::new())
                                .push(word.clone());
                        }
                    }
                    chars[i] = old_char;
                }
            }
            layer = next_layer;
        }

        let mut result = Vec::new();
        if found {
            let mut current_path = vec![end_word.clone()];
            Self::backtrack(
                &end_word,
                &begin_word,
                &parents,
                &mut current_path,
                &mut result,
            );
        }

        result
    }

    fn backtrack(
        current: &String,
        target: &String,
        parents: &HashMap<String, Vec<String>>,
        path: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        if current == target {
            let mut full_path = path.clone();
            full_path.reverse();
            result.push(full_path);
            return;
        }

        if let Some(parent_list) = parents.get(current) {
            for parent in parent_list {
                path.push(parent.clone());
                Self::backtrack(parent, target, parents, path, result);
                path.pop();
            }
        }
    }
}
```

La implementacion en Rust separa las dos fases de forma nitida. El bucle `while` principal implementa el BFS por capas: al inicio de cada iteracion, todas las palabras de la capa actual se eliminan de `word_set`, asegurando que no se revisiten en niveles posteriores pero si se puedan conectar entre si dentro del mismo nivel. El `HashMap<String, Vec<String>>` de padres acumula *todas* las conexiones validas en caminos minimos. La bandera `found` permite que el BFS complete el nivel actual antes de detenerse -- esto es vital porque multiples caminos minimos pueden alcanzar `endWord` desde diferentes palabras del mismo nivel. La funcion `backtrack` recorre el mapa de padres en sentido inverso, construyendo caminos desde `endWord` hacia `beginWord` y revertiendolos al final. El patron clasico de `push`/`pop` en el backtracking reutiliza el mismo vector de camino, evitando asignaciones innecesarias.

## Conclusion

Word Ladder II ilustra una leccion fundamental sobre BFS en grafos: cuando necesitamos *todos* los caminos minimos y no solo uno, la exploracion y la reconstruccion deben vivir en fases separadas. El BFS por capas construye un DAG implicito de relaciones padre-hijo respetando las distancias minimas, y el backtracking posterior lo recorre exhaustivamente. El truco critico -- eliminar palabras del conjunto por capas completas en lugar de individualmente -- es lo que permite capturar la totalidad de los caminos optimos sin sacrificar la garantia de minimalidad.
