---
title: "0005 Longest Palindromic Substring - ES"
problemUrl: "https://leetcode.com/problems/longest-palindromic-substring/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["string", "two-pointers", "expand-around-center"]
complexity:
  time: "O(n^2)"
  space: "O(n)"
---

# Longest Palindromic Substring: Expandiendo Desde el Centro

## El Problema
Dada una cadena `s`, encontrar la subcadena palindrómica más larga contenida en ella.

Un palíndromo es una cadena que se lee igual de izquierda a derecha que de derecha a izquierda. Por ejemplo, en `"babad"` la respuesta podría ser `"bab"` o `"aba"`, y en `"cbbd"` la respuesta es `"bb"`.

La fuerza bruta sería revisar todas las subcadenas posibles y verificar cuáles son palíndromos, pero eso nos daría $O(n^3)$. Necesitamos algo mejor.

## La Intuición: Todo Palíndromo Tiene un Centro
Cuando vi este problema, la observación clave fue que **todo palíndromo se expande simétricamente desde su centro**. En lugar de generar todas las subcadenas y verificar cada una, podemos hacer lo inverso: para cada posible centro, expandir hacia afuera mientras los caracteres coincidan.

Hay un detalle sutil: los palíndromos pueden tener longitud impar o par. Un palíndromo impar como `"aba"` tiene un solo carácter como centro (la `b`). Un palíndromo par como `"abba"` tiene su centro entre los dos caracteres `b`. Esto significa que para cada posición `i` debemos hacer **dos expansiones**: una con centro en un solo carácter (`i, i`) y otra con centro entre dos caracteres (`i, i+1`).

## El Algoritmo
1. Convertimos el string en un vector de caracteres para acceso por índice.
2. Para cada posición `i` de 0 a `n-1`:
   - Expandimos desde `(i, i)` para palíndromos de longitud impar.
   - Expandimos desde `(i, i+1)` para palíndromos de longitud par.
3. En cada expansión, movemos los punteros `l` y `r` hacia afuera mientras `chars[l] == chars[r]`.
4. Mantenemos registro de la posición de inicio y la longitud máxima encontrada.
5. Al final, extraemos la subcadena correspondiente.

La función `expand` es el corazón de la solución. Recibe el array de caracteres y dos índices iniciales, y devuelve la posición de inicio y la longitud del palíndromo encontrado. El caso especial es cuando `l` llega a 0: necesitamos retornar inmediatamente porque no podemos decrementar más (estamos en un `usize`, restar de 0 causaría un underflow).

### Implementación en Rust
En Rust, trabajamos con `Vec<char>` porque los strings en Rust son UTF-8 y no permiten indexación directa por posición. La función `expand` maneja cuidadosamente los bordes del array, especialmente cuando el puntero izquierdo llega a la posición 0.

```rust
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut max_len = 0;

        for i in 0..chars.len() {
            let (s1, l1) = Self::expand(&chars, i, i);

            let (s2, l2) = Self::expand(&chars, i, i + 1);

            if l1 > max_len {
                max_len = l1;
                start = s1;
            }

            if l2 > max_len {
                max_len = l2;
                start = s2;
            }
        }

        chars[start..start + max_len].iter().collect()
    }

    fn expand(chars: &[char], mut l: usize, mut r: usize) -> (usize, usize) {
        let n = chars.len();

        while r < n && chars[l] == chars[r] {
            if l == 0 && chars[l] == chars[r] {
                if l == 0 {
                    return (0, r + 1);
                }
            }
            l -= 1;
            r += 1;

            if r >= n || chars[l] != chars[r] {
                return (l + 1, r - (l + 1));
            }
        }

        (l + 1, r - (l + 1))
    }
}
```

## Conclusión
La técnica de expandir desde el centro convierte un problema que parece requerir $O(n^3)$ en uno resoluble en $O(n^2)$. Por cada uno de los $n$ centros posibles, la expansión puede recorrer como máximo $O(n)$ caracteres, dándonos $O(n^2)$ en tiempo y $O(n)$ en espacio (por el `Vec<char>`).

La lección de este problema es que a veces **invertir la perspectiva** simplifica todo. En lugar de verificar si cada subcadena es un palíndromo, construimos palíndromos desde adentro hacia afuera. Es una idea elegante que aparece en muchos otros problemas de strings.
