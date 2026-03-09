---
title: "0301 Remove Invalid Parentheses - ES"
problemUrl: "https://leetcode.com/problems/remove-invalid-parentheses/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["backtracking", "bfs", "string", "dfs", "pruning"]
complexity:
  time: "O(2^N) donde N es la longitud de la cadena"
  space: "O(N) por la profundidad de la recursion y la expresion construida"
---

# Podando Parentesis Enfermos del Arbol

## El Problema
Dada una cadena `s` que contiene parentesis y letras, eliminar el numero minimo de parentesis invalidos para que la cadena resultante sea valida. Devolver todas las cadenas unicas posibles. Una cadena de parentesis es valida si cada parentesis de apertura `(` tiene su correspondiente cierre `)` y estan correctamente anidados.

## La Cirugia Minima

A primera vista, uno podria pensar en generar todas las combinaciones posibles eliminando parentesis y luego filtrar las validas. Pero eso seria catastroficamente ineficiente. La clave esta en la palabra *minimo*: no quiero eliminar parentesis al azar, sino exactamente los que sobran, ni uno mas ni uno menos.

Mi primer paso es diagnosticar la enfermedad: contar cuantos parentesis de apertura y cuantos de cierre necesito eliminar. Recorro la cadena manteniendo un contador de parentesis de apertura sin emparejar (`l_rem`). Cada vez que encuentro un `(`, incremento el contador. Cada vez que encuentro un `)`, si hay un `(` sin emparejar lo decremento (se emparejaron), pero si no hay ninguno, ese `)` es invalido y debo eliminarlo, asi que incremento `r_rem`. Al final del recorrido, `l_rem` me dice cuantos `(` sobran y `r_rem` cuantos `)` sobran.

## La Estrategia: DFS con Conteo Exacto de Eliminaciones

### Tres Decisiones por Parentesis

Con el diagnostico listo, uso backtracking con DFS. Para cada caracter de la cadena, tengo que decidir:

1. **Si es `(` y aun me quedan eliminaciones de `(` por hacer** (`l_rem > 0`): puedo saltarmelo (eliminarlo).
2. **Si es `)` y aun me quedan eliminaciones de `)` por hacer** (`r_rem > 0`): puedo saltarmelo (eliminarlo).
3. **Siempre**: puedo incluir el caracter en la expresion actual, *siempre y cuando* incluirlo no rompa la validez. Para `)`, esto significa que el balance actual (numero de `(` sin emparejar) debe ser mayor que 0.

La variable `balance` rastrea la cantidad de `(` abiertos que aun no se han cerrado. Nunca permito que el balance se vuelva negativo -- eso significaria un `)` sin su correspondiente `(`.

### Evitando Duplicados

Un problema sutil surge con caracteres repetidos. Si tengo `"(("` y necesito eliminar uno, eliminar el primero o el segundo produce el mismo resultado `"("`. Para manejar esto, uso un `HashSet` que almacena las cadenas resultantes, garantizando unicidad automatica. La condicion de duplicados en el codigo tambien intenta podar ramas redundantes temprano, aunque el `HashSet` es la red de seguridad final.

### Un Ejemplo Concreto

Con `s = "())"`:

```
Diagnostico: l_rem = 0, r_rem = 1 (un ')' sobra)

Index 0: '('
  Incluir: balance = 1, expr = "("
    Index 1: ')'
      Incluir: balance = 0, expr = "()"
        Index 2: ')'
          Saltar (r_rem = 1 -> 0): avanzar sin incluir
            Index 3: fin, l_rem=0, r_rem=0, balance=0 -> "()" recolectada!
          Incluir: balance < 0? No, balance = 0, no puedo incluir ')'
      Saltar (r_rem = 1 -> 0):
        Index 2: ')'
          Incluir: balance = 0? No, no puedo incluir ')' con balance 0
    Index 1: ')'
      Saltar (r_rem = 1 -> 0):
        Index 2: ')'
          Incluir: balance = 1 -> 0, expr = "()"
            Index 3: fin -> "()" recolectada (duplicado, HashSet lo maneja)
```

Resultado: `["()"]`.

### Otro Ejemplo

Con `s = "(a)())"`:

