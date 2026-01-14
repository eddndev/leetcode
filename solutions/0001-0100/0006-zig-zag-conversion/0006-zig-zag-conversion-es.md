---
title: "0006 Zigzag Conversion - ES"
problemUrl: "https://leetcode.com/problems/zigzag-conversion/"
difficulty: "Medium"
pubDate: "2026-01-14"
tags: ["string", "math", "optimization"]
complexity:
  time: "O(n)"
  space: "O(1)"
---

# Zigzag Conversion: De la Simulación Visual a la Abstracción Matemática

## El Problema
El problema nos pide tomar una cadena (string) y reordenarla como si estuviera escrita en un patrón de zigzag dado un número específico de filas, para luego leerla línea por línea.

Parece un problema trivial de manipulación de caracteres, pero esconde una lección valiosa sobre cómo el enfoque visual puede traicionarnos en términos de rendimiento.

## La Trampa de la Intuición: Listas Enlazadas
Cuando vi este problema por primera vez, mi mente se fue directo a la simulación física. Me imaginé $N$ filas formándose dinámicamente mientras un "repartidor" distribuía las letras de arriba a abajo y luego en diagonal hacia arriba.

¿La estructura de datos "perfecta" para esto? **Listas Enlazadas**.
Visualmente eran intuitivas: podía crear `numRows` cabezas de lista e ir enganchando nodos a medida que recorría el string original. Al final, solo tendría que concatenar las listas.

Mi primera aproximación fue visualmente atractiva, pero **grosera para el compilador y el procesador**:

```c
#include <stdlib.h>

typedef struct Node {
    char value;
    struct Node* next;
} Node; 

Node** create_tableu(int num_rows) {
    Node** tableu = (Node**)malloc(sizeof(Node*) * num_rows);
    for(int i = 0; i < num_rows; i++){
        Node* temp = (Node*) malloc(sizeof(Node));
        temp->value = '\0';
        temp->next = NULL;
        tableu[i] = temp;
    }
    return tableu;
}

Node* add_char(Node* row, char c){
    Node* temp = (Node*)malloc(sizeof(Node));
    temp->value = '\0';
    temp->next = NULL;

    Node* aux = row;
    aux->value = c;
    aux->next = temp;
    return temp;
}

char* convert(char* s, int numRows) {
    if(numRows == 1) return s;

    // SIMULACIÓN LITERAL (Ingeua)
    // Usar listas enlazadas para representar cada fila.
    // PROBLEMA: Un malloc por CADA carácter del input.
    // Esto fragmenta la memoria y el overhead del syscall es masivo.
    
    int currentRow = 0, i = 0, step = 1;
    char currentChar = s[i];
    Node** tableu = create_tableu(numRows);
    Node** tails = (Node**) malloc(sizeof(Node*) * numRows);
    for (int idx = 0; idx < numRows; idx++) {
        tails[idx] = tableu[idx];
    }
    
    while(currentChar != '\0') {
         // ¡COSTOSO! Una asignación dinámica en el hot path.
        tails[currentRow] = add_char(tails[currentRow], currentChar);
        
        // Lógica de "rebote" del zigzag
        if (step > 0) {
            step = (currentRow + step < numRows) ? 1 : -1;
        } else {
            step = (currentRow + step < 0) ? 1 : -1;
        }

        currentRow += step;
        currentChar = s[++i];
    }

    // Reconstrucción del string final
    char* result = (char*)malloc(sizeof(char) * i + 1);
    currentRow = 0; 
    int j = 0;
    for (int k = 0; k < numRows; k++){
        Node* aux = tableu[k];
        while (aux->next && aux->value != '\0') {
            result[j] = aux->value;
            aux = aux->next;
            if (j < i) j++;
        }
    }
    result[i] = '\0';
    
    // NOTA: Faltaría liberar toda la memoria de los nodos aquí,
    // lo que haría el código aún más lento.
    return result;   
}
```

### El Golpe de Realidad

Mi intuición se derrumbó al ver los resultados. Aunque la lógica era correcta, el *runtime* era de decenas de milisegundos.

Había caído en un error de novato: **asumir que usar punteros automáticamente significa velocidad**.
Estaba realizando una llamada a `malloc` por cada carácter del string. La asignación dinámica de memoria (Syscalls) es costosa. Estaba fragmentando la memoria y destruyendo la localidad de caché. Quería llegar a 0ms, pero con ese enfoque era imposible.

## El Cambio de Paradigma: Matemáticas sobre Memoria

Volví a la mesa de dibujo. Decidí dejar de "mover" los datos y empezar a calcular dónde debían estar.

Observando los índices, noté que los saltos seguían un patrón matemático preciso:

1. **Filas Superior e Inferior:** Tienen un salto constante (`jump = 2 * numRows - 2`).
2. **Filas Intermedias:** Tienen el mismo salto principal, pero con un "paso extra" en medio (la letra diagonal).

La fórmula para ese índice intermedio fue lo que más me costó deducir: `i + jump - 2 * r`.

### La Optimización Final: Robando Nanosegundos

Con la lógica matemática logré un tiempo decente (3ms), pero sabía que podía exprimir más. Revisé mi código y noté redundancias: llamadas a `strlen` repetidas o implícitas.

En C, cada ciclo cuenta. Esta versión final calcula la longitud una sola vez, hace una única asignación de memoria exacta y usa aritmética pura para llenar el buffer.

```c
#include <stdlib.h>
#include <string.h>

char* convert(char* s, int numRows) {
    if (numRows == 1) return s;
    
    int len = strlen(s);
    
    // Una sola asignación de memoria exacta. 
    // Máxima eficiencia y localidad de caché.
    char* result = (char*)malloc(len + 1);
    
    int idx_write = 0;
    
    // El "ciclo" del zigzag se repite cada (2 * numRows - 2) caracteres.
    // Ejemplo con numRows=3:
    // P   A   H   N
    // A P L S I I G
    // Y   I   R
    // El salto es 2*3 - 2 = 4. De la 'P' a la 'A' (arriba) hay 4 posiciones.
    int jump = 2 * numRows - 2;

    for (int r = 0; r < numRows; r++) {
        // Iteramos sobre los caracteres que caen "verticalmente" en la fila 'r'
        for (int i = r; i < len; i += jump) {
            result[idx_write++] = s[i];
            
            // Para las filas intermedias (no la primera ni la última),
            // hay un carácter extra en la "diagonal" entre las columnas verticales.
            if (r > 0 && r < numRows - 1) {
                // Matemática pura para encontrar la posición diagonal
                int diagonal_idx = i + jump - 2 * r;
                
                if (diagonal_idx < len) {
                    result[idx_write++] = s[diagonal_idx];
                }
            }
        }
    }

    result[len] = '\0';
    return result;
}
```

## Conclusión

Pasé de simular el movimiento físico (lento y costoso) a modelar el comportamiento matemático (rápido y eficiente).

Este ejercicio reforzó una lección fundamental de la programación de sistemas: **La abstracción matemática casi siempre vence a la simulación literal.** Evitar `malloc` dentro de bucles y trabajar con memoria contigua es la diferencia entre un código que funciona y un código de alto rendimiento.
