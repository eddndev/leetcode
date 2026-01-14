---
title: "0008 String to Integer (atoi) - ES"
problemUrl: "https://leetcode.com/problems/string-to-integer-atoi/"
difficulty: "Medium"
pubDate: "2026-01-14"
tags: ["string", "math", "pointer-arithmetic", "overflow-handling"]
complexity:
  time: "O(n)"
  space: "O(1)"
---

## El Problema
Implementar la función `myAtoi(string s)`, que convierte una cadena a un entero de 32 bits con signo.

El algoritmo parece directo: leer números y convertirlos. Sin embargo, la dificultad radica en el **ruido**: espacios en blanco iniciales, signos opcionales (`+` o `-`), caracteres basura después del número y, el jefe final: **números que no caben en un entero (`Integer Overflow`)**.

## La Trampa de la Simplicidad
Cualquier estudiante de primer semestre puede escribir un parser que funcione con `"123"`. Pero escribir uno que sobreviva a `"   -42 with words"`, `"words and 987"` o `"-91283472332"` requiere mentalidad de ingeniero de sistemas.

Mi enfoque no fue tratar de "limpiar" el string (lo cual costaría memoria y tiempo), sino **navegarlo** con precisión quirúrgica usando punteros.

## La Implementación: Punteros y Control de Daños
En C, iterar con índices (`s[i]`) es seguro, pero iterar con punteros (`*iterator`) es la forma idiomática y eficiente de "consumir" una cadena.

### 1. La Función Auxiliar `inline`
Para mantener el código limpio y rápido, definí una función estática en línea. Al marcarla como `static inline`, le sugerimos al compilador que incruste el código directamente, evitando el overhead de la llamada a función dentro del bucle principal.

```c
static inline int isDigit(char c) {
    return (c >= '0' && c <= '9');
}

```

### 2. Detectando el Desbordamiento (Overflow)

El mayor desafío es detectar cuándo el número acumulado va a exceder los límites de un `int` de 32 bits (`INT_MAX` o `INT_MIN`).
Si esperamos a que se desborde, el comportamiento es indefinido (o cíclico).

La solución pragmática: Usar un contenedor más grande (`long long`, que son 64 bits garantizados en este entorno) para realizar la acumulación. Esto nos permite "ver el futuro": si el número en el contenedor de 64 bits excede el límite de 32 bits, podemos **saturarlo** (clamping) antes de devolverlo.

```c
#include <limits.h>

// ... helper isDigit ...

int myAtoi(char* s) {
    char *iterator = s;

    // 1. Descartar espacios en blanco (Whitespace skipping)
    while ((*iterator) == ' ') {
        iterator++;
    }
    
    // 2. Manejo del Signo
    // Lógica ternaria compacta: Si es '-', sign es -1. Si es '+', es 1 (true).
    // Si no es ninguno, verificamos si es dígito.
    int sign = ((*iterator) == '-') ? -1 : (*iterator) == '+';
    
    if (sign == 0 && !isDigit(*iterator)) return 0; // Basura al inicio
    if (sign == 0) sign = 1; // Era un dígito, el signo es positivo implícito
    else iterator++; // Era un signo explícito, avanzamos

    // 3. Conversión y Protección contra Overflow
    long long acumulator = 0; 
    while (isDigit(*iterator)) {
        int digit = (*iterator) - '0';
        acumulator = acumulator * 10 + digit;

        // EL GUARDIÁN: Chequeo de límites en cada paso
        // Nota: INT_MAX es 2147483647, INT_MIN es -2147483648
        // La magnitud del negativo es INT_MAX + 1
        if (sign == -1 && acumulator > (long long)INT_MAX + 1) return INT_MIN;
        if (sign == 1 && acumulator > INT_MAX) return INT_MAX; 

        iterator++;
    }

    return acumulator * sign;
}

```

## Conclusión

Este ejercicio demuestra que parsear texto no se trata solo de lógica de caracteres, sino de **aritmética defensiva**.
El uso de `long long` como buffer de seguridad y la navegación por punteros nos permite procesar la cadena en una sola pasada, sin memoria extra y manejando robustamente los límites de la arquitectura de 32 bits.