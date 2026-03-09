---
title: "0052 N-Queens II - ES"
problemUrl: "https://leetcode.com/problems/n-queens-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["backtracking", "bitmask", "recursion"]
complexity:
  time: "O(N!)"
  space: "O(1)"
---

# Contando Coronas Sin Construir Tronos

## El Problema
Dado un entero `n`, devolver la cantidad de formas distintas de colocar `n` reinas en un tablero de `n x n` de manera que ninguna reina ataque a otra. A diferencia de N-Queens I, no necesitamos devolver las configuraciones del tablero -- solo el conteo.

## De Tableros a Aritmetica Pura

En N-Queens I, el trabajo principal va a construir y almacenar cada configuracion valida del tablero. Pero cuando la pregunta es simplemente "cuantas hay?", toda esa contabilidad desaparece. Sin vector de tablero, sin funcion de formato, sin lista de resultados. El problema entero colapsa en un unico contador recursivo impulsado por tres bitmasks. Lo que queda es la forma mas pura del algoritmo: propagacion de restricciones a traves de bits y nada mas.

## Tres Mascaras, Un Contador

El enfoque es identico al de N-Queens I en espiritu, pero despojado de todo lo innecesario. Tres enteros codifican toda la informacion que necesitamos:

- `cols` registra que columnas ya estan ocupadas por una reina.
- `diags` registra las diagonales de arriba-izquierda a abajo-derecha bajo ataque. Cuando descendemos una fila, estas amenazas se desplazan un bit a la izquierda (`<< 1`).
- `anti_diags` registra las diagonales de arriba-derecha a abajo-izquierda bajo ataque. Estas se desplazan un bit a la derecha (`>> 1`) al bajar.

Las posiciones disponibles en la fila actual se calculan en una sola expresion:

```
available = ((1 << n) - 1) & NOT(cols OR diags OR anti_diags)
```

La mascara `(1 << n) - 1` nos mantiene dentro de los limites del tablero. El `NOT` de las restricciones combinadas nos da exactamente las casillas seguras. Sin bucles, sin busquedas en arreglos -- una operacion bitwise y se exactamente donde puedo colocar la siguiente reina.

## Aislando Posiciones con el Truco del Bit Mas Bajo

Para iterar sobre las posiciones disponibles, uso la extraccion clasica del bit menos significativo:

```
position = available & -available       // Aisla el bit mas bajo encendido
available = available & (available - 1) // Apaga ese bit
```

Cada `position` representa una columna segura. Recurro con las mascaras actualizadas y, al retornar, la siguiente iteracion del while toma la siguiente posicion disponible. No hay un paso explicito de deshacer porque las mascaras se pasan por valor -- cada llamada recursiva recibe su propia copia de las restricciones. El backtracking es implicito y libre de asignaciones de memoria.

## El Caso Base: Fila Igual a N

Cuando `row == n`, hemos colocado una reina en cada fila sin conflictos. Eso significa una configuracion valida mas encontrada, asi que retornamos 1. Los valores de retorno burbujean a traves de la recursion, acumulando el conteo total sin jamas materializar un tablero.

## Solucion en Rust

```rust
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        Self::solve(n, 0, 0, 0, 0)
    }

    fn solve(n: i32, row: i32, cols: i32, diags: i32, anti_diags: i32) -> i32 {
        if row == n {
            return 1;
        }

        let mut count = 0;

        let mut available = ((1 << n) - 1) & !(cols | diags | anti_diags);

        while available != 0 {
            let position = available & -available;

            available = available & (available - 1);

            count += Self::solve(
                n,
                row + 1,
                cols | position,
                (diags | position) << 1,
                (anti_diags | position) >> 1,
            );
        }

        count
    }
}
```

Lo que mas me satisface de esta solucion es lo poco estado que carga. No hay ninguna estructura mutable pasada a traves de la recursion -- sin vectores, sin arreglos, sin representacion del tablero en absoluto. Los cinco parametros de `solve` son enteros simples, pasados por valor. Cada llamada recursiva recibe su propia instantanea de restricciones, asi que el backtracking ocurre automaticamente cuando la llamada retorna. El algoritmo entero vive en aritmetica.

El truco de desplazamiento diagonal merece atencion: `(diags | position) << 1` y `(anti_diags | position) >> 1` propagan las amenazas diagonales hacia abajo en una sola operacion por direccion. Para cuando la siguiente llamada a `solve` se ejecuta, las mascaras ya reflejan donde ataca cada reina previamente colocada. Sin post-procesamiento, sin arreglos de restricciones separados -- solo enteros desplazados.

La complejidad espacial cae a O(1) de espacio auxiliar (mas alla de la pila de recursion de profundidad `n`) porque no almacenamos nada excepto los parametros enteros en cada frame. Comparemos eso con N-Queens I, que necesita espacio O(N^2) solo para las configuraciones del tablero.

## Conclusion

N-Queens II es un hermoso ejemplo de como eliminar requisitos de salida puede simplificar una solucion dramaticamente. El mismo backtracking con bitmasks de N-Queens I, despojado de la construccion y almacenamiento del tablero, se convierte en una recursion pura de conteo con cero asignaciones de memoria. Tres mascaras propagan restricciones, el truco del bit menos significativo itera sobre candidatos, y la semantica de paso por valor maneja el backtracking de forma gratuita. El resultado es un algoritmo que cuenta todas las configuraciones validas usando nada mas que aritmetica de enteros y la pila de llamadas.
