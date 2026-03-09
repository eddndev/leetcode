---
title: "0321 Create Maximum Number - ES"
problemUrl: "https://leetcode.com/problems/create-maximum-number/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["greedy", "stack", "monotonic-stack", "merge"]
complexity:
  time: "O(K * (M + N)) donde M y N son las longitudes de los dos arreglos"
  space: "O(M + N)"
---

# Forjando el Numero Mas Grande con Dos Mazos

## El Problema
Dados dos arreglos de enteros `nums1` y `nums2` de longitudes `m` y `n` respectivamente, y un entero `k` donde `k <= m + n`, crear el numero maximo de longitud `k` usando digitos de ambos arreglos. El orden relativo de los digitos del mismo arreglo debe preservarse. Devolver el arreglo de `k` digitos que represente el numero mas grande posible.

## La Intuicion Inicial

Mi primer pensamiento fue programacion dinamica, pero el espacio de estados explota rapidamente cuando necesito rastrear posiciones en ambos arreglos y la cantidad de digitos seleccionados. En cambio, note que el problema se puede descomponer limpiamente en tres subproblemas independientes:

1. **Cuantos digitos tomo de cada arreglo?** Si tomo `i` digitos de `nums1`, tomo `k - i` de `nums2`.
2. **Cuales digitos elijo de cada arreglo?** Para una cantidad fija, quiero la *subsecuencia mas grande posible* de esa longitud.
3. **Como combino dos subsecuencias en la secuencia fusionada mas grande?** Esto es un merge greedy, no un simple entrelazado.

La belleza es que cada uno de estos subproblemas tiene una solucion limpia y eficiente, y componerlos da la respuesta global.

## Repartiendo el Presupuesto

Itero sobre todas las particiones validas: tomar `i` digitos de `nums1` y `k - i` de `nums2`, donde `i` va desde `max(0, k - n)` hasta `min(k, m)`. Para cada particion, extraigo la mejor subsecuencia de cada arreglo, las fusiono y me quedo con el mejor candidato global.

## Extrayendo la Subsecuencia Maxima

Dado un arreglo y una longitud objetivo `k`, quiero la subsecuencia lexicograficamente mas grande de longitud `k`. Este es un problema clasico de pila monotonica. Recorro el arreglo de izquierda a derecha, manteniendo una pila. Para cada nuevo elemento, mientras el tope de la pila sea menor que el elemento actual *y* aun tenga suficientes elementos restantes para llenar la pila hasta longitud `k`, saco el tope. Luego empujo el elemento actual (si la pila aun no esta llena).

La variable `drop` cuenta cuantos elementos me esta permitido descartar. Inicialmente es `n - k` (el total de elementos que debo saltar). Cada vez que saco de la pila, efectivamente descarte un elemento, asi que decremento `drop`. Al final, trunco la pila a exactamente `k` elementos para manejar el caso donde nunca saque suficientes.

### Un Ejemplo Rapido

Para `nums = [9, 1, 2, 5, 8, 3]` y `k = 3`:

- `drop = 3`. Pila: `[]`
- `9`: empujar. Pila: `[9]`
- `1`: `1 < 9`, solo empujar. Pila: `[9, 1]`
- `2`: `2 > 1`, sacar `1` (drop=2), `2 < 9`, empujar. Pila: `[9, 2]`
- `5`: `5 > 2`, sacar `2` (drop=1), `5 < 9`, empujar. Pila: `[9, 5]`
- `8`: `8 > 5`, sacar `5` (drop=0), `8 < 9`, empujar. Pila: `[9, 8]`
- `3`: drop=0 asi que no saco nada, empujar. Pila: `[9, 8, 3]`

Resultado: `[9, 8, 3]` -- la subsecuencia de 3 digitos mas grande.

## El Merge: Greedy Lexicografico

Fusionar dos subsecuencias en la secuencia mas grande posible es mas complicado de lo que parece. En cada paso, comparo las porciones *restantes* de ambas subsecuencias lexicograficamente, no solo sus elementos frontales. Si `s1[i..] > s2[j..]`, tomo de `s1`; de lo contrario tomo de `s2`.

Por que no puedo simplemente comparar los elementos frontales? Consideremos fusionar `[6, 7]` y `[6, 0, 4]`. Ambas empiezan con `6`, pero tomar de `[6, 7]` primero da `[6, 6, 7, 0, 4]` mientras que tomar de `[6, 0, 4]` primero da `[6, 6, 0, 7, 4]`. La primera es claramente mejor. Comparar los sufijos completos `[6, 7] > [6, 0, 4]` me dice correctamente que debo tomar del primer arreglo.

En Rust, esta comparacion es hermosamente simple: la comparacion de slices (`s1[i..] > s2[j..]`) realiza comparacion lexicografica de forma nativa.

## Solucion en Rust

```rust
impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let k = k as usize;
        let mut best_result = vec![0; k];

        let start = if k > n2 { k - n2 } else { 0 };
        let end = if k < n1 { k } else { n1 };

        for i in start..=end {
            let len1 = i;
            let len2 = k - i;

            let sub1 = Self::get_max_subsequence(&nums1, len1);
            let sub2 = Self::get_max_subsequence(&nums2, len2);

            let candidate = Self::merge(&sub1, &sub2);

            if candidate > best_result {
                best_result = candidate;
            }
        }

        best_result
    }

    fn get_max_subsequence(nums: &Vec<i32>, k: usize) -> Vec<i32> {
        let mut stack = Vec::with_capacity(k);
        let n = nums.len();
        let mut drop = n - k;

        for &val in nums {
            while drop > 0 && !stack.is_empty() && val > *stack.last().unwrap() {
                stack.pop();
                drop -= 1;
            }
            stack.push(val);
        }

        stack.truncate(k);
        stack
    }

    fn merge(s1: &Vec<i32>, s2: &Vec<i32>) -> Vec<i32> {
        let len = s1.len() + s2.len();
        let mut res = Vec::with_capacity(len);
        let mut i = 0;
        let mut j = 0;

        while i < s1.len() || j < s2.len() {
            if i < s1.len() && (j == s2.len() || s1[i..] > s2[j..]) {
                res.push(s1[i]);
                i += 1;
            } else {
                res.push(s2[j]);
                j += 1;
            }
        }
        res
    }
}
```

La implementacion se descompone limpiamente en tres funciones que reflejan los tres subproblemas. `max_number` orquesta la enumeracion de particiones, inicializando `best_result` como un vector de ceros (el candidato mas pequenio posible) y actualizandolo cada vez que se encuentra un mejor candidato mediante comparacion directa de vectores. Los limites `start` y `end` aseguran que nunca pida mas digitos de los que un arreglo puede proveer. `get_max_subsequence` implementa el enfoque de pila monotonica con `with_capacity(k)` para una sola alocacion y un `truncate` al final por seguridad. La funcion `merge` aprovecha la comparacion nativa de slices de Rust `s1[i..] > s2[j..]`, que realiza comparacion lexicografica elemento por elemento -- una linea elegante que reemplaza lo que seria un ciclo manual en la mayoria de otros lenguajes.

## Conclusion

Create Maximum Number es un problema que recompensa la descomposicion. Lo que inicialmente parece un problema de optimizacion monolitico se divide en tres piezas bien entendidas: enumerar la particion, extraer subsecuencias maximas con una pila monotonica, y fusionar con comparacion lexicografica. Cada pieza es individualmente sencilla, y su composicion produce la solucion optima. La comparacion nativa de slices de Rust es la cereza del pastel, haciendo que la logica del merge sea correcta y concisa sin necesidad de un comparador personalizado.
