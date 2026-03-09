---
title: "0440 K-th Smallest in Lexicographical Order - ES"
problemUrl: "https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["trie", "math"]
complexity:
  time: "O(log(n)^2), donde n es el limite superior del rango"
  space: "O(1)"
---

# El Cartografo del Diccionario Infinito

## El Problema
Dados dos enteros `n` y `k`, encontrar el k-esimo numero mas pequeno en orden lexicografico dentro del rango `[1, n]`. Por ejemplo, si `n = 13` y `k = 2`, los numeros en orden lexicografico son `[1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]`, y el segundo es `10`.

## La Trampa de la Fuerza Bruta

Mi primer impulso fue generar todos los numeros del 1 al n, ordenarlos lexicograficamente y devolver el k-esimo. Pero con `n` llegando hasta `10^9`, eso no es viable ni en tiempo ni en memoria. Necesito una forma de navegar el orden lexicografico sin construir la lista completa.

## Pensar en Arboles, No en Listas

La clave esta en visualizar los numeros del 1 al n como un trie (arbol de prefijos) de 10 ramas. La raiz tiene hijos del 1 al 9, cada uno de esos tiene hijos del 0 al 9, y asi sucesivamente. El recorrido en preorden de este trie produce exactamente el orden lexicografico. Entonces mi pregunta se transforma: en lugar de recorrer nodo por nodo, puedo saltar subarboles enteros si se que k no esta dentro de ellos?

## Contando los Nodos de un Subarbol

Para decidir si debo descender al subarbol que comienza con el prefijo `cur` o saltar al siguiente hermano `cur + 1`, necesito saber cuantos numeros hay en el subarbol de `cur`. Esto se calcula nivel por nivel: en el primer nivel esta solo `cur`, en el segundo estan los numeros de `cur * 10` a `cur * 10 + 9`, en el tercero de `cur * 100` a `cur * 100 + 99`, y asi hasta que los numeros excedan `n`. En cada nivel, los numeros validos van desde `first` hasta `min(n + 1, last) - 1`, donde `first` y `last` definen los limites del rango en ese nivel.

## La Estrategia de Navegacion

Empiezo con `cur = 1` y `k = k - 1` (porque ya estoy parado en el primer numero lexicografico). En cada paso:

1. Calculo `steps`, el numero de nodos en el subarbol de `cur`.
2. Si `steps <= k`, significa que el k-esimo numero no esta en este subarbol. Salto al hermano siguiente: `cur += 1` y descuento `steps` de `k`.
3. Si `steps > k`, el numero que busco esta dentro de este subarbol. Desciendo un nivel: `cur *= 10` y descuento 1 de `k` (porque consumo el nodo actual).

Repito hasta que `k` llega a 0, momento en que `cur` es la respuesta.

## Paso a Paso con un Ejemplo

Con `n = 13`, `k = 2`:

- **Inicio**: `cur = 1`, `k = 1` (despues de restar 1).
- **Paso 1**: Calculo los pasos del subarbol de `1`: nivel 1 tiene `1` (el propio 1), nivel 2 tiene `min(14, 20) - 10 = 4` (los numeros 10, 11, 12, 13). Total: `steps = 5`. Como `5 > 1`, desciendo: `cur = 10`, `k = 0`.
- **k = 0**: La respuesta es `10`.

## Solucion en Rust

```rust
impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut cur = 1;
        let mut k = k - 1;

        while k > 0 {
            let mut steps: i64 = 0;
            let mut first = cur as i64;
            let mut last = first + 1;
            let target = n as i64;

            while first <= target {
                steps += std::cmp::min(target + 1, last) - first;
                first *= 10;
                last *= 10;
            }

            if steps <= k as i64 {
                cur += 1;
                k -= steps as i32;
            } else {
                cur *= 10;
                k -= 1;
            }
        }
        cur
    }
}
```

La funcion usa `i64` para los calculos intermedios porque al multiplicar los limites de nivel por 10 repetidamente, los valores pueden desbordar un `i32`. La variable `first` rastrea el inicio del rango en cada nivel del subarbol, y `last` el inicio del rango del hermano siguiente en ese mismo nivel. La condicion `min(target + 1, last) - first` asegura que no contemos numeros mayores que `n`. El ciclo externo avanza horizontalmente (al hermano) o verticalmente (al hijo) segun si el subarbol cabe dentro de los `k` pasos restantes, navegando el trie virtual sin construirlo jamas.

## Conclusion

K-th Smallest in Lexicographical Order es un problema que revela la estructura oculta detras de algo tan cotidiano como el orden del diccionario. Lo que parece un problema de ordenamiento se convierte en una navegacion inteligente sobre un trie implicito, donde contar los nodos de un subarbol reemplaza la necesidad de recorrerlos uno por uno. La belleza de la solucion esta en que logramos O(log(n)^2) en tiempo y O(1) en espacio, saltando subarboles enteros como un explorador que conoce el mapa antes de pisar el terreno.
