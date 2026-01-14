---
title: "3454 Separate Squares II - ES"
problemUrl: "https://leetcode.com/problems/separate-squares-ii/"
difficulty: "Hard"
pubDate: "2026-01-14"
tags: ["segment-tree", "sweep-line", "binary-search", "coordinate-compression"]
complexity:
  time: "O(N log N)"
  space: "O(N)"
---

# Domando la Geometría con Árboles de Segmentos en C

## El Abismo Inicial
Cuando vi este problema, y aunque todavía no lo sabía, sabía que debía utilizar alguna estructura de datos compleja, algo como un mapa hash o árbol binario. Realmente el ver en lo que consistía me abrumó por unos momentos, pero tomé aire, calma, y empecé a pensar. ¿Qué se requiere para resolver esta clase de problemas?

Entonces empecé a verle la forma. Las restricciones eran muy estrictas: el área sombreada solo podía contarse una vez (unión de áreas). Supe que debía tener algo que tuviera en cuenta toda el área horizontal activa para ciertas coordenadas verticales. ¿Pero cómo construía una estructura de datos que fuera capaz de hacer semejante cosa?

Me presioné un poco al ver el bajo porcentaje de aceptación que tenía el problema. Un consejo: no vean lo que los demás han logrado, enfóquense en lo que saben, y si no saben, el primer paso siempre es encontrar las cosas que desconocen. Es demasiado contraintuitivo a mi parecer ese consejo, pero gracias a ello descubrí una estructura bastante interesante: **los Árboles de Segmentos (Segment Trees)**.

Siendo sincero, en mi vida había escuchado de ellos, pero prácticamente son árboles binarios que permiten consultar y actualizar eficientemente información sobre intervalos. Ahí podía almacenar la "cobertura" del eje X. Y desde el inicio supe que no podía crear una matriz que abarcara todos los puntos en X (ya que llegan hasta $10^9$), solamente los que nos importaban.

## La Estrategia: "Sweep Line" + Compresión de Coordenadas
El rompecabezas se completó cuando entendí que no necesitaba procesar el plano 2D estático. Podía usar una **Línea de Barrido (Sweep Line)**: una línea imaginaria que sube desde $Y=0$.

Cada vez que la línea toca el borde inferior de un cuadrado, "activamos" un rango en el Segment Tree. Cuando toca el borde superior, lo "desactivamos". El área es simplemente la integral de: `(Altura actual - Altura anterior) * Ancho cubierto en X`.

### El Reto de C: Hacerlo a Mano
En lenguajes de alto nivel, existen librerías para mapas ordenados. En C, tuve que implementar todo el "tooling" desde cero:
1.  **Coordinate Compression:** Como las coordenadas X eran enormes, las mapeé a índices pequeños ($0, 1, 2...$) usando `qsort`, un algoritmo de `unique` manual y búsqueda binaria.
2.  **Segment Tree Array-Based:** Implementé el árbol en un simple arreglo lineal, manejando la recursividad para calcular la longitud cubierta (`tree_len`) basada en si un nodo estaba totalmente cubierto (`count > 0`) o parcialmente cubierto por sus hijos.

## La Optimización "Old School"
El problema requiere dividir el área total en dos partes iguales. Esto implica un problema de "el huevo y la gallina": necesito el Área Total para saber cuál es la mitad, pero no tengo el Área Total hasta terminar de procesar.

La solución ingenua sería:
1.  Correr el algoritmo para sacar el Total.
2.  Hacer `free` de todo.
3.  Volver a hacer `malloc` e inicializar todo para buscar la mitad.

Pero en C, podemos ser más astutos. En lugar de destruir y crear memoria (costoso), utilicé `memset` para "resetear" el árbol a ceros instantáneamente entre la primera y la segunda pasada.

```c
// ... (Lógica de Segment Tree y Barrido) ...

    // --- SEGUNDO BARRIDO ---
    // En lugar de realocar memoria, limpiamos el bloque existente.
    // Esto ahorra ciclos valiosos de gestión de memoria del sistema.
    memset(tree_count, 0, (4 * num_intervals + 1) * sizeof(int));
    memset(tree_len, 0, (4 * num_intervals + 1) * sizeof(double));

// ...

```

