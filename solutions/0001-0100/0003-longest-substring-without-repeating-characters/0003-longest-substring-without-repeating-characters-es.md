---
title: "0003 Longest Substring Without Repeating Characters - ES"
problemUrl: "https://leetcode.com/problems/longest-substring-without-repeating-characters/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["hash-table", "string", "sliding-window"]
complexity:
  time: "O(n)"
  space: "O(1)"
---

# Longest Substring Without Repeating Characters: La Ventana Deslizante y el Truco del Array Estático

## El Problema
Dada una cadena `s`, encontrar la longitud de la subcadena más larga sin caracteres repetidos.

Suena simple al principio, pero la clave está en cómo rastreamos eficientemente qué caracteres ya están dentro de nuestra "ventana" actual y cómo la ajustamos sin retroceder innecesariamente.

## La Primera Intuición: HashSet y Fuerza Bruta
Mi primer instinto fue el enfoque ingenuo: para cada posición del string, expandir hacia la derecha mientras no haya repeticiones, usando un `HashSet` para detectar duplicados. Esto funciona, pero tiene complejidad $O(n^2)$ en el peor caso. Cada vez que encontramos un duplicado, movemos el inicio un paso y volvemos a escanear. Inaceptable.

## La Evolución: Sliding Window con HashMap
El siguiente paso natural fue la técnica de **ventana deslizante** (sliding window). Mantenemos dos punteros, `left` y `right`, que definen los bordes de nuestra ventana. Cuando `right` avanza y encuentra un carácter repetido, en lugar de reiniciar desde cero, movemos `left` justo después de la última aparición de ese carácter.

Usando un `HashMap` podríamos almacenar la última posición de cada carácter. Pero un HashMap tiene overhead: hashing, manejo de colisiones, asignación dinámica interna. Para un problema donde el universo de caracteres es limitado (128 caracteres ASCII), hay una solución más elegante.

## La Optimización: Array Estático de 128 Posiciones
En lugar de un HashMap, usé un array estático `[usize; 128]`. Cada índice del array corresponde al valor ASCII de un carácter, y almacena la **posición + 1** de la última vez que vimos ese carácter.

¿Por qué posición + 1? Porque inicializamos todo en 0, y necesitamos distinguir "nunca visto" (valor 0) de "visto en la posición 0". Al guardar `right + 1`, la comparación `last_seen[idx] > left` nos dice directamente si el carácter está dentro de la ventana actual.

Este truco elimina la necesidad de un HashMap y nos da acceso $O(1)$ garantizado sin colisiones ni overhead de hashing. Además, el array de 128 elementos vive en el stack, no en el heap, lo cual es excelente para la localidad de caché.

### Implementación en Rust
En Rust, el método `.as_bytes()` sobre un `String` es $O(1)$ porque internamente `String` ya almacena los datos como un buffer de bytes UTF-8. Combinado con `.enumerate()`, recorremos el string de forma idiomática y eficiente.

```rust
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen = [0usize; 128];
        let mut max_len = 0;
        let mut left = 0;

        // .as_bytes() is O(1) in Rust 'cause String already saves the buffer internaly
        // .enumerate() gives us the index (right) and the value (byte)
        for (right, &byte) in s.as_bytes().iter().enumerate() {
            let idx = byte as usize;

            // If the character has been seen in the current range of the window
            if last_seen[idx] > left {
                left = last_seen[idx];
            }

            // We update the position of the character (idx + 1)
            last_seen[idx] = right + 1;

            let current_len = (right - left + 1) as i32;
            if current_len > max_len {
                max_len = current_len;
            }
        }

        max_len
    }
}
```

## Conclusión

Pasé del enfoque ingenuo $O(n^2)$ a una solución $O(n)$ con espacio constante $O(1)$ (el array de 128 elementos es fijo, independiente del tamaño del input).

La lección clave de este problema es que cuando el universo de claves es pequeño y conocido, **un array estático siempre vence a un HashMap**. Sin hashing, sin colisiones, sin asignaciones dinámicas. Solo aritmética de índices y memoria contigua en el stack.
