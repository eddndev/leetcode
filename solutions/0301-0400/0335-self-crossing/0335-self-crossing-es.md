---
title: "0335 Self Crossing - ES"
problemUrl: "https://leetcode.com/problems/self-crossing/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["geometry", "math", "array"]
complexity:
  time: "O(n) donde n es la longitud del arreglo de distancias"
  space: "O(1)"
---

# La Espiral que Muerde su Propia Cola

## El Problema
Se te da un arreglo de enteros `distance`. Partiendo del origen en un plano 2D, te mueves `distance[0]` metros al norte, luego `distance[1]` metros al oeste, luego `distance[2]` metros al sur, luego `distance[3]` metros al este, y asi sucesivamente, ciclando entre las direcciones. Devolver `true` si el camino se cruza a si mismo en algun punto, y `false` en caso contrario.

## La Intuicion Inicial

Cuando me encontre con este problema por primera vez, imagine trazar una espiral sobre papel cuadriculado. Caminas al norte, giras a la izquierda para ir al oeste, giras otra vez para ir al sur, y asi sucesivamente -- siempre girando a la izquierda. Si la espiral sigue expandiendose hacia afuera, el camino nunca se cruza. Si se encoge consistentemente hacia adentro, el camino tambien se mantiene limpio. El cruce ocurre cuando la espiral transiciona de expandirse a encogerse, o cuando los segmentos tienen justo la longitud correcta para tocar o solapar una arista anterior.

El enfoque de fuerza bruta de rastrear cada punto visitado y verificar intersecciones seria costoso. Pero la naturaleza restringida del movimiento -- siempre girando a la izquierda en un ciclo fijo de cuatro direcciones -- significa que la geometria del camino es altamente estructurada. Solo necesito verificar una ventana pequena de segmentos recientes para detectar un cruce.

## Los Tres Patrones de Cruce

Despues de dibujar algunos ejemplos, me di cuenta de que un cruce en el paso `i` solo puede ocurrir en una de tres configuraciones geometricas distintas. Cada una involucra al segmento actual intersectando uno de los segmentos previos recientes.

**Patron 1 -- La cuarta arista cruza la primera:** El segmento actual `i` cruza al segmento `i-2` cuando el movimiento actual es al menos tan largo como el de dos pasos atras, Y el movimiento de un paso atras es a lo sumo el de tres pasos atras. Este es el caso clasico donde la espiral intenta expandirse despues de encogerse. La cuarta arista alcanza lo suficiente para cruzar de vuelta sobre la primera.

**Patron 2 -- La quinta arista cae exactamente sobre la primera:** El segmento `i` toca al segmento `i-4` cuando el movimiento de un paso atras es exactamente igual al de tres pasos atras (las aristas paralelas tienen la misma longitud), Y el movimiento actual mas el de cuatro pasos atras alcanza la arista de dos pasos atras. Esto crea una situacion donde el camino se pliega y la quinta arista toca exactamente a la primera.

**Patron 3 -- La sexta arista cruza la primera:** Este es el caso mas sutil. El segmento `i` cruza al segmento `i-4` cuando la espiral se encoge parcialmente pero luego un segmento posterior retrocede. Requiere que cinco condiciones se alineen: el movimiento de un paso atras es menor que el de tres pasos atras, el movimiento de un paso atras mas el de cinco pasos atras alcanza al menos el de tres pasos atras, el movimiento de dos pasos atras supera al de cuatro pasos atras, y el movimiento actual mas el de cuatro pasos atras alcanza al menos el de dos pasos atras.

## Por que Solo Estos Tres?

La belleza de este enfoque es su completitud. Como el camino siempre gira a la izquierda y cada segmento corre en una de cuatro direcciones cardinales, cualquier cruce debe involucrar al segmento actual impactando uno de los ultimos segmentos. Los segmentos mas lejanos son geometricamente inalcanzables por el segmento actual -- los giros intermedios garantizan que estan demasiado lejos. Asi que verificar estos tres patrones en cada paso es necesario y suficiente.

## Recorriendo la Logica

Para cada indice `i` empezando desde 3, verifico primero el patron 1 ya que solo necesita cuatro segmentos. Si `i >= 4`, verifico adicionalmente el patron 2. Si `i >= 5`, verifico el patron 3. En el momento en que cualquier patron coincide, retorno `true`. Si recorro todo el arreglo sin coincidencias, el camino nunca se cruza consigo mismo.

Las condiciones usan solamente comparaciones y sumas sobre los valores de distancia -- sin rastreo de coordenadas, sin busquedas en conjuntos, sin calculos de interseccion geometrica. La solucion entera corre en una sola pasada lineal con espacio extra constante.

## Solucion en Rust

```rust
impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        let n = distance.len();

        if n <= 3 {
            return false;
        }

        for i in 3..n {
            if distance[i] >= distance[i - 2] && distance[i - 1] <= distance[i - 3] {
                return true;
            }

            if i >= 4 {
                if distance[i - 1] == distance[i - 3]
                    && distance[i] + distance[i - 4] >= distance[i - 2]
                {
                    return true;
                }
            }

            if i >= 5 {
                if distance[i - 1] <= distance[i - 3]
                    && distance[i - 1] + distance[i - 5] >= distance[i - 3]
                    && distance[i - 2] > distance[i - 4]
                    && distance[i] + distance[i - 4] >= distance[i - 2]
                {
                    return true;
                }
            }
        }

        false
    }
}
```

La implementacion comienza con un retorno temprano para arreglos de tres o menos elementos, ya que cuatro segmentos son el minimo requerido para un cruce. El ciclo principal itera desde el indice 3, verificando cada patron en orden. El patron 1 compara `distance[i]` contra `distance[i-2]` y `distance[i-1]` contra `distance[i-3]` para detectar cuando la inversion de la espiral causa una interseccion directa. El patron 2 entra en accion en el indice 4 y captura el caso de solapamiento exacto donde aristas paralelas tienen longitudes identicas. El patron 3, disponible desde el indice 5 en adelante, maneja el escenario mas complicado donde una contraccion parcial seguida de una expansion parcial causa que la sexta arista recorte a la primera.

La elegancia de la solucion reside en reducir un problema de interseccion geometrica a un punado de comparaciones aritmeticas. Nunca se calculan coordenadas -- las distancias relativas entre aristas paralelas contienen toda la informacion necesaria para detectar cruces.

## Conclusion

Self Crossing es un problema que recompensa el razonamiento geometrico sobre la fuerza bruta. Lo que inicialmente parece requerir rastreo de coordenadas y calculo de intersecciones se reduce a tres patrones simples que involucran solo las ultimas distancias. La estructura restringida de giros a la izquierda de la espiral implica que los cruces solo pueden ocurrir en configuraciones predecibles, y reconocer estos patrones transforma una simulacion potencialmente cuadratica en una solucion `O(n)` de una sola pasada con espacio `O(1)`. La parte mas dificil no es el codigo -- es convencerse de que tres patrones son verdaderamente exhaustivos, lo cual requiere diagramar cuidadosamente como una espiral que gira a la izquierda puede plegarse sobre si misma.
