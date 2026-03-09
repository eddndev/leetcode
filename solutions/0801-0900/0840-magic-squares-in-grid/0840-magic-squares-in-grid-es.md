---
title: "0840 Magic Squares In Grid - ES"
problemUrl: "https://leetcode.com/problems/magic-squares-in-grid/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["array", "matrix", "math"]
complexity:
  time: "O(R * C)"
  space: "O(1)"
---

# Magic Squares In Grid: El Centro lo Dice Todo

## El Problema
Dada una cuadricula `grid` de enteros, contar cuantos subcuadrados de 3x3 son cuadrados magicos. Un cuadrado magico de 3x3 es una cuadricula de 3x3 rellena con numeros distintos del 1 al 9 tal que cada fila, columna y ambas diagonales suman lo mismo (15).

El enfoque ingenuo seria verificar cada subcuadricula 3x3 revisando todas las propiedades una por una. Pero hay un atajo elegante que nos ahorra trabajo innecesario.

## La Intuicion: El 5 Siempre Esta en el Centro

La suma magica de una cuadricula 3x3 con los numeros del 1 al 9 es siempre 15 (la suma total 1+2+...+9 = 45 dividida entre 3 filas). Hay una propiedad menos obvia: el centro de cualquier cuadrado magico de 3x3 valido **siempre es 5**. Esto se puede demostrar sumando las dos diagonales y la fila y columna central: el centro aparece en las cuatro sumas, y la unica forma de que todo cuadre es que sea 5.

Esto nos da un filtro rapido. Antes de hacer cualquier verificacion costosa, revisamos si el centro de la subcuadricula es 5. Si no lo es, la descartamos inmediatamente.

Despues del filtro, verificamos tres cosas en orden:
1. **Todos los valores son distintos y estan entre 1 y 9** - usamos un arreglo `seen` de 10 booleanos.
2. **Las tres filas suman 15** - si alguna falla, pasamos a la siguiente subcuadricula.
3. **Las tres columnas y las dos diagonales suman 15** - la verificacion final.

El orden importa. Cada verificacion actua como un cortocircuito: si falla, saltamos al siguiente candidato sin gastar tiempo en las verificaciones restantes.

## Solucion en C

El manejo de la cuadricula en C con dobles punteros refleja directamente la firma de LeetCode. El arreglo `seen` de tamaño fijo (10 elementos) significa que no hay asignacion dinamica. Los `continue` encadenados mantienen el flujo plano en lugar de anidar condicionales profundamente.

```c
#include <stdbool.h>
#include <string.h>

int numMagicSquaresInside(int **grid, int gridSize, int *gridColSize) {
    size_t r = gridSize, c = gridColSize[0];

    if (r < 3 || c < 3) return 0;

    int result = 0;
    for (int i = 0; i <= r - 3; i++) {
        for (int j = 0; j <= c - 3; j++) {
            if (grid[i + 1][j + 1] != 5) {
                continue;
            }

            bool seen[10] = {false};
            bool valid = true;

            for (int k = i; k < i + 3; k++) {
                for (int m = j; m < j + 3; m++) {
                    int val = grid[k][m];
                    if (val < 1 || val > 9 || seen[val]) {
                        valid = false;
                        break;
                    }
                    seen[val] = true;
                }
                if (!valid) break;
            }

            if (!valid) continue;

            if (grid[i][j] + grid[i][j + 1] + grid[i][j + 2] != 15) continue;
            if (grid[i + 1][j] + grid[i + 1][j + 1] + grid[i + 1][j + 2] != 15) continue;
            if (grid[i + 2][j] + grid[i + 2][j + 1] + grid[i + 2][j + 2] != 15) continue;

            if (grid[i][j] + grid[i + 1][j] + grid[i + 2][j] != 15) continue;
            if (grid[i][j + 1] + grid[i + 1][j + 1] + grid[i + 2][j + 1] != 15) continue;
            if (grid[i][j + 2] + grid[i + 1][j + 2] + grid[i + 2][j + 2] != 15) continue;

            if (grid[i][j] + grid[i + 1][j + 1] + grid[i + 2][j + 2] != 15) continue;
            if (grid[i][j + 2] + grid[i + 1][j + 1] + grid[i + 2][j] != 15) continue;

            result++;
        }
    }
    return result;
}
```

## Conclusion

La complejidad temporal es $O(R \times C)$ donde R y C son las dimensiones de la cuadricula: iteramos por cada posicion posible de subcuadricula, y cada verificacion toma tiempo constante (siempre es 3x3). El espacio es $O(1)$ ya que el arreglo `seen` tiene tamaño fijo. El truco del centro igual a 5 es la observacion que transforma una verificacion exhaustiva en algo que descarta la mayoria de candidatos de forma instantanea.
