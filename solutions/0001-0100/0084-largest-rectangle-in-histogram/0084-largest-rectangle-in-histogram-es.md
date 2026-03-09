---
title: "0084 Largest Rectangle in Histogram - ES"
problemUrl: "https://leetcode.com/problems/largest-rectangle-in-histogram/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["stack", "array", "monotonic-stack"]
complexity:
  time: "O(N)"
  space: "O(N)"
---

# El Rascacielos Escondido en el Histograma

## El Problema
Dado un arreglo de enteros `heights` que representa las alturas de barras en un histograma donde cada barra tiene ancho 1, encontrar el area del rectangulo mas grande que se puede formar dentro del histograma.

## La Intuicion Inicial

La primera vez que me enfrente a este problema, pense en lo obvio: para cada barra, expandirme hacia la izquierda y hacia la derecha mientras las barras vecinas sean al menos tan altas como ella, y calcular el area del rectangulo que se forma. Eso funciona, pero en el peor caso cada barra recorre todo el arreglo, dando O(N^2). Para un histograma con 100,000 barras, eso no es suficiente.

Necesitaba una forma de saber, para cada barra, hasta donde puede extenderse sin repetir trabajo. Y ahi es donde entra la **pila monotona**.

## La Pila Monotona: Demoler para Construir

La idea central es mantener una pila que almacene indices de barras en orden creciente de altura. Recorremos el histograma de izquierda a derecha, y cada vez que encontramos una barra mas baja que la del tope de la pila, sabemos que la barra del tope ya no puede extenderse mas hacia la derecha: la barra actual la "corta". Ese es el momento de calcular su area.

Cuando sacamos un indice de la pila, la altura del rectangulo es la altura de esa barra. El ancho se determina asi: el borde derecho es la posicion actual `i` (la barra que provoco la extraccion), y el borde izquierdo es el nuevo tope de la pila (la barra anterior que era mas baja). Si la pila queda vacia, significa que la barra extraida era la mas baja vista hasta ahora, asi que el rectangulo se extiende desde el inicio del histograma.

### Por que Funciona

La pila mantiene un invariante poderoso: cada barra en la pila sabe que todas las barras entre ella y la barra que tiene encima son al menos tan altas como ella. Cuando una barra es expulsada, ya tenemos toda la informacion necesaria para calcular su rectangulo maximo sin haber mirado hacia atras explicitamente. La pila actua como una memoria comprimida de los "limites izquierdos" de cada barra.

### Un Ejemplo Paso a Paso

Para `heights = [2, 1, 5, 6, 2, 3]`:
- `i=0`: Pila vacia, push 0. Pila: `[0]`
- `i=1`: `heights[1]=1 < heights[0]=2`. Pop 0: altura=2, ancho=1 (pila vacia, ancho=i=1), area=2. Push 1. Pila: `[1]`
- `i=2`: `heights[2]=5 >= heights[1]=1`. Push 2. Pila: `[1, 2]`
- `i=3`: `heights[3]=6 >= heights[2]=5`. Push 3. Pila: `[1, 2, 3]`
- `i=4`: `heights[4]=2 < heights[3]=6`. Pop 3: altura=6, ancho=4-2-1=1, area=6. `heights[4]=2 < heights[2]=5`. Pop 2: altura=5, ancho=4-1-1=2, area=10. `heights[4]=2 >= heights[1]=1`. Push 4. Pila: `[1, 4]`
- `i=5`: `heights[5]=3 >= heights[4]=2`. Push 5. Pila: `[1, 4, 5]`
- Fase de limpieza (altura virtual 0): Pop 5: altura=3, ancho=6-4-1=1, area=3. Pop 4: altura=2, ancho=6-1-1=4, area=8. Pop 1: altura=1, ancho=6 (pila vacia), area=6.
- Maximo: **10**

El truco final es crucial: al terminar de recorrer el arreglo, agregamos una barra virtual de altura 0 para forzar la extraccion de todo lo que quede en la pila. Esto garantiza que ninguna barra se quede sin ser evaluada.

## Solucion en Rust

```rust
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut stack: Vec<usize> = Vec::with_capacity(n);
        let mut max_area = 0;

        for i in 0..=n {
            let current_h = if i == n { 0 } else { heights[i] };

            while let Some(&top_index) = stack.last() {
                if current_h < heights[top_index] {
                    stack.pop();
                    let height = heights[top_index];
                    let width = if let Some(&prev_index) = stack.last() {
                        i - prev_index - 1
                    } else {
                        i
                    };

                    max_area = max_area.max(height * width as i32);
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        max_area
    }
}
```

La implementacion en Rust aprovecha la expresividad del pattern matching. El `while let Some(&top_index) = stack.last()` es idiomatico y elegante: inspecciona el tope de la pila sin sacarlo, y solo hace `pop` cuando confirma que la barra actual es mas baja. El rango `0..=n` incluye la iteracion extra con la barra virtual de altura 0, que se maneja limpiamente con el `if i == n { 0 }`. La preallocacion con `Vec::with_capacity(n)` es un detalle fino: en el peor caso (un histograma estrictamente creciente), la pila contendra todos los indices antes de la fase de limpieza. El calculo del ancho con `i - prev_index - 1` cuando hay un elemento previo en la pila, o simplemente `i` cuando no lo hay, captura exactamente los dos casos de la logica sin necesidad de centinelas adicionales.

## Conclusion

Este problema es una joya de las pilas monotonas. La intuicion de "cuando algo ya no puede crecer, es momento de medirlo" aparece en muchos problemas de programacion competitiva, y dominar este patron abre la puerta a una familia entera de problemas similares. Lo que hace especial a esta solucion es que cada barra entra y sale de la pila exactamente una vez, dando un tiempo amortizado de O(N) a pesar de tener un bucle anidado. A veces la estructura de datos correcta no es la que almacena mas informacion, sino la que descarta informacion en el momento justo.
