---
title: "0007 Reverse Integer - ES"
problemUrl: "https://leetcode.com/problems/reverse-integer/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["math", "overflow-handling"]
complexity:
  time: "O(log x)"
  space: "O(1)"
---

# Reverse Integer: Invertir Dígitos sin Explotar

## El Problema
Dado un entero con signo de 32 bits, devolver el número con sus dígitos invertidos. Si el resultado se desborda fuera del rango de un entero con signo de 32 bits (`[-2^31, 2^31 - 1]`), devolver `0`.

A primera vista parece un ejercicio trivial de manipulación numérica. Extraer dígitos con módulo, reconstruir el número multiplicando por 10... algo que cualquiera puede hacer en un par de minutos. Pero la trampa está en lo que no se ve: **el desbordamiento (overflow)**.

## La Trampa del Overflow

El rango de un `i32` va de `-2,147,483,648` a `2,147,483,647`. Invertir un número como `1,999,999,999` produce `9,999,999,991`, que excede el máximo. Si no controlamos esto, el programa colapsa o devuelve basura.

La pregunta clave es: **¿cómo detectar el overflow antes de que ocurra?**

### El Enfoque Clásico (C/C++)
En C, la estrategia típica es comparar manualmente contra `INT_MAX / 10` antes de multiplicar. Es funcional pero verboso y propenso a errores de signo.

### El Enfoque Idiomático (Rust)
Rust, por diseño, nos obliga a pensar en el overflow. En modo debug, las operaciones aritméticas que desbordan provocan un `panic`. En modo release, hacen wrapping silencioso. Ninguna de las dos opciones es lo que queremos.

La solución elegante: usar los métodos `checked_mul` y `checked_add` que devuelven `Option<i32>`. Si la operación es segura, obtenemos `Some(valor)`. Si desborda, obtenemos `None`. Encadenarlos con `and_then` nos da una pipeline limpia y segura.

Lo que en C requiere comparaciones manuales contra límites, en Rust se convierte en una sola expresión declarativa dentro de un `match`.

## La Solución

El algoritmo es directo:
1. Extraer el último dígito con `num % 10`.
2. Reducir el número con `num /= 10`.
3. Acumular en `rev` multiplicando por 10 y sumando el dígito, pero **protegiendo** cada operación contra overflow.
4. Si en cualquier paso la multiplicación o la suma desborda, devolvemos `0` inmediatamente.

Un detalle sutil: en Rust, el operador `%` preserva el signo del dividendo. Si `num` es negativo, `digit` también lo será. Esto significa que no necesitamos manejar el signo por separado: `checked_add` con un dígito negativo efectivamente resta, y la detección de overflow funciona correctamente para ambos extremos del rango.

```rust
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut rev = 0i32;

        while num != 0 {
            let digit = num % 10;
            num /= 10;

            match rev.checked_mul(10).and_then(|v| v.checked_add(digit)) {
                Some(val) => rev = val,
                None => return 0,
            }
        }

        rev
    }
}
```

## Conclusión

Este problema es un recordatorio de que la aritmética de enteros no es tan inocente como parece. La diferencia entre un programa correcto y uno con comportamiento indefinido puede ser una sola multiplicación sin guardia.

Rust convierte lo que en otros lenguajes es disciplina del programador en una **garantía del sistema de tipos**. Los métodos `checked_*` no son un lujo: son la forma correcta de trabajar con aritmética que puede desbordar.
