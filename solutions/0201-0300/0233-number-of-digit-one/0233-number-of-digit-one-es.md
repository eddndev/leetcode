---
title: "0233 Number of Digit One - ES"
problemUrl: "https://leetcode.com/problems/number-of-digit-one/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["math", "digit-dp", "combinatorics"]
complexity:
  time: "O(log N) donde N es el numero de entrada"
  space: "O(1) espacio extra constante"
---

# Contando Cada Uno Escondido

## El Problema
Dado un entero `n`, contar el numero total de veces que el digito `1` aparece en todos los enteros no negativos menores o iguales a `n`. Por ejemplo, dado `n = 13`, los enteros del 1 al 13 son `1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13`, y el digito `1` aparece un total de 6 veces.

## La Trampa de la Fuerza Bruta

El enfoque ingenuo es obvio: iterar del 1 al `n`, extraer cada digito de cada numero, y contar los unos. Pero con `n` alcanzando hasta 10^9, esto significa miles de millones de numeros con hasta 10 digitos cada uno. Es demasiado lento. La verdadera pregunta es si podemos calcular la respuesta matematicamente, sin visitar cada numero.

## La Clave: Contar Posicion por Posicion

En lugar de contar unos *por numero*, cuento unos *por posicion de digito*. Para cada posicion -- unidades, decenas, centenas, y asi sucesivamente -- calculo cuantas veces aparece el digito `1` a lo largo de todos los numeros del 1 al `n`. El total es simplemente la suma a traves de todas las posiciones.

### Diseccionando una Posicion

Consideremos una posicion especifica con valor posicional `i` (donde `i` cicla a traves de 1, 10, 100, ...). Para cualquier numero `n`, lo divido en tres partes relativas a esta posicion:

- **prefijo**: los digitos por encima de la posicion `i`, calculado como `n / (i * 10)`
- **digito**: el digito en la posicion `i`, calculado como `(n / i) % 10`
- **sufijo**: los digitos por debajo de la posicion `i`, calculado como `n % i`

La cantidad de unos en esta posicion depende enteramente de lo que sea `digito`:

1. **Si digito es 0**: Los unos en esta posicion provienen solo de ciclos completos del prefijo. Cada ciclo completo de los digitos superiores contribuye exactamente `i` unos (uno por cada combinacion de los digitos del sufijo). Asi que la cuenta es `prefijo * i`.

2. **Si digito es 1**: Obtenemos todos los unos de los ciclos completos (`prefijo * i`), mas un ciclo parcial. El ciclo parcial contribuye `sufijo + 1` unos -- el digito actual es 1 para valores del sufijo de 0 hasta `sufijo`. Asi que la cuenta es `prefijo * i + sufijo + 1`.

3. **Si digito es 2 o mayor**: El ciclo actual esta completamente terminado. La cuenta es `(prefijo + 1) * i` -- todos los ciclos completos incluyendo el actual.

### Un Recorrido con n = 314

Tracemos `n = 314`:

**Posicion de unidades (i=1):** prefijo = 31, digito = 4, sufijo = 0. Como digito > 1: cuenta = (31 + 1) * 1 = 32.

**Posicion de decenas (i=10):** prefijo = 3, digito = 1, sufijo = 4. Como digito = 1: cuenta = 3 * 10 + (4 + 1) = 35.

**Posicion de centenas (i=100):** prefijo = 0, digito = 3, sufijo = 14. Como digito > 1: cuenta = (0 + 1) * 100 = 100.

**Total:** 32 + 35 + 100 = 167 unos en todos los enteros del 1 al 314.

### Por Que Funciona

La estructura matematica proviene de como ciclan los numeros decimales. En cualquier rango contiguo de 10 numeros (digamos 0-9, 10-19, etc.), el digito de las unidades es `1` exactamente una vez. En cualquier rango de 100 numeros, el digito de las decenas es `1` exactamente 10 veces. En general, en cualquier rango de `10 * i` numeros, la posicion `i` tiene un `1` exactamente `i` veces. El prefijo nos dice cuantos ciclos completos han pasado, y el digito y el sufijo nos dicen que tan lejos estamos en el ciclo parcial actual.

## Solucion en Rust

```rust
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }

        let n = n as i64;
        let mut count = 0;
        let mut i = 1;

        while i <= n {
            let prefix = n / (i * 10);
            let digit = (n / i) % 10;
            let suffix = n % i;

            if digit == 0 {
                count += prefix * i;
            } else if digit == 1 {
                count += prefix * i + (suffix + 1);
            } else {
                count += (prefix + 1) * i;
            }

            if i > n / 10 {
                break;
            }
            i *= 10;
        }

        count as i32
    }
}
```

La implementacion convierte `n` a `i64` desde el principio para evitar desbordamiento al calcular `i * 10` cerca del limite superior de `i32`. La variable `i` representa el valor posicional actual y se multiplica por 10 en cada iteracion, asi que el bucle se ejecuta como maximo 10 veces para `n` hasta 10^9. La guarda `if i > n / 10 { break; }` previene que `i` desborde en la siguiente multiplicacion -- una vez que `i` supera `n / 10`, el siguiente `i *= 10` superaria a `n` y la condicion del bucle `i <= n` lo terminaria de todos modos, pero esta salida temprana evita la multiplicacion por completo. La bifurcacion de tres vias sobre `digit` codifica directamente la formula matematica: cero significa que no hay contribucion de ciclo parcial, uno significa una contribucion parcial de `sufijo + 1`, y cualquier valor mayor significa que el ciclo completo esta terminado. La conversion final de vuelta a `i32` es segura porque la respuesta siempre esta acotada por `n * log10(n)`, bien dentro del rango de `i32` para entradas validas.

## Conclusion

El problema Number of Digit One recompensa el pensamiento matematico sobre la complejidad algoritmica. Al cambiar la perspectiva de "cuantos unos contiene cada numero" a "cuantos unos contribuye cada posicion," transformamos un problema que parece requerir enumeracion O(N) en uno resoluble en tiempo O(log N) con espacio constante. Los tres casos -- digito por debajo, en, o por encima de 1 -- capturan la estructura completa de ciclos de los numeros decimales con precision quirurgica. Sin estructuras de datos, sin recursion, sin memoizacion -- solo aritmetica y un bucle limpio a traves de las posiciones de digitos.
