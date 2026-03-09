---
title: "0065 Valid Number - ES"
problemUrl: "https://leetcode.com/problems/valid-number/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "simulation"]
complexity:
  time: "O(N)"
  space: "O(1)"
---

# La Gramatica de los Numeros

## El Problema
Dada una cadena `s`, determinar si representa un numero valido. Un numero valido puede ser un entero o un decimal, opcionalmente seguido de una parte exponencial (`e` o `E`). Los enteros y decimales pueden ir precedidos por un signo (`+` o `-`). Los decimales deben contener al menos un digito y exactamente un punto. La parte exponencial consiste en `e` o `E` seguido de un entero (que a su vez puede tener signo).

## La Simplicidad Enganosa

A primera vista, este problema parece trivial: simplemente parsear la cadena y ver si se parece a un numero. Pero cuando empiezas a listar los casos borde, la lista no termina nunca. Es `".1"` valido? Si. Y `"."` solo? No. Puede un exponente tener un decimal? No. Puede un signo aparecer en medio de un numero? Solo justo despues de `e` o `E`. Cada regla tiene su matiz, y una implementacion descuidada fallara en algun caso oscuro.

Considere usar una expresion regular o una maquina de estados finitos con estados y transiciones explicitamente definidos. Ambas opciones son validas, pero quise algo mas directo: un solo recorrido por la cadena con un punado de banderas booleanas que rastrean lo que hemos visto hasta ahora. La logica se convierte en un `match` cuidadoso sobre cada caracter, donde las banderas nos dicen si lo que estamos viendo sigue siendo legal.

## La Estrategia de un Solo Recorrido

La idea es caminar por la cadena caracter por caracter, manteniendo tres banderas:

- **`seen_digit`**: Hemos visto al menos un digito? Esto es crucial porque cadenas como `"."`, `"e5"` o `"+"` son invalidas: un numero debe contener digitos.
- **`seen_exponent`**: Ya encontramos un `e` o `E`? No puede haber dos exponentes.
- **`seen_dot`**: Ya encontramos un punto? Un numero puede tener como maximo uno, y no puede aparecer en la parte exponencial.

Para cada caracter, aplicamos una regla especifica:

1. **Digito (`0-9`):** Siempre valido. Marcamos `seen_digit = true`.

2. **Signo (`+` o `-`):** Solo valido al inicio de la cadena o inmediatamente despues de `e`/`E`. Si aparece en cualquier otro lugar, la cadena es invalida.

3. **Exponente (`e` o `E`):** Invalido si ya vimos uno, o si no ha aparecido ningun digito antes (porque `"e5"` no es un numero). Cuando lo aceptamos, marcamos `seen_exponent = true` y **reiniciamos `seen_digit` a `false`**. Este ultimo detalle es esencial: la parte exponencial debe contener sus propios digitos, asi que `"1e"` es invalido.

4. **Punto (`.`):** Invalido si ya vimos un punto o si estamos en la parte exponencial (porque los exponentes deben ser enteros). En caso contrario, lo aceptamos y marcamos `seen_dot = true`.

5. **Cualquier otra cosa:** Inmediatamente invalido.

Al final, retornamos `seen_digit`. Esta verificacion final atrapa casos como `"1e"` (exponente sin digitos despues) o `"."` (punto sin ningun digito). El reinicio de `seen_digit` cuando encontramos un exponente es lo que hace que esto funcione: obliga a la parte exponencial a demostrar que tiene sus propios digitos.

### Recorriendo un Ejemplo

Para `s = "-3.14e2"`:
- `'-'`: indice 0, signo al inicio es permitido.
- `'3'`: digito, `seen_digit = true`.
- `'.'`: no hay punto previo, no hay exponente, `seen_dot = true`.
- `'1'`: digito, `seen_digit = true`.
- `'4'`: digito, `seen_digit = true`.
- `'e'`: `seen_digit` es true, no hay exponente previo. `seen_exponent = true`, reiniciamos `seen_digit = false`.
- `'2'`: digito, `seen_digit = true`.
- Fin: `seen_digit` es `true`. **Valido.**

Para `s = "1e"`:
- `'1'`: digito, `seen_digit = true`.
- `'e'`: valido (hay digito previo, no hay exponente previo). `seen_exponent = true`, `seen_digit = false`.
- Fin: `seen_digit` es `false`. **Invalido.** El exponente no tiene digitos.

## Solucion en Rust

```rust
impl Solution {
    pub fn is_number(s: String) -> bool {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut seen_digit = false;
        let mut seen_exponent = false;
        let mut seen_dot = false;

        for (i, &b) in bytes.iter().enumerate() {
            match b {
                b'0'..=b'9' => {
                    seen_digit = true;
                }

                b'+' | b'-' => {
                    if i > 0 && bytes[i - 1] != b'e' && bytes[i - 1] != b'E' {
                        return false;
                    }
                }

                b'e' | b'E' => {
                    if seen_exponent || !seen_digit {
                        return false;
                    }
                    seen_exponent = true;

                    seen_digit = false;
                }

                b'.' => {
                    if seen_dot || seen_exponent {
                        return false;
                    }
                    seen_dot = true;
                }

                _ => return false,
            }
        }

        seen_digit
    }
}
```

La implementacion en Rust trabaja directamente sobre bytes con `as_bytes()`, lo cual es eficiente y natural ya que todos los caracteres validos en un numero son ASCII. La expresion `match` se mapea limpiamente a las reglas que describimos: cada brazo maneja una categoria de caracter, y las banderas imponen las restricciones. Hay una elegancia sutil en como funciona la validacion del signo: en lugar de rastrear una bandera `seen_sign` separada, simplemente verificamos el caracter anterior. Si el signo no esta en la posicion 0 y el caracter antes de el no es `e` o `E`, es ilegal. Esto evita estado extra mientras se mantiene perfectamente correcto.

## Conclusion

Este problema recompensa la precision por encima de la astucia. No hay ningun truco algoritmico aqui, no hay tabla de programacion dinamica, no hay recorrido de grafos. Se trata de entender una gramatica y codificarla fielmente en una serie de verificaciones simples. Las tres banderas booleanas actuan como una maquina de estados minima, y el recorrido unico por la cadena asegura que nunca hacemos mas trabajo del necesario. Lo que hace elegante a la solucion no es su complejidad sino su mesura: justo el estado suficiente para capturar cada regla, y ni un bit mas.
