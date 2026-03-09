---
title: "0214 Shortest Palindrome - ES"
problemUrl: "https://leetcode.com/problems/shortest-palindrome/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "kmp", "palindrome", "rolling-hash"]
complexity:
  time: "O(N) donde N es la longitud de la cadena"
  space: "O(N) para la cadena combinada y el arreglo LPS"
---

# El Espejo Roto: Construyendo Palindromos con KMP

## El Problema
Dada una cadena `s`, encontrar el palindromo mas corto que se puede formar agregando caracteres **unicamente al inicio** de `s`. Es decir, debemos prepender la menor cantidad de caracteres posible para convertir `s` en un palindromo.

## La Intuicion Inicial

Mi primera reaccion fue pensar en fuerza bruta: probar cada posible prefijo palindromico de `s`, desde el mas largo hasta el mas corto. El prefijo palindromico mas largo que encuentre me dice exactamente que caracteres sobran al final -- esos caracteres, invertidos, son los que debo agregar al inicio. Pero verificar si cada prefijo es palindromo cuesta `O(N)`, y hay `N` prefijos posibles, asi que el enfoque ingenuo es `O(N^2)`. Para una cadena de hasta 50,000 caracteres, necesitaba algo mejor.

La pregunta clave es: *cual es el prefijo palindromico mas largo de `s`?* Si lo encuentro eficientemente, el resto del problema es trivial -- solo necesito tomar los caracteres que quedan despues de ese prefijo, invertirlos, y preponerlos.

## La Estrategia: KMP al Rescate

### La Conexion con Pattern Matching

Aqui es donde la solucion se vuelve elegante. Considero la cadena `s` y su reverso `rev(s)`. Si concateno `s + '#' + rev(s)`, puedo usar la tabla de prefijos del algoritmo KMP (tambien conocida como tabla LPS -- Longest Proper Prefix which is also Suffix) para encontrar exactamente lo que necesito.

El valor en la ultima posicion de la tabla LPS me dice la longitud del prefijo mas largo de `s` que coincide con un sufijo de `rev(s)`. Pero un sufijo de `rev(s)` *es* un prefijo de `s` leido al reves. Entonces, esta coincidencia me indica precisamente la longitud del prefijo palindromico mas largo de `s`.

### El Separador Critico

El caracter `'#'` entre `s` y `rev(s)` es fundamental. Sin el, podrian ocurrir coincidencias falsas que crucen el limite entre ambas cadenas, produciendo un valor LPS mayor al correcto. El separador garantiza que cualquier coincidencia detectada corresponda genuinamente a un prefijo de `s` que es palindromo.

### Un Ejemplo Concreto

Con `s = "aacecaaa"`:
```
s       = "aacecaaa"
rev(s)  = "aaacecaa"
combined = "aacecaaa#aaacecaa"

Tabla LPS:
a a c e c a a a # a a a c e c a a
0 1 0 0 0 1 1 1 0 1 1 1 0 0 0 1 2  <-- ultimo valor = 7
```

El ultimo valor es 7, lo que significa que "aacecaa" (los primeros 7 caracteres de `s`) forman el prefijo palindromico mas largo. Solo falta el ultimo caracter 'a', que invertido y prepuesto da `"aaacecaaa"`.

Otro ejemplo con `s = "abcd"`:
```
combined = "abcd#dcba"

Tabla LPS:
a b c d # d c b a
0 0 0 0 0 0 0 0 1  <-- ultimo valor = 1
```

Solo el primer caracter 'a' es palindromico por si mismo. Necesitamos preponer `rev("bcd") = "dcb"`, resultando en `"dcbabcd"`.

## El Algoritmo Paso a Paso

1. Calcular `rev(s)` -- el reverso de la cadena.
2. Construir la cadena combinada `s + '#' + rev(s)`.
3. Calcular la tabla LPS sobre la cadena combinada.
4. El valor `lps[ultimo]` nos da `palindrome_len`, la longitud del prefijo palindromico mas largo.
5. Tomar los primeros `n - palindrome_len` caracteres de `rev(s)` (el sufijo de `s` que no es parte del palindromo, invertido).
6. Concatenar ese fragmento con `s` original.

La belleza de este enfoque es que reduce un problema de palindromos a un problema de pattern matching, resuelto en tiempo lineal con la maquinaria de KMP.

## Solucion en Rust

```rust
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let n = s.len();
        if n == 0 {
            return s;
        }

        let s_bytes = s.as_bytes();
        let mut rev_s_bytes = s_bytes.to_vec();
        rev_s_bytes.reverse();

        let mut combined = Vec::with_capacity(2 * n + 1);
        combined.extend_from_slice(s_bytes);
        combined.push(b'#');
        combined.extend_from_slice(&rev_s_bytes);

        let m = combined.len();
        let mut lps = vec![0; m];
        let mut j = 0;

        for i in 1..m {
            while j > 0 && combined[i] != combined[j] {
                j = lps[j - 1];
            }
            if combined[i] == combined[j] {
                j += 1;
            }
            lps[i] = j;
        }

        let palindrome_len = lps[m - 1];

        let suffix_to_add =
            unsafe { String::from_utf8_unchecked(rev_s_bytes[0..n - palindrome_len].to_vec()) };

        suffix_to_add + &s
    }
}
```

La implementacion opera enteramente a nivel de bytes con `as_bytes()`, evitando el coste de manipular caracteres UTF-8 multibyte cuando sabemos que la entrada contiene solo ASCII. La cadena combinada se construye con `Vec::with_capacity(2 * n + 1)` para evitar realocaciones durante las inserciones. El bucle de KMP es clasico: cuando hay un mismatch, retrocede usando `lps[j - 1]` hasta encontrar un prefijo compatible o llegar al inicio. El uso de `unsafe { String::from_utf8_unchecked(...) }` es seguro aqui porque los bytes originales provienen de una `String` valida -- solo estamos tomando un subconjunto de bytes que ya sabemos son UTF-8 correcto. Finalmente, la concatenacion `suffix_to_add + &s` aprovecha la implementacion de `Add<&str>` para `String` en Rust, que toma ownership del lado izquierdo y appenda el lado derecho sin copias adicionales.

## Conclusion

Shortest Palindrome es un ejemplo fascinante de como un problema aparentemente sobre palindromos se resuelve mejor con herramientas de pattern matching. La tabla LPS de KMP, disenada originalmente para buscar subcadenas, resulta ser exactamente lo que necesitamos para encontrar el prefijo palindromico mas largo. La construccion `s + '#' + rev(s)` es el puente conceptual que conecta ambos mundos -- transforma "encontrar el mayor prefijo de `s` que es palindromo" en "encontrar la mayor coincidencia entre un prefijo de `s` y un sufijo de `rev(s)`", y eso es precisamente lo que KMP hace en tiempo lineal.
