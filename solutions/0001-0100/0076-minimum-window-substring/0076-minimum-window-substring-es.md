---
title: "0076 Minimum Window Substring - ES"
problemUrl: "https://leetcode.com/problems/minimum-window-substring/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "sliding-window", "hash-table"]
complexity:
  time: "O(N + M)"
  space: "O(1)"
---

# La Ventana Mas Pequena que lo Contiene Todo

## El Problema
Dadas dos cadenas `s` y `t`, encontrar la subcadena minima de `s` que contenga todos los caracteres de `t` (incluyendo duplicados). Si no existe tal ventana, devolver una cadena vacia.

## La Intuicion Inicial

Cuando me enfrente a este problema por primera vez, el camino de fuerza bruta era obvio: revisar todas las subcadenas posibles de `s` y verificar si contienen todos los caracteres de `t`. Eso es O(N^2 * M) en el mejor caso, y claramente no va a funcionar para entradas grandes.

El salto mental viene de reconocer que esto es un problema clasico de **ventana deslizante**. En lugar de generar todas las subcadenas, puedo mantener una ventana definida por dos punteros, `left` y `right`, y deslizarla a traves de `s`. El puntero derecho expande la ventana para incluir mas caracteres, y el izquierdo la contrae para encontrar la ventana valida minima. La pregunta se convierte en: como se de forma eficiente cuando mi ventana contiene todos los caracteres de `t`?

## El Enfoque del Mapa de Frecuencias

La respuesta es un mapa de frecuencias. Uso un arreglo de tamano 128 (cubriendo todos los caracteres ASCII) para almacenar la cantidad de cada caracter necesario de `t`. A medida que el puntero derecho avanza, decremento el conteo del caracter que encuentra. Cuando el conteo de un caracter pasa de positivo a cero o menos, significa que hemos satisfecho el requerimiento de ese caracter. Rastreo esto con una sola variable `count`, inicializada en `t.len()`, que representa el numero total de caracteres que aun "debemos."

La clave: cuando `count` llega a cero, la ventana actual contiene todos los caracteres de `t`. En ese momento, intento contraer la ventana desde la izquierda, buscando una ventana valida mas pequena.

Cuando muevo el puntero izquierdo hacia adelante, incremento el conteo del caracter que se esta eliminando. Si ese conteo sube por encima de cero, significa que hemos perdido un caracter que `t` realmente necesita, asi que `count` vuelve a subir y la ventana ya no es valida. Dejamos de contraer y retomamos la expansion con el puntero derecho.

### Por Que Funciona

La sutileza esta en como el mapa de frecuencias maneja los caracteres que no estan en `t`. Esos caracteres comienzan con un conteo de 0 en el mapa. Cuando el puntero derecho los encuentra, su conteo se vuelve negativo. Cuando el puntero izquierdo los libera, su conteo sube de vuelta hacia cero pero nunca por encima de el. Entonces `count` (que solo se incrementa cuando un valor del mapa supera cero) nunca se ve afectado por caracteres irrelevantes. El mapa separa naturalmente los caracteres "necesarios" del "ruido."

### Un Ejemplo Paso a Paso

Para `s = "ADOBECODEBANC"`, `t = "ABC"`:
- Inicializamos el mapa: `A:1, B:1, C:1`, `count = 3`
- Right avanza por `A`: mapa `A:0`, count = 2. Ventana: `"A"`
- Right continua por `D`, `O`, `B`: en `B`, mapa `B:0`, count = 1. Ventana: `"ADOB"`
- Right por `E`, `C`: en `C`, mapa `C:0`, count = 0. Ventana: `"ADOBEC"` (longitud 6)
- Ahora contraemos desde la izquierda. Liberamos `A`: mapa `A:1`, count = 1. Ventana invalida. Registramos `start=0, len=6`
- Right continua... Eventualmente encuentra la ventana `"BANC"` (longitud 4), que es la respuesta.

Lo hermoso es que ambos punteros solo avanzan hacia adelante, asi que cada caracter se visita como maximo dos veces: una por `right` y otra por `left`.

## Solucion en Rust

```rust
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        let mut map = [0; 128];

        for &b in t_bytes {
            map[b as usize] += 1;
        }

        let mut left = 0;
        let mut min_len = usize::MAX;
        let mut start_index = 0;
        let mut count = t.len();
        for (right, &char_right) in s_bytes.iter().enumerate() {
            if map[char_right as usize] > 0 {
                count -= 1;
            }
            map[char_right as usize] -= 1;

            while count == 0 {
                let current_len = right - left + 1;

                if current_len < min_len {
                    min_len = current_len;
                    start_index = left;
                }

                let char_left = s_bytes[left];
                map[char_left as usize] += 1;

                if map[char_left as usize] > 0 {
                    count += 1;
                }

                left += 1;
            }
        }

        if min_len == usize::MAX {
            "".to_string()
        } else {
            String::from_utf8_lossy(&s_bytes[start_index..start_index + min_len]).to_string()
        }
    }
}
```

La implementacion en Rust aprovecha trabajar directamente con slices de bytes mediante `as_bytes()`, lo cual evita la sobrecarga de iterar caracter por caracter sobre cadenas UTF-8. El mapa de frecuencias es un arreglo de tamano fijo `[0; 128]` en lugar de un HashMap, dandonos busquedas O(1) sin costo de asignacion de memoria. El uso de `usize::MAX` como centinela para `min_len` es idiomatico en Rust para representar "aun no hay respuesta valida," y la extraccion final del slice `s_bytes[start_index..start_index + min_len]` reconstruye la respuesta limpiamente sin copias innecesarias durante la busqueda en si.

## Conclusion

Este problema es una demostracion de libro de texto de la tecnica de ventana deslizante en su maxima expresion. La idea central es que nunca necesitamos retroceder: ambos punteros avanzan hacia adelante, y el mapa de frecuencias actua como un libro contable compacto que nos dice exactamente cuando tenemos suficiente y cuando hemos perdido demasiado. El espacio es O(1) porque el mapa tiene un tamano fijo de 128 sin importar la entrada, y el tiempo es O(N + M) porque recorremos `s` con ambos punteros y `t` una vez para la inicializacion. A veces la solucion mas elegante no viene de estructuras de datos complejas, sino de una contabilidad cuidadosa de lo que necesitamos y lo que tenemos.
