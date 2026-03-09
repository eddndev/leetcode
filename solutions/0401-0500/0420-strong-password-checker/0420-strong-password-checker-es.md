---
title: "0420 Strong Password Checker - ES"
problemUrl: "https://leetcode.com/problems/strong-password-checker/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["greedy", "string"]
complexity:
  time: "O(N) donde N es la longitud de la contraseña"
  space: "O(N) para almacenar las secuencias repetidas"
---

# El Acto de Equilibrio del Cerrajero en Tres Frentes

## El Problema
Dada una cadena `password`, devolver el numero minimo de pasos requeridos para convertirla en una contraseña fuerte. Una contraseña fuerte tiene al menos 6 caracteres, como maximo 20 caracteres, contiene al menos una letra minuscula, al menos una letra mayuscula, al menos un digito, y no contiene tres caracteres repetidos consecutivos. En un paso, se puede insertar un caracter, eliminar un caracter o reemplazar un caracter.

## La Intuicion Inicial

A primera vista, este problema parece que deberia ceder ante una formulacion limpia de programacion dinamica. Pero la interaccion entre tres restricciones diferentes -- requisitos de longitud, requisitos de tipos de caracteres y restricciones de caracteres repetidos -- hace que los espacios de estado de DP exploten. Lo que hace que este problema sea verdaderamente dificil es que las tres operaciones (insertar, eliminar, reemplazar) interactuan con las restricciones de maneras fundamentalmente diferentes, y la estrategia optima cambia dramaticamente dependiendo de si la contraseña es demasiado corta, esta bien, o es demasiado larga.

La percepcion clave que desbloquea la solucion es dividir el analisis en tres regimenes basados en la longitud, y dentro de cada regimen determinar como las operaciones trabajan juntas.

## Los Tres Regimenes

### Demasiado Corta (longitud < 6)

Cuando la contraseña tiene menos de 6 caracteres, necesito agregar caracteres. Cada insercion hace simultaneamente dos cosas: me acerca a la longitud minima de 6 y puede corregir un tipo de caracter faltante. Tambien puede romper una secuencia repetida insertando un caracter diferente en el medio. Asi que la respuesta es simplemente el maximo entre `missing_types` y `6 - n`. Ambas restricciones necesitan satisfacerse, y dado que cada insercion puede abordar ambas, simplemente tomo la que demande mas operaciones.

### Longitud Correcta (6 <= longitud <= 20)

Cuando la longitud ya esta dentro de los limites, nunca necesito insertar ni eliminar. La unica operacion que importa es el reemplazo. Una secuencia de `k` caracteres identicos requiere `floor(k / 3)` reemplazos para romperla -- coloco un caracter diferente cada tres posiciones, lo que evita que tres caracteres consecutivos coincidan. La respuesta es el maximo entre `missing_types` y el total de reemplazos necesarios. De nuevo, los reemplazos pueden satisfacer ambos objetivos simultaneamente: reemplazar un caracter puede introducir un tipo faltante mientras tambien rompe una secuencia repetida.

### Demasiado Larga (longitud > 20)

Aqui es donde vive la verdadera complejidad. Definitivamente necesito `n - 20` eliminaciones para reducir la longitud. Pero las eliminaciones tambien pueden reducir el numero de reemplazos necesarios para secuencias repetidas, y quiero usarlas de la manera mas eficiente posible.

Esta es la observacion critica: para una secuencia repetida de longitud `k`, necesito `floor(k / 3)` reemplazos. Pero si elimino caracteres de esa secuencia, puedo reducir `k` y asi reducir la cuenta de reemplazos. La eficiencia de la eliminacion depende de `k mod 3`:

- Si `k mod 3 == 0`: eliminar solo 1 caracter reduce la cuenta de reemplazos en 1. Estos son los objetivos mas eficientes para eliminaciones.
- Si `k mod 3 == 1`: eliminar 2 caracteres reduce la cuenta de reemplazos en 1. Aun vale la pena pero es menos eficiente.
- Si `k mod 3 == 2` (o cualquier otro caso): necesito eliminar 3 caracteres para reducir la cuenta de reemplazos en 1. Este es el uso menos eficiente de las eliminaciones.

Asi que priorizo las eliminaciones de forma greedy: primero gasto eliminaciones en secuencias donde `k mod 3 == 0` (1 eliminacion ahorra 1 reemplazo), luego en secuencias donde `k mod 3 == 1` (2 eliminaciones ahorran 1 reemplazo), y finalmente uso las eliminaciones restantes a una tasa de 3 por 1 en todo lo demas.

Despues de aplicar las eliminaciones de manera optima, los reemplazos que queden aun deben realizarse. Y la respuesta final es `eliminaciones + max(missing_types, reemplazos_restantes)`.