## Solución Final

Esta combinación de **Compresión de Coordenadas + Segment Tree + Sweep Line** resultó en una solución extremadamente eficiente.

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Estructura de Evento: El corazón del Sweep Line.
// Representa el borde inferior (type=1) o superior (type=-1) de un cuadrado.
typedef struct {
    int y;
    int type; // 1 = Entra un cuadrado, -1 = Sale
    int x_start;
    int x_end;
} Event;

// Ordenamos eventos de abajo hacia arriba (menor Y).
// Si hay empates en Y, procesamos primero las entradas (+1) y luego las salidas (-1).
int compareEvent(const void* a, const void* b) {
    Event* eventA = (Event*) a;
    Event* eventB = (Event*) b;

    if (eventA->y != eventB->y) {
        return eventA->y - eventB->y;
    }

    return eventA->type - eventB->type;
}

int compareInts(const void* a, const void* b) {
    return (*(int*)a - *(int*)b);
}

// Búsqueda binaria para encontrar el índice mapeado (discretizado) de una coordenada X original.
int get_x_index(int* x_coords, int n, int target) {
    int left = 0;
    int right = n - 1;
    while (left <= right) {
        int mid = left + (right - left) / 2;
        if (x_coords[mid] == target) return mid;
        if (x_coords[mid] < target) left = mid + 1;
        else right = mid - 1;
    }
    return -1;
}

// Lógica del Segment Tree.
// `node`: índice actual en el array del árbol.
// `start`, `end`: rango de índices comprimidos que cubre este nodo.
// `l`, `r`: rango de la query actual (el cuadrado que entra/sale).
void update(
    int node, int start, int end, int l, int r, int val,
    int* tree_count, double* tree_len, int* x_coords
) {
    if (l >= r || start >= r || end <= l) return;

    // Si el nodo está totalmente contenido en el rango de actualización, suma/resta `val`.
    if (l <= start && end <= r) tree_count[node] += val;
    else {
        // Si es parcial, bajamos a los hijos.
        int mid = start + (end - start) / 2;

        update(node * 2, start, mid, l, r,val, tree_count, tree_len, x_coords);
        update(node * 2 + 1, mid, end, l, r, val, tree_count, tree_len, x_coords);
    }

    // Recálculo de `tree_len[node]`: Longitud activa cubierta por este nodo.
    // CASO 1: Este nodo tiene un count > 0 -> Todo su rango está cubierto.
    if (tree_count[node] > 0) tree_len[node] = (double)(x_coords[end] - x_coords[start]);
    // CASO 2: Es una hoja (y count == 0) -> No cubre nada.
    else if (end - start == 1) tree_len[node] = 0.0;
    // CASO 3: Es nodo interno (y count == 0) -> La suma de lo que cubran sus hijos.
    else tree_len[node] = tree_len[node * 2] + tree_len[node * 2 +1];
}