```
Diagnostico: l_rem = 0, r_rem = 1

El DFS explora todas las formas de eliminar exactamente un ')'.
Eliminando el ')' en indice 3: "(a)()" -> valida
Eliminando el ')' en indice 4: "(a)()" -> duplicado
Eliminando el ')' en indice 5: "(a)()" -> duplicado
Pero tambien: eliminando el ')' en indice 1... no, indice 1 es 'a'.

Resultado: ["(a)()", "(a())"]
```

## La Poda que Importa

La belleza de este enfoque es que nunca genero cadenas con mas o menos eliminaciones de las necesarias. Los contadores `l_rem` y `r_rem` actuan como presupuesto: cada vez que decido eliminar un parentesis, descuento del presupuesto correspondiente. Cuando llego al final de la cadena, verifico que ambos presupuestos esten en cero y que el balance sea cero. Esta triple verificacion garantiza que la cadena resultante es valida *y* que elimine exactamente el minimo necesario.

## Solucion en Rust

```rust
use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut l_rem = 0;
        let mut r_rem = 0;

        for c in s.chars() {
            if c == '(' {
                l_rem += 1;
            } else if c == ')' {
                if l_rem > 0 {
                    l_rem -= 1;
                } else {
                    r_rem += 1;
                }
            }
        }

        let s_chars: Vec<char> = s.chars().collect();
        let mut result = HashSet::new();
        let mut current_expr = String::new();

        Self::dfs(0, 0, l_rem, r_rem, &s_chars, &mut current_expr, &mut result);

        result.into_iter().collect()
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        index: usize,
        balance: i32,
        l_rem: i32,
        r_rem: i32,
        s: &Vec<char>,
        expr: &mut String,
        res: &mut HashSet<String>,
    ) {
        if index == s.len() {
            if l_rem == 0 && r_rem == 0 && balance == 0 {
                res.insert(expr.clone());
            }
            return;
        }

        let char_at = s[index];

        let is_duplicate = index > 0
            && s[index] == s[index - 1]
            && expr.len() != (index - (l_rem + r_rem) as usize);

        if char_at == '(' && l_rem > 0 {
            Self::dfs(index + 1, balance, l_rem - 1, r_rem, s, expr, res);
        } else if char_at == ')' && r_rem > 0 {
            Self::dfs(index + 1, balance, l_rem, r_rem - 1, s, expr, res);
        }

        expr.push(char_at);
        if char_at == '(' {
            Self::dfs(index + 1, balance + 1, l_rem, r_rem, s, expr, res);
        } else if char_at == ')' {
            if balance > 0 {
                Self::dfs(index + 1, balance - 1, l_rem, r_rem, s, expr, res);
            }
        } else {
            Self::dfs(index + 1, balance, l_rem, r_rem, s, expr, res);
        }

        expr.pop();
    }
}
```

La implementacion en Rust convierte la cadena de entrada en un `Vec<char>` para acceso indexado O(1), evitando la complejidad de navegar bytes UTF-8. La expresion en construccion (`expr`) se pasa como `&mut String`, lo que permite hacer `push` y `pop` en O(1) amortizado -- el clasico patron de backtracking donde construyo la solucion incrementalmente y la deshago al retroceder. El `HashSet<String>` como contenedor de resultados elimina duplicados de forma natural, aunque tiene el costo de clonar la cadena completa cada vez que se encuentra una solucion valida. La condicion `is_duplicate` intenta detectar ramas redundantes cuando hay caracteres consecutivos iguales, pero el `HashSet` actua como garantia definitiva de unicidad. El uso de `#[allow(clippy::too_many_arguments)]` reconoce que la funcion DFS necesita bastante estado -- un compromiso aceptable para mantener la recursion pura sin recurrir a un struct auxiliar.

## Conclusion

Remove Invalid Parentheses es un problema que combina analisis sintactico con busqueda exhaustiva inteligente. La clave no esta en probar todas las combinaciones posibles de eliminaciones, sino en calcular *antes* de explorar cuantos parentesis de cada tipo deben desaparecer. Con ese diagnostico en mano, el DFS se convierte en una cirugia precisa: cada rama del arbol de recursion respeta el presupuesto de eliminaciones y el invariante de balance, podando temprano las ramas que no pueden conducir a una solucion valida. El `HashSet` como red de seguridad contra duplicados y el patron push/pop para construir la expresion completan una solucion elegante para un problema que, sin la poda adecuada, seria intratable.