## Por Que Funciona el Greedy

La prioridad greedy para las eliminaciones es correcta porque tenemos un presupuesto fijo de eliminaciones (`n - 20`) y queremos maximizar la reduccion en reemplazos. Cada secuencia "mod 0" nos da la mejor tasa de cambio: una eliminacion por un reemplazo menos. Cada secuencia "mod 1" es la siguiente mejor: dos eliminaciones por un reemplazo menos. Todo lo demas cuesta tres eliminaciones por reemplazo ahorrado. Al gastar nuestro presupuesto en este orden, minimizamos los reemplazos sobrantes.

El `max(missing_types, ...)` aparece en cada regimen porque los tipos de caracteres faltantes y las correcciones estructurales son restricciones independientes que a veces pueden satisfacerse con la misma operacion pero nunca interfieren entre si.

## Solucion en Rust

```rust
use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let chars: Vec<char> = password.chars().collect();
        let n = chars.len() as i32;

        let has_lower = chars.iter().any(|c| c.is_ascii_lowercase());
        let has_upper = chars.iter().any(|c| c.is_ascii_uppercase());
        let has_digit = chars.iter().any(|c| c.is_ascii_digit());

        let missing_types = !has_lower as i32 + !has_upper as i32 + !has_digit as i32;

        let mut replace = 0;
        let mut one_seq = Vec::new();
        let mut two_seq = Vec::new();

        let mut i = 2;
        while i < n {
            if chars[i as usize] == chars[(i - 1) as usize]
                && chars[i as usize] == chars[(i - 2) as usize]
            {
                let mut length = 2;
                while i < n && chars[i as usize] == chars[(i - 1) as usize] {
                    length += 1;
                    i += 1;
                }
                replace += length / 3;
                if length % 3 == 0 {
                    one_seq.push(length);
                } else if length % 3 == 1 {
                    two_seq.push(length);
                }
            } else {
                i += 1;
            }
        }

        if n < 6 {
            max(missing_types, 6 - n)
        } else if n <= 20 {
            max(missing_types, replace)
        } else {
            let delete_needed = n - 20;
            let mut delete_left = delete_needed;

            replace -= min(delete_left, one_seq.len() as i32 * 1) / 1;
            delete_left = max(0, delete_left - one_seq.len() as i32 * 1);

            replace -= min(delete_left, two_seq.len() as i32 * 2) / 2;
            delete_left = max(0, delete_left - two_seq.len() as i32 * 2);

            replace -= delete_left / 3;

            delete_needed + max(missing_types, replace)
        }
    }
}
```

La solucion comienza escaneando la contraseña para detectar la presencia de tipos de caracteres. La conversion de booleano a entero `!has_lower as i32` cuenta elegantemente los tipos faltantes. Luego recorre la cadena detectando secuencias repetidas de tres o mas caracteres identicos. Para cada secuencia, calcula `length / 3` reemplazos necesarios y clasifica la secuencia por `length % 3` en `one_seq` (mod 0, donde 1 eliminacion ahorra 1 reemplazo) o `two_seq` (mod 1, donde 2 eliminaciones ahorran 1 reemplazo). Las secuencias donde `length % 3 == 2` no necesitan seguimiento especial porque solo se benefician de eliminaciones a la tasa generica de 3 por 1.

En la rama de demasiado larga, el presupuesto de eliminaciones se gasta en orden de prioridad. La expresion `min(delete_left, one_seq.len() as i32 * 1) / 1` limita las eliminaciones al presupuesto o al numero de secuencias mod-0. La division por 1 es una operacion nula pero mantiene el patron: para secuencias mod-1 la division es por 2, reflejando la tasa de cambio de 2 eliminaciones por reemplazo ahorrado. Finalmente, `delete_left / 3` maneja cualquier presupuesto restante a la tasa de 3 por 1 aplicada uniformemente a las secuencias sobrantes.

## Conclusion

Strong Password Checker es uno de esos problemas raros donde la dificultad no reside en ninguna tecnica algoritmica individual sino en la cuidadosa orquestacion de multiples restricciones que interactuan entre si. No hay ninguna estructura de datos sofisticada, ninguna recursion profunda, ninguna identidad matematica ingeniosa -- solo un analisis exhaustivo de casos combinado con un esquema de prioridad greedy para asignar eliminaciones. La descomposicion en tres regimenes mantiene la logica manejable, y la clasificacion mod-3 de secuencias repetidas transforma una optimizacion aparentemente intratable en un recorrido greedy en tiempo lineal. Lo que hace a este problema hermoso, y frustrante, es que la respuesta es conceptualmente simple una vez que la ves, pero descubrir la descomposicion correcta demanda paciencia y precision.
