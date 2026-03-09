---
title: "0239 Sliding Window Maximum - ES"
problemUrl: "https://leetcode.com/problems/sliding-window-maximum/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["deque", "sliding-window", "monotonic-queue"]
complexity:
  time: "O(N) donde N es la longitud del arreglo"
  space: "O(K) donde K es el tamano de la ventana"
---

# El Centinela al Frente de la Fila

## El Problema
Dado un arreglo de enteros `nums` y una ventana deslizante de tamano `k` que se mueve desde el extremo izquierdo del arreglo hasta el extremo derecho, devolver un arreglo que contenga el valor maximo en cada posicion de la ventana. Solo se pueden ver los `k` numeros dentro de la ventana, y esta se mueve una posicion a la derecha en cada paso.

## La Trampa Ingenua

El enfoque obvio -- recorrer los `k` elementos en cada posicion de la ventana para encontrar el maximo -- funciona pero tiene un tiempo de ejecucion O(N*K). Para arreglos grandes con ventanas grandes, esto es demasiado lento. Cada vez que la ventana se desplaza una posicion, descartamos la mayor parte del trabajo previo. Ya conociamos el maximo entre `k-1` de esos elementos; seguramente podemos reutilizar ese conocimiento de alguna manera.

Pero hay una complicacion sutil. Si el maximo era el elemento que acaba de salir de la ventana, no tenemos una forma economica de saber cual es el *siguiente maximo*. Una variable simple de rastreo de maximo no puede manejar la eviccion. Necesitamos una estructura de datos que mantenga el orden, soporte eliminacion eficiente desde ambos extremos y siempre nos de el maximo actual en O(1).

## La Estrategia: Una Deque Monotonica

### La Observacion Clave

Me di cuenta de que no es necesario recordar cada elemento en la ventana. Si estoy mirando el elemento `nums[i]` y hay un elemento anterior `nums[j]` (donde `j < i`) que es *menor* que `nums[i]`, entonces `nums[j]` nunca podra ser el maximo de ninguna ventana futura. Por que? Porque `nums[i]` entro a la ventana despues y es mayor -- sobrevivira a `nums[j]` en la ventana y siempre lo superara. Asi que `nums[j]` es inutil y puede descartarse.

Esta observacion lleva a una **deque monotonicamente decreciente**: una cola de doble extremo donde los elementos siempre estan en orden decreciente del frente hacia atras. El frente de la deque es siempre el maximo de la ventana actual.

### Construyendo la Deque

Para cada nuevo elemento `nums[i]`, realizo tres operaciones:

1. **Expulsar al expirado**: si el elemento al frente de la deque tiene un indice que esta fuera de la ventana actual (es decir, su indice es `<= i - k`), lo saco del frente. Ha caducado.

2. **Imponer monotonicidad**: saco elementos del final de la deque mientras sean menores o iguales a `nums[i]`. Estos elementos ahora estan dominados por el recien llegado y nunca seran un maximo de ventana.

3. **Insertar al recien llegado**: empujo `i` al final de la deque.

Despues de procesar los primeros `k-1` elementos (es decir, cuando `i >= k - 1`), el frente de la deque contiene el indice del maximo de la ventana actual, que registro en el resultado.

### Un Ejemplo Concreto

Con `nums = [1, 3, -1, -3, 5, 3, 6, 7]` y `k = 3`:

```
i=0: nums[0]=1.  Deque: [0].          Ventana aun no llena.
i=1: nums[1]=3.  Sacar 0 (1<=3). Deque: [1].  Ventana aun no llena.
i=2: nums[2]=-1. Deque: [1, 2].       Ventana [1,3,-1] -> max=nums[1]=3
i=3: nums[3]=-3. Deque: [1, 2, 3].    Ventana [3,-1,-3] -> max=nums[1]=3
i=4: nums[4]=5.  Sacar 3,2,1. Deque: [4]. Ventana [-1,-3,5] -> max=nums[4]=5
i=5: nums[5]=3.  Deque: [4, 5].       Ventana [-3,5,3] -> max=nums[4]=5
i=6: nums[6]=6.  Sacar 5. Deque: [4, 6]. Frente 4 expirado (4<=6-3). Sacar 4. Deque: [6].
     Ventana [5,3,6] -> max=nums[6]=6
i=7: nums[7]=7.  Sacar 6. Deque: [7]. Ventana [3,6,7] -> max=nums[7]=7

Resultado: [3, 3, 5, 5, 6, 7]
```

### Por Que Cada Elemento Se Toca Como Maximo Dos Veces

Cada indice entra a la deque exactamente una vez (empujado al final) y sale de la deque como maximo una vez (sacado de cualquier extremo). Asi que a lo largo de las `N` iteraciones, el numero total de operaciones en la deque es como maximo `2N`. Esto hace que el algoritmo sea O(N) en general, independientemente del tamano de la ventana `k`.

## Solucion en Rust

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut deque: VecDeque<usize> = VecDeque::with_capacity(k);
        let mut result = Vec::with_capacity(n - k + 1);

        for i in 0..n {
            if let Some(&front) = deque.front() {
                if i >= k && front <= i - k {
                    deque.pop_front();
                }
            }

            while let Some(&back) = deque.back() {
                if nums[back] <= nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back(i);

            if i >= k - 1 {
                result.push(nums[*deque.front().unwrap()]);
            }
        }

        result
    }
}
```

La implementacion almacena indices en lugar de valores en la deque, lo cual sirve un doble proposito: los indices permiten verificar si el elemento del frente ha expirado (quedo fuera de la ventana), y siempre puedo recuperar el valor mediante `nums[index]`. El `VecDeque` se pre-asigna con capacidad `k` ya que la deque nunca contiene mas de `k` elementos. El vector de resultados se pre-asigna con capacidad `n - k + 1` -- el numero exacto de ventanas. La verificacion de expiracion `front <= i - k` solo se activa cuando `i >= k`, previniendo un desbordamiento en la resta sin signo. El bucle de imposicion de monotonicidad usa `<=` en lugar de `<`, lo que significa que los elementos iguales tambien son expulsados; esto es seguro porque el elemento mas nuevo con el mismo valor sobrevivira al mas antiguo y producira el mismo maximo.

## Conclusion

El Sliding Window Maximum es una aplicacion de libro de texto del patron de deque monotonica. Al mantener una secuencia decreciente de candidatos, aseguramos que el maximo siempre este al frente, la expiracion sea una simple comparacion de indices, y cada elemento se procese en tiempo amortizado O(1). El resultado es un algoritmo de tiempo lineal que evita elegantemente comparaciones redundantes, convirtiendo lo que parece un problema O(N*K) en uno O(N) con solo una deque y una unica pasada por el arreglo.
