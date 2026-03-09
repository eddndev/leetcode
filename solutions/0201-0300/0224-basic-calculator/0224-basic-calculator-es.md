---
title: "0224 Basic Calculator - ES"
problemUrl: "https://leetcode.com/problems/basic-calculator/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["stack", "string", "math", "recursion"]
complexity:
  time: "O(N) donde N es la longitud de la cadena"
  space: "O(N) para la pila en el peor caso de parentesis anidados"
---

# Desenredando Parentesis: Una Calculadora Desde Cero

## El Problema
Dada una cadena `s` que representa una expresion matematica valida con digitos, `+`, `-`, parentesis `(` y `)`, y espacios, evaluar la expresion y devolver su resultado. No se permite usar funciones de evaluacion integradas como `eval()`.

## La Trampa de los Parentesis Anidados

A primera vista, sumar y restar numeros parece trivial. Pero los parentesis cambian todo. Una expresion como `1 - (2 + 3 - (4 + 5))` requiere recordar el contexto exterior mientras evaluamos la subexpresion interior. Cada apertura de parentesis crea un nuevo "mundo" con su propio resultado parcial, y al cerrarlo debemos fusionar ese resultado con el contexto que dejamos atras.

Un enfoque ingenuo seria buscar los parentesis mas internos, evaluarlos, reemplazar la subexpresion por su resultado, y repetir. Pero esto implica multiples pasadas sobre la cadena y manipulacion costosa de strings. La clave es darse cuenta de que podemos procesar la cadena en una sola pasada si tenemos un mecanismo para *pausar* y *reanudar* el calculo exterior cuando encontramos parentesis.

## La Estrategia: Una Pila como Memoria de Contexto

### El Flujo Natural

Mi idea central fue tratar la pila como una memoria de contexto. Mientras recorro la cadena de izquierda a derecha, mantengo tres variables: `result` (el resultado acumulado actual), `current_number` (el numero que estoy construyendo digito a digito) y `sign` (el signo que precede al siguiente numero, `+1` o `-1`).

Cuando encuentro un digito, lo agrego al numero que estoy construyendo. Cuando encuentro `+` o `-`, actualizo el signo. Cuando el numero esta completo (porque llego un operador o un parentesis), lo incorporo al resultado con `result += sign * current_number`.

### Parentesis como Puntos de Guardado

Lo elegante surge con los parentesis:

1. **Al abrir `(`**: Guardo el estado actual en la pila -- primero `result`, luego `sign` -- y reinicio ambos. Es como hacer una "instantanea" del calculo exterior antes de sumergirme en la subexpresion. El `result` se reinicia a `0` y el `sign` a `+1` porque dentro de los parentesis empiezo un calculo fresco.

2. **Al cerrar `)`**: Recupero el signo y el resultado anteriores de la pila. El resultado de la subexpresion que acabo de calcular se multiplica por el signo guardado y se suma al resultado anterior. En esencia, `resultado_exterior + signo_guardado * resultado_subexpresion`.

### Un Ejemplo Concreto

Con `s = "1 - (2 + 3 - (4 + 5))"`:

```
Caracter '1': current_number=1
Caracter ' ': (ignorado)
Caracter '-': result += 1*1 = 1. sign = -1
Caracter ' ': (ignorado)
Caracter '(': push result=1, push sign=-1. result=0, sign=1
  Caracter '2': current_number=2
  Caracter '+': result += 1*2 = 2. sign = 1
  Caracter '3': current_number=3
  Caracter ' ': (ignorado)
  Caracter '-': result += 1*3 = 5. sign = -1
  Caracter ' ': (ignorado)
  Caracter '(': push result=5, push sign=-1. result=0, sign=1
    Caracter '4': current_number=4
    Caracter '+': result += 1*4 = 4. sign = 1
    Caracter '5': current_number=5
  Caracter ')': result += 1*5 = 9. pop sign=-1, pop result=5. result = 5 + (-1)*9 = -4
Caracter ')': pop sign=-1, pop result=1. result = 1 + (-1)*(-4) = 5

Resultado final: 5
```

Verificacion: `1 - (2 + 3 - (4 + 5)) = 1 - (5 - 9) = 1 - (-4) = 5`.

### Por Que la Pila Es Suficiente

Cada par de parentesis anidados agrega exactamente dos elementos a la pila (resultado y signo), asi que la profundidad de la pila es proporcional al nivel maximo de anidamiento. Dado que los parentesis estan balanceados por restriccion del problema, nunca nos quedamos con datos huerfanos en la pila. Y como procesamos la cadena caracter por caracter sin retroceder, el tiempo total es lineal.

## Solucion en Rust

```rust
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut result = 0;
        let mut current_number = 0;
        let mut sign = 1;

        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            if c.is_ascii_digit() {
                current_number = 0;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    current_number = current_number * 10 + (chars[i] as i32 - '0' as i32);
                    i += 1;
                }
                result += sign * current_number;
                i -= 1;
            } else if c == '+' {
                sign = 1;
            } else if c == '-' {
                sign = -1;
            } else if c == '(' {
                stack.push(result);
                stack.push(sign);
                result = 0;
                sign = 1;
            } else if c == ')' {
                let prev_sign = stack.pop().unwrap();
                let prev_result = stack.pop().unwrap();
                result = prev_result + (prev_sign * result);
            }
            i += 1;
        }

        result
    }
}
```

La implementacion recorre la cadena con un indice `i` que avanza manualmente para poder consumir numeros de multiples digitos en un bucle interno. Cuando encuentra un digito, entra en un sub-bucle que acumula el numero completo multiplicando por 10 y sumando cada digito. El `i -= 1` al final del bloque de digitos compensa el `i += 1` general del bucle exterior, evitando saltarse el caracter siguiente. Los operadores `+` y `-` simplemente actualizan la variable `sign` sin tocar `result` -- el numero se incorporara al resultado cuando se lea completo. La pila almacena pares `(result, sign)` como dos enteros separados en orden inverso: primero `result`, luego `sign`, de modo que al hacer `pop()` recuperamos primero el signo y luego el resultado acumulado. La formula `prev_result + (prev_sign * result)` fusiona elegantemente la subexpresion evaluada con el contexto exterior, sin importar cuantos niveles de anidamiento haya.

## Conclusion

El problema de la calculadora basica es un ejercicio clasico sobre como usar una pila para manejar contextos anidados. Al reconocer que los parentesis actuan como puntos de guardado y restauracion, la solucion se reduce a un recorrido lineal donde la pila preserva el estado exterior mientras evaluamos subexpresiones internas. El resultado es un algoritmo O(N) en tiempo y espacio que procesa la cadena en una sola pasada, sin necesidad de recursion explicita ni analisis sintactico complejo.
