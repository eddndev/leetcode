---
title: "0008 String to Integer (atoi)"
problemUrl: "https://leetcode.com/problems/string-to-integer-atoi/"
difficulty: "Medium"
pubDate: "2026-01-14"
tags: ["string", "math", "pointer-arithmetic", "overflow-handling"]
complexity:
  time: "O(n)"
  space: "O(1)"
---

# String to Integer (atoi): The Art of Handling Chaos and Overflows

## The Problem
Implement the `myAtoi(string s)` function, which converts a string to a 32-bit signed integer.

The algorithm seems straightforward: read numbers and convert them. However, the difficulty lies in the **noise**: leading whitespace, optional signs (`+` or `-`), garbage characters after the number, and the final boss: **Integer Overflow**.

## The Simplicity Trap
Any first-year student can write a parser that works with `"123"`. But writing one that survives `"   -42 with words"`, `"words and 987"`, or `"-91283472332"` requires a systems engineer mindset.

My approach wasn't to try and "clean" the string (which would cost memory and time), but to **navigate** it with surgical precision using pointers.

## The Implementation: Pointers and Damage Control
In C, iterating with indices (`s[i]`) is safe, but iterating with pointers (`*iterator`) is the idiomatic and efficient way to "consume" a string.

### 1. The `inline` Helper
To keep the code clean and fast, I defined a static inline function. By marking it as `static inline`, we suggest the compiler to embed the code directly, avoiding function call overhead inside the hot loop.

```c
static inline int isDigit(char c) {
    return (c >= '0' && c <= '9');
}

```

### 2. Detecting Overflow

The biggest challenge is detecting when the accumulated number is about to exceed the limits of a 32-bit `int` (`INT_MAX` or `INT_MIN`).
If we wait for it to overflow, the behavior is undefined (or cyclic).

The pragmatic solution: Use a larger container (`long long`, which is guaranteed to be at least 64 bits in this environment) to perform the accumulation. This allows us to "see the future": if the number in the 64-bit container exceeds the 32-bit limit, we can **clamp** it before returning.

```c
#include <limits.h>

// ... helper isDigit ...

int myAtoi(char* s) {
    char *iterator = s;

    // 1. Whitespace skipping
    while ((*iterator) == ' ') {
        iterator++;
    }
    
    // 2. Sign Handling
    // Compact ternary logic: If '-', sign is -1. If '+', it's 1 (true).
    // If neither, we check if it's a digit.
    int sign = ((*iterator) == '-') ? -1 : (*iterator) == '+';
    
    if (sign == 0 && !isDigit(*iterator)) return 0; // Garbage at start
    if (sign == 0) sign = 1; // It was a digit, implicit positive sign
    else iterator++; // It was an explicit sign, advance

    // 3. Conversion and Overflow Protection
    long long acumulator = 0; 
    while (isDigit(*iterator)) {
        int digit = (*iterator) - '0';
        acumulator = acumulator * 10 + digit;

        // THE GUARDIAN: Boundary check at every step
        // Note: INT_MAX is 2147483647, INT_MIN is -2147483648
        // The magnitude of min is INT_MAX + 1
        if (sign == -1 && acumulator > (long long)INT_MAX + 1) return INT_MIN;
        if (sign == 1 && acumulator > INT_MAX) return INT_MAX; 

        iterator++;
    }

    return acumulator * sign;
}

```

## Conclusion

This exercise demonstrates that text parsing is not just about character logic, but about **defensive arithmetic**.
Using `long long` as a safety buffer and pointer navigation allows us to process the string in a single pass, without extra memory, and robustly handling the limits of 32-bit architecture.