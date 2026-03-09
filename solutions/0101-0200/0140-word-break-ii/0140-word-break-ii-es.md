---
title: "0140 Word Break II - ES"
problemUrl: "https://leetcode.com/problems/word-break-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "backtracking", "memoization", "hash-table", "string"]
complexity:
  time: "O(N * 2^N)"
  space: "O(N * 2^N)"
---

# Dividiendo una Cadena en Todas las Oraciones Posibles

## El Problema
Dada una cadena `s` y un diccionario de palabras `wordDict`, agregar espacios en `s` para construir oraciones donde cada palabra sea una palabra valida del diccionario. Devolver todas las oraciones posibles en cualquier orden.

## Por que Este Problema Muerde

Word Break I hace una pregunta de si o no: *¿se puede* segmentar la cadena? Eso es un problema limpio de programacion dinamica con comportamiento O(N^2). Word Break II pide *todas* las segmentaciones validas. Esa sola palabra -- "todas" -- transforma el problema de polinomial a potencialmente exponencial. Una cadena como `"aaa...a"` con `wordDict = ["a", "aa", "aaa", ...]` puede producir una explosion combinatoria de oraciones validas.

Mi primer instinto fue usar la misma tabla de DP de Word Break I y luego de alguna manera reconstruir los caminos. Pero la reconstruccion en si es la parte dificil -- no basta saber *que* una posicion es alcanzable; necesito recordar *cuales palabras* llegaron ahi y seguir cada bifurcacion. La herramienta natural para esto es DFS con memorizacion: explorar desde cada posicion, probar cada palabra del diccionario que coincida empezando ahi, resolver recursivamente el resto y cachear los resultados.

## La Estrategia: DFS con Memorizacion

La idea es simple en espiritu. Partiendo del indice 0 de la cadena, pruebo todos los prefijos posibles. Si ese prefijo existe en el diccionario, resuelvo recursivamente el sufijo restante. Los resultados del sufijo se combinan con la palabra actual para formar oraciones completas.

Sin memorizacion, esto revisitaria los mismos sufijos exponencialmente muchas veces. Consideremos la cadena `"catsanddog"` con las palabras `["cat", "cats", "and", "sand", "dog"]`. Tanto `"cat" + "sand..."` como `"cats" + "and..."` eventualmente necesitan resolver el sufijo `"dog"`. La memorizacion asegura que una vez que he calculado todas las oraciones a partir de un indice dado, nunca las recalculo.

### Recorriendo un Ejemplo

Con `s = "catsanddog"` y `wordDict = ["cat", "cats", "and", "sand", "dog"]`:

```
dfs(0): probar prefijos de "catsanddog"
  "cat" coincide -> dfs(3): probar prefijos de "sanddog"
    "sand" coincide -> dfs(7): probar prefijos de "dog"
      "dog" coincide -> dfs(10): caso base, retorna [""]
      -> retorna ["dog"]
    -> retorna ["sand dog"]
  -> retorna ["cat sand dog"]

  "cats" coincide -> dfs(4): probar prefijos de "anddog"
    "and" coincide -> dfs(7): EN CACHE -> ["dog"]
    -> retorna ["and dog"]
  -> retorna ["cats and dog"]

Final: ["cat sand dog", "cats and dog"]
```

Notar como `dfs(7)` se llama dos veces pero se computa solo una vez. El memo en el indice 7 almacena `["dog"]` y lo retorna inmediatamente la segunda vez.

### El Caso Base

Cuando `start == s.len()`, hemos consumido la cadena completa. Retornar un vector que contiene una sola cadena vacia es lo correcto -- senala que "el resto de la cadena esta vacio, asi que la oracion esta completa." El llamador entonces no agrega nada despues de la palabra actual, produciendo una oracion que termina limpiamente.

### ¿Por que un HashSet para el Diccionario?

Cada llamada a `dfs` itera sobre todas las posiciones finales posibles y verifica si la subcadena `s[start..end]` existe en el diccionario. Usar un `HashSet` hace que cada consulta sea O(1) amortizado, en lugar de recorrer todo el diccionario por cada prefijo candidato. Esto importa porque el bucle interno se ejecuta hasta N veces por llamada.

## Solucion en Rust

```rust
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let word_set: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
        let mut memo: HashMap<usize, Vec<String>> = HashMap::new();

        Self::dfs(0, &s, &word_set, &mut memo)
    }

    fn dfs(
        start: usize,
        s: &str,
        word_set: &HashSet<&str>,
        memo: &mut HashMap<usize, Vec<String>>,
    ) -> Vec<String> {
        if let Some(res) = memo.get(&start) {
            return res.clone();
        }

        if start == s.len() {
            return vec![String::new()];
        }

        let mut results = Vec::new();

        for end in start + 1..=s.len() {
            let word = &s[start..end];

            if word_set.contains(word) {
                let sub_sentences = Self::dfs(end, s, word_set, memo);

                for sub in sub_sentences {
                    let mut sentence = String::from(word);
                    if !sub.is_empty() {
                        sentence.push(' ');
                        sentence.push_str(&sub);
                    }
                    results.push(sentence);
                }
            }
        }

        memo.insert(start, results.clone());
        results
    }
}
```

La implementacion en Rust aprovecha las fortalezas del lenguaje con un manejo cuidadoso de prestamos. El `word_set` es un `HashSet<&str>` que toma prestado del `word_dict` original, evitando clonar las cadenas del diccionario. El `memo` es un `HashMap<usize, Vec<String>>` indexado por la posicion inicial -- cada entrada almacena todas las oraciones validas que se pueden formar a partir de `s[start..]`. La funcion `dfs` revisa el memo primero, retorna inmediatamente si hay un acierto en cache, y de lo contrario itera sobre todas las posiciones finales posibles. Cuando un prefijo coincide con una palabra del diccionario, recursa sobre el resto y ensambla cada sub-oracion anteponiendo la palabra actual. La verificacion `if !sub.is_empty()` maneja el caso base elegantemente: cuando estamos al final de la cadena, no agregamos un espacio sobrante. Despues de calcular todas las oraciones para un indice inicial dado, los resultados se clonan en el memo antes de retornar -- un costo necesario ya que el modelo de propiedad de Rust no permite retornar y almacenar el mismo vector sin clonar.

## Conclusion

Word Break II es el tipo de problema donde el enfoque ingenuo (generar todas las divisiones posibles, verificar cada una) y el enfoque optimo comparten la misma complejidad en el peor caso -- la salida misma puede ser exponencial. El valor de la memorizacion aqui no esta en cambiar la complejidad asintotica del peor caso; esta en eliminar computacion redundante para las muchas entradas donde el numero de oraciones validas es manejable. El patron DFS-con-memo es la eleccion natural: explora el espacio de busqueda de arriba hacia abajo, ramifica en cada palabra valida del diccionario y cachea los resultados para que los sufijos compartidos nunca se recalculen. Es backtracking con memoria -- lo mejor de ambos mundos.
