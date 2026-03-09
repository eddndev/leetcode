---
title: "0068 Text Justification - ES"
problemUrl: "https://leetcode.com/problems/text-justification/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "simulation", "greedy"]
complexity:
  time: "O(N)"
  space: "O(N)"
---

# El Arte de Componer Texto a Mano

## El Problema
Dada una lista de palabras y un ancho maximo `maxWidth`, formatear el texto de manera que cada linea tenga exactamente `maxWidth` caracteres y este completamente justificada (alineada tanto a la izquierda como a la derecha). Las palabras deben empaquetarse con un algoritmo greedy: en cada linea caben tantas palabras como sea posible, y los espacios sobrantes se distribuyen lo mas uniformemente posible entre los huecos. La ultima linea debe estar alineada a la izquierda, con espacios al final para completar el ancho.

## La Intuicion Inicial

Cuando lei este problema por primera vez, mi primer pensamiento fue: esto no es un problema de algoritmos clasico, es un problema de ingenieria. No hay un truco matematico oculto ni una estructura de datos exotica. Lo que se necesita es simular exactamente lo que haria un procesador de texto, paso por paso, sin equivocarse en los detalles.

Y ahi esta precisamente la dificultad. Los casos borde son muchos: lineas con una sola palabra (no hay huecos donde repartir espacios), la ultima linea (alineacion izquierda en vez de justificada), y la distribucion desigual de espacios (los huecos de la izquierda reciben un espacio extra cuando la division no es exacta). El problema es "Hard" no porque la idea sea compleja, sino porque la implementacion tiene que ser impecable.

## La Estrategia Greedy de Empaquetado

La primera fase es decidir que palabras van en cada linea. Uso un enfoque greedy: voy acumulando palabras mientras la suma de sus longitudes mas los espacios minimos (uno entre cada par de palabras) no exceda `maxWidth`. En el momento en que una nueva palabra no cabe, justifico la linea actual y comienzo una nueva.

La condicion clave es `current_len + word.len() + current_line.len() > max_width`. Aqui `current_len` es la suma de caracteres de las palabras ya acumuladas, y `current_line.len()` es el numero de palabras, que coincide con el numero minimo de espacios necesarios si agregaramos la palabra nueva (un espacio entre cada par). Si esa suma excede el ancho, la palabra no cabe y debemos justificar lo que tenemos.

## La Justificacion Linea por Linea

Una vez que se que palabras van en una linea, hay dos casos:

**Caso 1: Ultima linea o linea con una sola palabra.** Aqui la justificacion es sencilla: las palabras se separan con un solo espacio, y el resto del ancho se rellena con espacios al final. No hay distribucion proporcional.

**Caso 2: Linea intermedia con multiples palabras.** Tengo `gaps` huecos (uno menos que el numero de palabras) y `total_spaces` espacios por repartir (el ancho total menos los caracteres de las palabras). Divido: `spaces_per_gap = total_spaces / gaps` me da los espacios base por hueco, y `extra_spaces = total_spaces % gaps` me dice cuantos huecos reciben uno mas. Los primeros `extra_spaces` huecos obtienen `spaces_per_gap + 1` espacios, y el resto obtiene `spaces_per_gap`. Esto garantiza que la diferencia entre cualquier par de huecos sea como maximo 1, y que los mas anchos esten a la izquierda, exactamente como pide el enunciado.

### Un Ejemplo Paso a Paso

Para `words = ["This", "is", "an", "example", "of", "text", "justification."]` con `maxWidth = 16`:

- **Linea 1:** "This" (4) + "is" (2) + "an" (2) = 8 caracteres, 3 palabras. Intentamos agregar "example" (7): 8 + 7 + 3 = 18 > 16. No cabe. Justificamos: 16 - 8 = 8 espacios para 2 huecos: 4 y 4. Resultado: `"This    is    an"`.
- **Linea 2:** "example" (7) + "of" (2) + "text" (4) = 13 caracteres, 3 palabras. Intentamos agregar "justification." (14): 13 + 14 + 3 = 30 > 16. No cabe. Justificamos: 16 - 13 = 3 espacios para 2 huecos: 2 y 1. Resultado: `"example  of text"`.
- **Linea 3:** "justification." (14), ultima linea. Alineacion izquierda con relleno: `"justification.  "`.

## Solucion en Rust

```rust
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut result = Vec::new();
        let mut current_line: Vec<&String> = Vec::new();
        let mut current_len = 0;

        for word in &words {
            if current_len + word.len() + current_line.len() > max_width {
                result.push(Self::justify_line(
                    &current_line,
                    current_len,
                    max_width,
                    false,
                ));

                current_line.clear();
                current_len = 0;
            }

            current_line.push(word);
            current_len += word.len();
        }

        if !current_line.is_empty() {
            result.push(Self::justify_line(
                &current_line,
                current_len,
                max_width,
                true,
            ));
        }

        result
    }

    fn justify_line(
        line: &[&String],
        line_char_len: usize,
        max_width: usize,
        is_last_line: bool,
    ) -> String {
        if is_last_line || line.len() == 1 {
            let mut s = String::with_capacity(max_width);

            for (i, word) in line.iter().enumerate() {
                if i > 0 {
                    s.push(' ');
                }
                s.push_str(word);
            }

            let remaining = max_width - s.len();
            for _ in 0..remaining {
                s.push(' ');
            }
            return s;
        }

        let gaps = line.len() - 1;
        let total_spaces = max_width - line_char_len;

        let spaces_per_gap = total_spaces / gaps;
        let extra_spaces = total_spaces % gaps;

        let mut s = String::with_capacity(max_width);

        for (i, word) in line.iter().enumerate() {
            s.push_str(word);

            if i < gaps {
                let spaces_to_add = spaces_per_gap + if i < extra_spaces { 1 } else { 0 };
                for _ in 0..spaces_to_add {
                    s.push(' ');
                }
            }
        }

        s
    }
}
```

La implementacion en Rust separa claramente las dos responsabilidades: `full_justify` se encarga del empaquetado greedy, y `justify_line` de la distribucion de espacios. El uso de `String::with_capacity(max_width)` es un detalle que evita reallocaciones innecesarias, ya que sabemos de antemano el tamano exacto de cada linea. La funcion auxiliar recibe un flag `is_last_line` para distinguir entre justificacion completa y alineacion izquierda, lo cual mantiene la logica limpia sin duplicar codigo. La aritmetica de division y modulo para distribuir espacios es concisa y correcta: `spaces_per_gap` da la base uniforme, y los primeros `extra_spaces` huecos absorben el residuo, garantizando que la diferencia maxima entre huecos sea exactamente uno.

## Conclusion

Text Justification es uno de esos problemas que parecen simples hasta que te sientas a implementarlos. No hay ningun momento "aja" ni una estructura de datos ingeniosa que descubrir. La dificultad esta en manejar correctamente todos los casos: lineas con una sola palabra, la ultima linea, la distribucion desigual de espacios. Es un ejercicio puro de precision en la implementacion, y es exactamente el tipo de problema que separa a quien entiende un algoritmo de quien puede traducirlo a codigo sin errores. A veces, el verdadero desafio no es encontrar la idea, sino ejecutarla sin fisuras.
