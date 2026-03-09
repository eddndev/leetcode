---
title: "0336 Palindrome Pairs - ES"
problemUrl: "https://leetcode.com/problems/palindrome-pairs/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["hash-map", "string", "palindrome"]
complexity:
  time: "O(N * K^2)"
  space: "O(N * K)"
---

# Cuando Dos Palabras se Completan

## El Problema
Dada una lista de palabras unicas, encontrar todos los pares de indices distintos `(i, j)` tales que la concatenacion de `words[i] + words[j]` forme un palindromo.

## La Trampa de la Fuerza Bruta

El enfoque ingenuo es tentador: probar cada par `(i, j)`, concatenar las cadenas y verificar si el resultado es un palindromo. Con hasta 5000 palabras de hasta 300 caracteres cada una, eso son O(N^2 * K) verificaciones, que pueden llegar a miles de millones de operaciones. Necesitamos algo mas inteligente.

## La Observacion Clave

Que hace que una concatenacion `words[i] + words[j]` sea un palindromo? Realmente solo hay unos pocos casos estructurales a considerar, y todos giran alrededor de dividir una palabra en cada posicion posible y hacer dos preguntas:

1. **Si la porcion izquierda de una palabra es un palindromo**, entonces el reverso de su porcion derecha, colocado antes de la palabra, completaria un palindromo completo. Asi que buscamos `reverso(porcion derecha)` en nuestro diccionario y, si lo encontramos, esa palabra va a la izquierda.

2. **Si la porcion derecha de una palabra es un palindromo**, entonces el reverso de su porcion izquierda, colocado despues de la palabra, completaria un palindromo completo. Asi que buscamos `reverso(porcion izquierda)` en nuestro diccionario y, si lo encontramos, esa palabra va a la derecha.

Iterando sobre cada punto de division posible `j` desde `0` hasta `len` para cada palabra, cubrimos todos los casos, incluyendo el caso borde importante donde una de las dos porciones es la palabra completa (y la otra esta vacia). Cuando la division esta en la posicion `0`, la porcion izquierda esta vacia (lo cual es trivialmente un palindromo), y estamos buscando el reverso completo de la palabra. Cuando la division esta en la posicion `len`, la porcion derecha esta vacia, y buscamos el reverso completo de nuevo, pero colocado de forma diferente.

La condicion extra `s2.len() > 0` en la segunda verificacion previene contar el mismo par dos veces cuando ambas porciones se reducen al mismo escenario de cadena vacia.

## Un Ejemplo Paso a Paso

Consideremos `words = ["abcd", "dcba", "lls", "s", "sssll"]`:

- Para `"abcd"`: en la division `j=4`, la porcion derecha esta vacia (palindromo), la porcion izquierda es `"abcd"`, su reverso `"dcba"` esta en el mapa. Asi que `("abcd", "dcba")` es un par valido. En la division `j=0`, la porcion izquierda esta vacia (palindromo), la porcion derecha es `"abcd"`, su reverso `"dcba"` esta en el mapa. Asi que `("dcba", "abcd")` tambien es un par valido.
- Para `"lls"`: en la division `j=2`, la porcion derecha es `"s"` (palindromo), la porcion izquierda es `"ll"`, su reverso `"ll"` no esta en el mapa. En la division `j=1`, la porcion izquierda es `"l"` (palindromo), la porcion derecha es `"ls"`, su reverso `"sl"` no esta en el mapa. Pero para `"sssll"` en la division `j=2`, la porcion izquierda es `"ss"` (palindromo), la porcion derecha es `"sll"`, su reverso `"lls"` esta en el mapa. Asi que `("lls", "sssll")` es valido.

Resultado: `[[0,1], [1,0], [2,4], [3,2]]`.

## Solucion en Rust

```rust
use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        // Construimos un mapa de palabra -> índice para búsquedas rápidas
        for (i, word) in words.iter().enumerate() {
            map.insert(word.as_str(), i);
        }

        let mut res = Vec::new();

        for (i, word) in words.iter().enumerate() {
            let n = word.len();
            let chars = word.as_bytes();

            for j in 0..=n {
                let s1 = &chars[0..j];
                let s2 = &chars[j..n];

                if is_palindrome(s1) {
                    let mut rev_s2 = s2.to_vec();
                    rev_s2.reverse();
                    if let Ok(target) = std::str::from_utf8(&rev_s2) {
                        if let Some(&k) = map.get(target) {
                            if k != i {
                                res.push(vec![k as i32, i as i32]);
                            }
                        }
                    }
                }

                if s2.len() > 0 && is_palindrome(s2) {
                    let mut rev_s1 = s1.to_vec();
                    rev_s1.reverse();
                    if let Ok(target) = std::str::from_utf8(&rev_s1) {
                        if let Some(&k) = map.get(target) {
                            if k != i {
                                res.push(vec![i as i32, k as i32]);
                            }
                        }
                    }
                }
            }
        }

        res
    }
}

fn is_palindrome(chars: &[u8]) -> bool {
    let mut left = 0;
    let mut right = chars.len();
    if right == 0 {
        return true;
    }
    right -= 1;

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
```

La implementacion en Rust comienza construyendo un `HashMap` de cada palabra a su indice para busquedas en O(1). Para cada palabra, iteramos por todas las posiciones de division posibles `j` desde `0` hasta `n` inclusive, produciendo una porcion izquierda `s1 = word[0..j]` y una porcion derecha `s2 = word[j..n]`. Cuando `s1` es un palindromo, verificamos si el reverso de `s2` existe en el mapa, lo que significaria que anteponer esa palabra crea un palindromo completo. Simetricamente, cuando `s2` es un palindromo, verificamos si el reverso de `s1` existe, lo que significaria que agregar esa palabra al final crea un palindromo. La guarda `s2.len() > 0` en la segunda rama previene el doble conteo del caso donde `j = n` (porcion derecha vacia), ya que el caso `j = 0` en la primera rama ya maneja la busqueda del reverso completo. Trabajar a nivel de bytes con `as_bytes()` evita la sobrecarga de UTF-8 para estas entradas que son solo ASCII, y `std::str::from_utf8` convierte de vuelta de forma segura para la busqueda en el mapa.

## Conclusion

Palindrome Pairs es un problema donde la fuerza bruta esta tentadoramente cerca de ser viable pero no alcanza. La clave que nos salva es descomponer la condicion de palindromo en casos estructurales: para cada division posible de una palabra, si un lado ya es un palindromo, el reverso del otro lado es exactamente el companiero que necesitamos. Un hash map convierte cada busqueda de companiero en tiempo constante, reduciendo la complejidad total a O(N * K^2), donde K es la longitud maxima de palabra. Es un ejemplo hermoso de como entender la estructura del resultado que buscamos puede reducir dramaticamente el espacio de busqueda.
