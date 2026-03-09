---
title: "0085 Maximal Rectangle - ES"
problemUrl: "https://leetcode.com/problems/maximal-rectangle/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["stack", "array", "dynamic-programming", "matrix", "monotonic-stack"]
complexity:
  time: "O(R * C)"
  space: "O(C)"
---

# El Rectangulo Escondido a Simple Vista

## El Problema
Dada una matriz binaria de `rows x cols` llena de `'0'`s y `'1'`s, encontrar el area del rectangulo mas grande que contenga solo `'1'`s.

## La Clave: Aplanar Dos Dimensiones en Una

La primera vez que vi este problema, intente pensar en terminos puramente bidimensionales: recorrer cada posible par de esquina superior-izquierda e inferior-derecha, verificando si la submatriz era toda de unos. Eso da O(R^2 * C^2) en el mejor caso, y se sentia como fuerza bruta con disfraz.

El momento de revelacion llego cuando deje de ver la matriz como una cuadricula y empece a verla como una **pila de histogramas**. Si fijo una fila como mi "nivel del suelo", la altura de cada columna es la cantidad de `'1'`s consecutivos que suben desde esa fila. Fila por fila, estoy construyendo un nuevo histograma, y la pregunta se convierte en: cual es el rectangulo mas grande en este histograma?

Y ese es un problema que ya se resolver en O(C) con una pila monotona.

## Construyendo los Histogramas Fila por Fila

Consideremos esta matriz:
```
1 0 1 0 0
1 0 1 1 1
1 1 1 1 1
1 0 0 1 0
```

Despues de procesar cada fila, el arreglo de alturas se ve asi:
- Fila 0: `[1, 0, 1, 0, 0]`
- Fila 1: `[2, 0, 2, 1, 1]`
- Fila 2: `[3, 1, 3, 2, 2]`
- Fila 3: `[4, 0, 0, 3, 0]`

La regla es simple: si la celda actual es `'1'`, incrementar la altura de la fila anterior; si es `'0'`, reiniciar a cero. Un `'0'` rompe la continuidad -- ningun rectangulo puede atravesarlo verticalmente.

En la fila 2, las alturas `[3, 1, 3, 2, 2]` codifican un histograma donde el rectangulo mas grande tiene area 6 (un rectangulo de 3 columnas de ancho por 2 de alto, abarcando las columnas 2 a 4). Ese resulta ser la respuesta para toda la matriz.

## La Pila Monotona: Medir Antes de Demoler

Para el histograma de cada fila, recorro de izquierda a derecha manteniendo una pila de indices de columna en orden creciente de altura. Cuando encuentro una columna mas baja que el tope de la pila, la barra del tope ya no puede extenderse hacia la derecha. La extraigo y calculo su rectangulo: la altura es el valor de la barra extraida, y el ancho se extiende desde el nuevo tope de la pila (limite izquierdo) hasta la posicion actual (limite derecho).

Despues de procesar todas las columnas, introduzco una columna virtual de altura 0 para forzar la extraccion de todo lo que quede en la pila. Esto garantiza que ninguna barra se quede sin ser medida.

La belleza esta en que cada indice de columna se agrega y se extrae como maximo una vez por fila, asi que el bucle interno es O(C) amortizado por fila, dando O(R * C) en total.

## Por que el Espacio Es Solo O(C)

Nunca almaceno la matriz completa de alturas. Un solo arreglo de longitud C se actualiza en su lugar conforme avanzo de una fila a la siguiente. La pila tambien contiene como maximo C elementos. Asi que el espacio extra total es O(C), independiente del numero de filas.

## Solucion en Rust

```rust
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut heights = vec![0; cols];
        let mut max_area = 0;

        let mut stack = Vec::with_capacity(cols + 1);

        for row in matrix {
            for (i, &val) in row.iter().enumerate() {
                if val == '1' {
                    heights[i] += 1;
                } else {
                    heights[i] = 0;
                }
            }

            stack.clear();

            for i in 0..=cols {
                let current_h = if i == cols { 0 } else { heights[i] };

                while let Some(&top) = stack.last() {
                    if current_h < heights[top] {
                        stack.pop();
                        let h = heights[top];

                        let w = if let Some(&prev) = stack.last() {
                            i - prev - 1
                        } else {
                            i
                        };

                        max_area = max_area.max(h * w as i32);
                    } else {
                        break;
                    }
                }
                stack.push(i);
            }
        }

        max_area
    }
}
```

La implementacion en Rust es notablemente compacta para todo lo que logra. El bucle externo itera sobre cada fila de la matriz, actualizando el arreglo `heights` en su lugar: un `'1'` extiende la barra hacia arriba, un `'0'` la reinicia al suelo. La pila se asigna una sola vez con `Vec::with_capacity(cols + 1)` y se limpia entre filas en lugar de reasignarse, evitando trabajo innecesario en el heap. El truco del centinela -- iterar sobre `0..=cols` y tratar `i == cols` como una barra virtual de altura 0 -- fuerza elegantemente a la pila a vaciarse sin necesidad de manejar el final de la fila como caso especial. El calculo del ancho con `i - prev - 1` cuando existe un indice previo en la pila, o simplemente `i` cuando la pila esta vacia, captura ambos casos de los limites sin necesidad de valores centinela explicitos en el arreglo de alturas. La conversion `w as i32` es segura porque las restricciones del problema garantizan que las dimensiones caben comodamente en enteros de 32 bits.

## Conclusion

Este problema es una clase magistral de reduccion: un problema 2D aparentemente complejo colapsa en una serie de problemas 1D, cada uno resoluble con una tecnica bien conocida. La interpretacion como histograma transforma la matriz en algo que una pila monotona puede consumir fila por fila, y el resultado es un algoritmo que es tanto optimo en tiempo como minimo en espacio. La leccion va mas alla de este problema especifico -- cada vez que te enfrentes a una cuadricula y te sientas abrumado por la dimensionalidad, preguntate: puedo fijar una dimension y resolver un problema mas simple a lo largo de la otra? La mayoria de las veces la respuesta es si, y la solucion resultante es mucho mas elegante que cualquier intento de luchar con ambas dimensiones a la vez.
