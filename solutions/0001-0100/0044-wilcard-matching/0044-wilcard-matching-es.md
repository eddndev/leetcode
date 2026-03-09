---
title: "0044 Wildcard Matching - ES"
problemUrl: "https://leetcode.com/problems/wildcard-matching/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "greedy", "two-pointers"]
complexity:
  time: "O(M * N)"
  space: "O(1)"
---

# Domando el Comodin

## El Problema
Dada una cadena `s` y un patron `p`, implementar el matching de patrones wildcard con soporte para `'?'` (coincide con cualquier caracter individual) y `'*'` (coincide con cualquier secuencia de caracteres, incluyendo la secuencia vacia). El matching debe cubrir **toda** la cadena de entrada, no solo una parte.

## El Primer Instinto

Cuando me enfrente a este problema por primera vez, mi instinto fue recurrir a programacion dinamica. Al fin y al cabo, es un problema de matching con dos cadenas y decisiones que se ramifican, igual que Regular Expression Matching. Una tabla DP de tamanio M x N lo resolveria de forma limpia. Pero entonces me pregunte: el `'*'` del wildcard matching realmente necesita tanta maquinaria?

En expresiones regulares, `'*'` esta atado al caracter que lo precede: `a*` significa "cero o mas `a`s". Ese acoplamiento crea subproblemas genuinamente solapados. Pero en wildcard matching, `'*'` es independiente: puede coincidir con cualquier secuencia de caracteres por si solo. Esa independencia es lo que abre la puerta a un enfoque **greedy** que usa solo espacio constante.

## La Estrategia Greedy

La idea es recorrer ambas cadenas simultaneamente con dos punteros, `s_idx` y `p_idx`, y manejar cada situacion conforme aparece:

1. **Match directo o `?`:** Si el caracter actual del patron coincide con el caracter actual de la cadena, o el patron tiene `'?'`, avanzamos ambos punteros. Este es el caso directo.

2. **Se encuentra un asterisco:** Cuando encontramos un `'*'` en el patron, no decidimos inmediatamente cuantos caracteres va a consumir. En su lugar, registramos la posicion de este asterisco (`star_idx`) y la posicion actual en la cadena (`s_tmp_idx`). Luego avanzamos solo el puntero del patron, intentando primero que el asterisco coincida con cero caracteres.

3. **No hay coincidencia pero hay un asterisco al cual volver:** Si ninguno de los casos anteriores aplica pero tenemos un asterisco previamente registrado, hacemos **backtrack**. Devolvemos el puntero del patron justo despues del asterisco, incrementamos `s_tmp_idx` en uno (dejando que el asterisco consuma un caracter mas) y colocamos `s_idx` en `s_tmp_idx`. Aqui es donde ocurre el reintento greedy.

4. **No hay coincidencia y no hay asterisco:** Si no hay ningun asterisco al cual volver, el match falla por completo.

Despues de consumir toda la cadena, puede haber caracteres `'*'` al final del patron. Todos ellos pueden coincidir con la secuencia vacia, asi que los saltamos. Si el puntero del patron llego al final, el match es exitoso.

### Por Que Funciona

La observacion clave es que un `'*'` solo necesita "recordar" su ocurrencia mas reciente. Si encontramos un segundo `'*'`, este subsume al primero: cualquier caracter que el primer asterisco necesitaba cubrir ahora queda cubierto por el alcance expandido del segundo. Esto significa que nunca necesitamos rastrear mas de un asterisco a la vez, y un solo puntero de backtrack es suficiente.

### Un Ejemplo Paso a Paso

Para `s = "adceb"`, `p = "*a*b"`:
- `p_idx=0` es `'*'`: registramos `star_idx=0`, `s_tmp_idx=0`. `p_idx=1`
- `s_idx=0` es `'a'`, `p_idx=1` es `'a'`: coinciden. `s_idx=1`, `p_idx=2`
- `p_idx=2` es `'*'`: registramos `star_idx=2`, `s_tmp_idx=1`. `p_idx=3`
- `s_idx=1` es `'d'`, `p_idx=3` es `'b'`: no coinciden. Backtrack: `s_tmp_idx=2`, `s_idx=2`, `p_idx=3`
- `s_idx=2` es `'c'`, `p_idx=3` es `'b'`: no coinciden. Backtrack: `s_tmp_idx=3`, `s_idx=3`, `p_idx=3`
- `s_idx=3` es `'e'`, `p_idx=3` es `'b'`: no coinciden. Backtrack: `s_tmp_idx=4`, `s_idx=4`, `p_idx=3`
- `s_idx=4` es `'b'`, `p_idx=3` es `'b'`: coinciden. `s_idx=5`, `p_idx=4`
- `s_idx=5`: cadena consumida. El patron tambien llego al final. Resultado: **true**

El segundo `'*'` intento coincidir con cero caracteres, luego uno, luego dos, luego tres, hasta que el patron restante `"b"` se alineo con el final de la cadena. Esa expansion incremental es el corazon del enfoque greedy.

## Solucion en Rust

```rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();

        let (mut s_idx, mut p_idx) = (0, 0);
        let (mut star_idx, mut s_tmp_idx) = (None, 0);

        while s_idx < s_bytes.len() {
            if p_idx < p_bytes.len() && (p_bytes[p_idx] == b'?' || p_bytes[p_idx] == s_bytes[s_idx])
            {
                s_idx += 1;
                p_idx += 1;
            } else if p_idx < p_bytes.len() && p_bytes[p_idx] == b'*' {
                star_idx = Some(p_idx);
                s_tmp_idx = s_idx;
                p_idx += 1;
            } else if let Some(star_p) = star_idx {
                p_idx = star_p + 1;
                s_tmp_idx += 1;
                s_idx = s_tmp_idx;
            } else {
                return false;
            }
        }

        while p_idx < p_bytes.len() && p_bytes[p_idx] == b'*' {
            p_idx += 1;
        }

        p_idx == p_bytes.len()
    }
}
```

La implementacion en Rust es concisa y expresiva. Convertir las cadenas a `&[u8]` con `as_bytes()` nos permite trabajar con comparaciones de bytes directamente, evitando cualquier sobrecarga de Unicode en un problema que solo maneja letras minusculas y dos caracteres especiales. El uso de `Option<usize>` para `star_idx` es idiomatico: `None` significa que aun no se ha visto ningun asterisco, y el patron `if let Some(star_p)` en la rama de backtracking se lee naturalmente como "si hay un asterisco al cual volver". Todo el algoritmo se ejecuta con solo cuatro variables escalares y sin allocaciones en el heap mas alla de las cadenas de entrada.

## Conclusion

Este problema es un hermoso ejemplo de como entender la estructura de un problema puede llevar a una solucion dramaticamente mas simple. El `'*'` del wildcard es fundamentalmente diferente al `'*'` de regex: es autocontenido, no esta acoplado a un caracter precedente. Esa independencia significa que no necesitamos una tabla DP para explorar todas las posibilidades. Una sola pasada con una estrategia greedy de backtracking, recordando solo el asterisco mas reciente, es suficiente para cubrir todos los casos. El resultado es O(1) de espacio y codigo que cabe en un solo loop, un recordatorio de que a veces la mejor optimizacion no es una estructura de datos ingeniosa, sino una mirada mas profunda al problema en si.