double separateSquares(int** squares, int squaresSize, int* squaresColSize) {
    Event* events  = (Event*) malloc(sizeof(Event) * 2 * squaresSize);

    // Coordinate Compression: Mapear coordenadas X gigantes a índices [0, 2N]
    int* x_coords = (int*)malloc(sizeof(int) * 2 * squaresSize);

    int e_idx = 0; // Index for events
    int x_idx = 0; // Index for x_coords

    for (int i = 0; i < squaresSize; i++ ) {
        int x = squares[i][0];
        int y = squares[i][1];
        int l = squares[i][2];
        
        // Evento de entrada (borde inferior)
        events[e_idx].y = y;
        events[e_idx].type = 1;
        events[e_idx].x_start = x;
        events[e_idx].x_end = x + l;
        e_idx++;

        // Evento de salida (borde superior)
        events[e_idx].y = y + l;
        events[e_idx].type = -1;
        events[e_idx].x_start = x;
        events[e_idx].x_end = x + l;
        e_idx++;

        x_coords[x_idx++] = x;
        x_coords[x_idx++] = x + l;
    }

    // Paso crítico: Ordenar eventos por Y, coordenadas X por valor.
    qsort(events, 2 * squaresSize, sizeof(Event), compareEvent);
    qsort(x_coords, 2 * squaresSize, sizeof(int), compareInts);

    // Unique: Eliminar duplicados en X para construir los intervalos base.
    int unique_count = 0;
    if (2 * squaresSize > 0) {
        int write_idx = 0;

        for (int read_idx = 1; read_idx < 2 * squaresSize; read_idx++) {
            if (x_coords[read_idx] != x_coords[write_idx]) {
                write_idx++;
                x_coords[write_idx] = x_coords[read_idx]; // Copy
            }
        }
        unique_count = write_idx + 1;
    }

    int num_intervals = unique_count - 1;
    if (num_intervals <= 0) {
        free(events); free(x_coords);
        return 0.0;
    }

    // Segment Tree en array flat (4N es suficiente para el peor caso).
    int* tree_count = (int*)calloc(4 * num_intervals + 1, sizeof(int));
    double* tree_len = (double*)calloc(4 * num_intervals + 1, sizeof(double));

    double total_area = 0.0;

    // --- PRIMER BARRIDO: CALCULAR ÁREA TOTAL ---
    for (int i = 0; i < 2 * squaresSize; i++) {
        if (i > 0) {
            double dy = (double)(events[i].y - events[i-1].y);
            // Área del slice = Altura (dy) * Ancho cubierto (tree_len[1])
            total_area += dy * tree_len[1];
        }

        int idx_start = get_x_index(x_coords, unique_count, events[i].x_start);
        int idx_end = get_x_index(x_coords, unique_count, events[i].x_end);

        update(
            1, 0, num_intervals, idx_start, idx_end, events[i].type,
            tree_count, tree_len, x_coords
        );
    }

    // --- RESETEO ESTRATÉGICO ---
    // Limpiamos el árbol con memset (muy rápido) para reusarlo.
    memset(tree_count, 0, (4 * num_intervals + 1) * sizeof(int));
    memset(tree_len, 0, (4 * num_intervals + 1) * sizeof(double));

    double target = total_area / 2.0;
    double current_area = 0.0;
    double final_y = events[0].y;

    // --- SEGUNDO BARRIDO: ENCONTRAR LA LÍNEA DE CORTE ---
    for (int i = 0; i < 2 * squaresSize; i++) {
        if (i > 0) {
            double dy = (double)(events[i].y - events[i - 1].y);
            double width = tree_len[1];
            double slice_area = dy * width;

            // ¿Nos pasamos de la mitad con este slice?
            if(current_area + slice_area >= target) {
                double missing_area = target - current_area;
                
                // Interpolación lineal dentro del slice actual para encontrar Y exacta.
                if (width > 0) final_y = events[i - 1].y + (missing_area / width);
                else final_y = events[i - 1].y;

                goto cleanup;
            }

            current_area += slice_area;
        }

        int idx_start = get_x_index(x_coords, unique_count, events[i].x_start);
        int idx_end = get_x_index(x_coords, unique_count, events[i].x_end);

        update(
            1, 0, num_intervals, idx_start, idx_end, events[i].type,
            tree_count, tree_len, x_coords
        );
    }

cleanup:
    free(events);
    free(x_coords);
    free(tree_count);
    free(tree_len);
    return final_y;
}
```

## Resultado

Aunque el tiempo final fue de **267ms** (esperable dada la complejidad $O(N \log N)$ y la magnitud del problema), la verdadera victoria no fue el *runtime*.

Lo que más valoro es la conformidad de haber entendido y aplicado una nueva estructura de datos: el **Segment Tree**. Ver cómo una abstracción puede domar un problema geométrico caótico y convertirlo en una serie de pasos lógicos y discretos ha sido una de las mejores lecciones de este desafío.
