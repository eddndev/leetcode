---
title: "0060 Permutation Sequence - ES"
problemUrl: "https://leetcode.com/problems/permutation-sequence/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["math", "recursion"]
complexity:
  time: "O(N^2)"
  space: "O(N)"
---

# Saltando Millones de Permutaciones con Factoriales

## El Problema
Dados dos enteros `n` y `k`, devolver la k-esima secuencia de permutacion de los numeros `[1, 2, ..., n]`. El conjunto de permutaciones esta ordenado lexicograficamente, y `k` empieza en 1.

## La Intuicion Inicial

El enfoque de fuerza bruta seria generar todas las `n!` permutaciones en orden y tomar la k-esima. Para `n = 9`, eso son 362,880 permutaciones. Funciona, pero es profundamente ineficiente: estamos construyendo cientos de miles de secuencias solo para descartar todas menos una. Tiene que haber una forma de saltar directamente a la respuesta.

Y la hay. La observacion clave es que las permutaciones tienen una estructura muy predecible cuando se ordenan lexicograficamente. Si tengo `n` numeros, las primeras `(n-1)!` permutaciones empiezan con el numero mas pequeno, las siguientes `(n-1)!` con el segundo mas pequeno, y asi sucesivamente. Esto significa que puedo determinar cada digito del resultado dividiendo y acotando, sin generar jamas una sola permutacion que no necesito.

## El Enfoque del Sistema Factorial

Pensemos asi: dado `n = 4` y `k = 9`, los digitos disponibles son `[1, 2, 3, 4]` y hay `4! = 24` permutaciones en total. El primer digito particiona esas 24 permutaciones en 4 grupos de `3! = 6` cada uno:

- Las permutaciones 1-6 empiezan con `1`
- Las permutaciones 7-12 empiezan con `2`
- Las permutaciones 13-18 empiezan con `3`
- Las permutaciones 19-24 empiezan con `4`

Como `k = 9` cae en el segundo grupo, el primer digito es `2`. Ahora hemos consumido 6 permutaciones (todo el primer grupo), asi que necesitamos la `(9 - 6) = 3`ra permutacion de los digitos restantes `[1, 3, 4]`.

Repetimos el proceso. Entre `[1, 3, 4]`, las `2! = 2` permutaciones por digito inicial nos dan:

- Las permutaciones 1-2 empiezan con `1`
- Las permutaciones 3-4 empiezan con `3`
- Las permutaciones 5-6 empiezan con `4`

La tercera cae en el segundo grupo, asi que el siguiente digito es `3`. Ahora necesitamos la primera permutacion de `[1, 4]`, que es simplemente `1, 4`.

Resultado: `"2314"`.

La implementacion hace esto aun mas limpio convirtiendo `k` a base 0 al inicio. Con `k = k - 1`, el indice en los digitos disponibles en cada paso es simplemente `k / factorial`, y el residuo `k % factorial` se convierte en el nuevo `k` para el siguiente paso. Sin gimnasia de off-by-one.

### Por que O(N^2)?

Cada paso implica eliminar un elemento de un vector de digitos restantes. Esa eliminacion es O(N) porque los elementos posteriores al eliminado deben desplazarse. Hacemos esto N veces, dando O(N^2) en total. Para la restriccion `n <= 9`, esto es completamente insignificante, pero vale la pena mencionarlo. Si `n` fuera grande, un arbol binario balanceado o un arbol de Fenwick podrian reducirlo a O(N log N).

## Solucion en Rust

```rust
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n_usize = n as usize;
        let mut fact = vec![1; n_usize];
        for i in 1..n_usize {
            fact[i] = fact[i - 1] * i as i32;
        }

        let mut numbers: Vec<char> = (1..=n as u8).map(|digit| (b'0' + digit) as char).collect();

        let mut k = k - 1;
        let mut result = String::with_capacity(n_usize);

        for i in (0..n_usize).rev() {
            let factorial = fact[i];

            let index = (k / factorial) as usize;

            result.push(numbers.remove(index));

            k %= factorial;
        }

        result
    }
}
```

La implementacion en Rust es compacta y expresiva. La tabla de factoriales `fact` se construye de abajo hacia arriba, con `fact[i]` almacenando `i!`. El vector `numbers` empieza como `['1', '2', ..., 'n']` y se reduce en un elemento por iteracion a medida que los digitos se seleccionan y eliminan. El `k` indexado en 0 dirige todo el proceso de seleccion: `k / fact[i]` nos dice que digito elegir, y `k %= fact[i]` acota la busqueda a las posiciones restantes. El uso de `String::with_capacity` evita realocaciones, y `Vec::remove` se encarga tanto de la extraccion como del desplazamiento en una sola llamada.

## Conclusion

Este problema es un ejemplo hermoso de como entender la estructura matematica detras de un objeto combinatorio puede eliminar clases enteras de computacion. En lugar de generar permutaciones, descomponemos `k` en el sistema numerico factorial, esencialmente leyendo la respuesta digito a digito. El enfoque es determinista, ligero en asignaciones de memoria, y corre en tiempo proporcional a `n^2` en el peor caso, aunque para las restricciones dadas es efectivamente instantaneo. A veces el mejor algoritmo no viene de estructuras de datos ingeniosas ni de recursion intrincada; viene de darse cuenta de que la respuesta siempre estuvo codificada en la aritmetica.
