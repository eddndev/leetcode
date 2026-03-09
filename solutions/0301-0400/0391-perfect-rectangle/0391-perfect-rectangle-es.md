---
title: "0391 Perfect Rectangle - ES"
problemUrl: "https://leetcode.com/problems/perfect-rectangle/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["geometry", "hash-set", "math", "sweep-line"]
complexity:
  time: "O(N) donde N es el numero de rectangulos"
  space: "O(N)"
---

# El Rompecabezas de Piezas Alineadas

## El Problema
Dado un arreglo `rectangles` donde `rectangles[i] = [xi, yi, ai, bi]` representa un rectangulo alineado con los ejes con esquina inferior izquierda `(xi, yi)` y esquina superior derecha `(ai, bi)`, devolver `true` si todos los rectangulos juntos forman una cobertura exacta de una region rectangular sin solapamientos ni huecos.

## La Intuicion Inicial

A primera vista, esto parece requerir algoritmos de linea de barrido o rastreo complejo de intervalos para detectar solapamientos y huecos entre miles de rectangulos. Pero hay una observacion elegante que reduce todo el problema a dos verificaciones simples: una sobre areas y otra sobre esquinas.

Si los rectangulos forman una cobertura perfecta, dos cosas deben ser ciertas simultaneamente. Primero, el area total de todos los rectangulos individuales debe ser igual al area del rectangulo envolvente -- el rectangulo mas pequeno que los contiene a todos. Segundo, y de manera mas sutil, las esquinas deben comportarse de una forma muy especifica.

## La Observacion de Paridad de Esquinas

Consideremos que sucede con las esquinas cuando los rectangulos cubren un plano perfectamente. Las esquinas interiores -- aquellas compartidas por multiples rectangulos -- siempre aparecen un numero par de veces (dos rectangulos comparten un borde, o cuatro se encuentran en un punto). Estas se cancelan. Las unicas esquinas que aparecen un numero impar de veces son las cuatro esquinas del rectangulo envolvente general, porque cada una de ellas pertenece a exactamente un rectangulo pequeno.

Esto me da un invariante poderoso: si rastro cada esquina de cada rectangulo usando un conjunto, alternando la pertenencia (insertando si esta ausente, eliminando si esta presente), entonces despues de procesar todos los rectangulos, el conjunto deberia contener exactamente cuatro puntos -- y esos cuatro puntos deben ser las esquinas del rectangulo envolvente.

## La Verificacion de Area Previene Falsos Positivos

La verificacion de esquinas por si sola no es suficiente. Consideremos dos rectangulos identicos apilados uno sobre otro -- sus esquinas se cancelarian perfectamente, pero se solapan. La verificacion de area captura esto: la suma de las areas individuales seria el doble del area del rectangulo envolvente. De forma inversa, la verificacion de area sola no previene configuraciones donde los rectangulos caben dentro del rectangulo envolvente sin cubrirlo completamente. Juntas, las dos verificaciones forman una condicion necesaria y suficiente para una cobertura rectangular perfecta.

## Uniendo Todo

Mi algoritmo hace una sola pasada por todos los rectangulos. Para cada uno, actualizo el rectangulo envolvente global rastreando `min_x`, `min_y`, `max_x` y `max_y`. Acumulo el area total usando `i64` para evitar desbordamiento. Alterno cada una de las cuatro esquinas en un `HashSet` -- si la esquina ya esta presente, la elimino; de lo contrario, la inserto.

Despues de la pasada, verifico tres cosas: el conjunto contiene exactamente cuatro esquinas, esas esquinas coinciden con el rectangulo envolvente, y el area acumulada es igual al area del rectangulo envolvente.

## Solucion en Rust

```rust
use std::collections::HashSet;
use std::i32;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        if rectangles.is_empty() {
            return false;
        }

        let mut corners = HashSet::new();
        let mut area_sum: i64 = 0;

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for rect in &rectangles {
            let x1 = rect[0];
            let y1 = rect[1];
            let x2 = rect[2];
            let y2 = rect[3];

            min_x = min_x.min(x1);
            min_y = min_y.min(y1);
            max_x = max_x.max(x2);
            max_y = max_y.max(y2);

            area_sum += (x2 - x1) as i64 * (y2 - y1) as i64;

            let points = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)];

            for p in points.iter() {
                if corners.contains(p) {
                    corners.remove(p);
                } else {
                    corners.insert(*p);
                }
            }
        }

        if corners.len() != 4 {
            return false;
        }

        let expected_corners = [
            (min_x, min_y),
            (min_x, max_y),
            (max_x, min_y),
            (max_x, max_y),
        ];

        for p in expected_corners.iter() {
            if !corners.contains(p) {
                return false;
            }
        }

        let expected_area = (max_x - min_x) as i64 * (max_y - min_y) as i64;

        area_sum == expected_area
    }
}
```

La implementacion es sorprendentemente directa para un problema Hard. El `HashSet` de tuplas `(i32, i32)` sirve como mecanismo de alternancia -- cada esquina se agrega o se elimina, computando efectivamente la paridad. El calculo del area usa `i64` para manejar casos donde las coordenadas alcanzan hasta 100,000 y el producto de dos diferencias podria desbordar `i32`. Despues de la unica pasada, las tres verificaciones de validacion se ejecutan en tiempo constante ya que solo estoy comparando contra cuatro esquinas esperadas y un area esperada.

## Conclusion

Perfect Rectangle es uno de esos problemas donde la dificultad no esta en el codigo sino en la intuicion. El enfoque de fuerza bruta de verificar cada par de rectangulos por solapamiento seria cuadratico y doloroso. El enfoque de linea de barrido funciona pero requiere manejo cuidadoso de arboles de intervalos. El enfoque de paridad de esquinas, combinado con verificacion de area, resuelve el problema en tiempo O(N) con espacio O(N), y el codigo resultante apenas tiene veinte lineas de logica. La realizacion clave -- que una cobertura perfecta produce exactamente cuatro esquinas sobrevivientes despues de la cancelacion por paridad, y que estas deben ser las esquinas del rectangulo envolvente -- transforma una pesadilla geometrica en un ejercicio de manipulacion de conjuntos. Es un recordatorio de que los algoritmos mas poderosos a veces no provienen de estructuras de datos sofisticadas, sino de una observacion geometrica profunda.
