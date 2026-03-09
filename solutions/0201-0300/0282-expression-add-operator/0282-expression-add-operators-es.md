---
title: "0282 Expression Add Operators - ES"
problemUrl: "https://leetcode.com/problems/expression-add-operators/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["backtracking", "math", "string", "recursion"]
complexity:
  time: "O(N * 4^N) donde N es la longitud de la cadena"
  space: "O(N) por la profundidad de la recursion"
---

# Insertando Operadores en el Caos Numerico

## El Problema
Dada una cadena `num` que contiene solo digitos y un entero `target`, devolver todas las posibilidades de insertar los operadores binarios `+`, `-` y `*` entre los digitos de `num` para que la expresion resultante se evalue al valor `target`. Los operandos en las expresiones retornadas no deben contener ceros a la izquierda.

## La Explosion Combinatoria

Este problema parece simple en la superficie: tomar una cadena de digitos e insertar operadores entre ellos. Pero la complejidad real se esconde en dos frentes. Primero, entre cada par de digitos consecutivos tengo cuatro opciones: no insertar nada (concatenar los digitos para formar un numero mas largo), insertar `+`, insertar `-`, o insertar `*`. Con `N` digitos, eso genera hasta `4^N` combinaciones posibles. Segundo, la multiplicacion rompe la asociatividad izquierda natural de la evaluacion: `2+3*4` no es `(2+3)*4 = 20` sino `2+(3*4) = 14`, porque la multiplicacion tiene mayor precedencia.

Mi primer instinto fue generar todas las expresiones posibles y evaluarlas con un parser, pero eso seria ineficiente. Lo que necesitaba era evaluar la expresion *mientras la construyo*, manteniendo suficiente informacion para manejar la precedencia de la multiplicacion correctamente.

## La Estrategia: Backtracking con Memoria del Ultimo Operando

### La Clave de la Multiplicacion

El truco central de este problema es rastrear el ultimo operando utilizado. Cuando proceso `2 + 3` y el valor acumulado es `5`, si el siguiente operador es `*4`, no puedo simplemente calcular `5 * 4 = 20`. Necesito *deshacer* la suma anterior: `(5 - 3) + (3 * 4) = 2 + 12 = 14`. Eso es exactamente lo que significa respetar la precedencia de operadores sin un parser completo.

Por eso mi funcion de backtracking mantiene dos valores:
- `current_val`: el resultado acumulado de la expresion hasta el momento
- `last_operand`: el ultimo operando aplicado, necesario para deshacer la operacion anterior en caso de multiplicacion

### Particionando la Cadena

En cada paso de la recursion, considero todas las posibles subcadenas que comienzan en el indice actual. La subcadena `num[index..=i]` forma un operando candidato. Si ese operando tiene mas de un digito y comienza con '0', lo descarto inmediatamente -- los ceros a la izquierda no estan permitidos.

Para el primer operando (cuando `index == 0`), no hay operador que insertar: simplemente uso el numero como valor inicial y como ultimo operando. Para los operandos subsiguientes, pruebo las tres operaciones:

- **Suma**: `current_val + val`, con `last_operand = val`
- **Resta**: `current_val - val`, con `last_operand = -val`
- **Multiplicacion**: `(current_val - last_operand) + (last_operand * val)`, con `last_operand = last_operand * val`

Notar que en la resta, el ultimo operando se almacena como `-val`. Esto es intencional: si despues viene una multiplicacion, necesito deshacer la resta completa, incluyendo el signo.

### Un Ejemplo Concreto

Con `num = "232"` y `target = 8`:

```
Indice 0: pruebo "2" como primer operando
  current_val = 2, last_operand = 2

  Indice 1: pruebo "3"
    +: current_val = 2+3 = 5, last = 3
      Indice 2: pruebo "2"
        +: 5+2 = 7 != 8
        -: 5-2 = 3 != 8
        *: (5-3)+(3*2) = 2+6 = 8 == 8 -> "2+3*2" recolectada!
    -: current_val = 2-3 = -1, last = -3
      Indice 2: pruebo "2"
        +: -1+2 = 1 != 8
        -: -1-2 = -3 != 8
        *: (-1-(-3))+(-3*2) = 2-6 = -4 != 8
    *: current_val = (2-2)+(2*3) = 6, last = 6
      Indice 2: pruebo "2"
        +: 6+2 = 8 == 8 -> "2*3+2" recolectada!
        -: 6-2 = 4 != 8
        *: (6-6)+(6*2) = 12 != 8

  Indice 1: pruebo "32" (concatenacion)
    +: 2+32 = 34 != 8
    -: 2-32 = -30 != 8
    *: (2-2)+(2*32) = 64 != 8

Indice 0: pruebo "23" como primer operando
  current_val = 23, last = 23
  ...ninguna combinacion alcanza 8

Indice 0: pruebo "232" como primer operando
  current_val = 232 != 8
```

Resultado: `["2+3*2", "2*3+2"]`.

## La Proteccion contra Overflow

Un detalle sutil pero critico: aunque el `target` se recibe como `i32`, los valores intermedios pueden exceder ese rango. Considerar `num = "999999999"` -- las concatenaciones generan numeros enormes, y las multiplicaciones los amplifican aun mas. Por eso la implementacion convierte inmediatamente `target` a `i64` y realiza toda la aritmetica en 64 bits, evitando desbordamientos silenciosos que producirian resultados incorrectos.

## Solucion en Rust

```rust
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut result = Vec::new();
        if num.is_empty() {
            return result;
        }

        let chars: Vec<char> = num.chars().collect();
        let target = target as i64;

        Self::backtrack(0, "", 0, 0, &chars, target, &mut result);

        result
    }

    fn backtrack(
        index: usize,
        path: &str,
        current_val: i64,
        last_operand: i64,
        chars: &Vec<char>,
        target: i64,
        result: &mut Vec<String>,
    ) {
        if index == chars.len() {
            if current_val == target {
                result.push(path.to_string());
            }
            return;
        }

        for i in index..chars.len() {
            if i > index && chars[index] == '0' {
                break;
            }

            let part_str: String = chars[index..=i].iter().collect();
            let val: i64 = part_str.parse().unwrap();

            if index == 0 {
                Self::backtrack(i + 1, &part_str, val, val, chars, target, result);
            } else {
                Self::backtrack(
                    i + 1,
                    &format!("{}+{}", path, part_str),
                    current_val + val,
                    val,
                    chars,
                    target,
                    result,
                );

                Self::backtrack(
                    i + 1,
                    &format!("{}-{}", path, part_str),
                    current_val - val,
                    -val,
                    chars,
                    target,
                    result,
                );

                Self::backtrack(
                    i + 1,
                    &format!("{}*{}", path, part_str),
                    (current_val - last_operand) + (last_operand * val),
                    last_operand * val,
                    chars,
                    target,
                    result,
                );
            }
        }
    }
}
```

La implementacion en Rust usa `&str` para el parametro `path` en lugar de `String`, lo que evita transferir ownership en cada llamada recursiva. Cada rama del backtracking construye una nueva cadena con `format!`, que vive en el stack frame correspondiente y se descarta automaticamente al retroceder. El uso de `chars: &Vec<char>` permite acceso indexado O(1) a los caracteres, evitando la complejidad de iterar sobre bytes UTF-8 directamente. La conversion del operando con `part_str.parse().unwrap()` es segura porque la cadena solo contiene digitos -- garantizado por las restricciones del problema. La condicion `i > index && chars[index] == '0'` implementa la poda de ceros a la izquierda con un `break` en lugar de `continue`: una vez que detecto que el primer digito es '0', no tiene sentido probar subcadenas mas largas, porque todas tendrian cero a la izquierda.

## Conclusion

Expression Add Operators es un problema que combina generacion combinatoria con evaluacion aritmetica en tiempo real. La dificultad no esta en la busqueda exhaustiva en si, sino en el manejo elegante de la precedencia de operadores sin construir un arbol de expresiones completo. El truco de rastrear el ultimo operando permite deshacer la operacion anterior cuando aparece una multiplicacion, simulando la precedencia correcta con solo dos variables de estado. La proteccion contra overflow con `i64` y la poda de ceros a la izquierda con `break` son detalles que separan una solucion correcta de una que falla en casos extremos.
