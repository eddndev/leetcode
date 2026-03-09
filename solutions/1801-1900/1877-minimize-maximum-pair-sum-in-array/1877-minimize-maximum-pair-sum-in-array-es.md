---
title: "1877 Minimize Maximum Pair Sum in Array - ES"
problemUrl: "https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["array", "two-pointers", "greedy", "sorting"]
complexity:
  time: "O(n log n)"
  space: "O(1)"
---

# Minimize Maximum Pair Sum in Array: Emparejar Extremos

## El Problema
Dado un arreglo `nums` de longitud par, debemos emparejarlo en `n / 2` pares. Queremos minimizar la **suma maxima** entre todos los pares. La suma de un par es simplemente la suma de sus dos elementos. Devolver el valor minimo posible de la suma maxima de pares.

Un enfoque ingenuo seria explorar todas las formas posibles de emparejar los elementos, pero eso crece factorialmente y es completamente impractico. Hay una observacion greedy que lo resuelve de forma directa.

## La Intuicion: Balancear los Extremos

La idea clave es que si queremos minimizar la suma maxima, necesitamos evitar que dos numeros grandes queden juntos en el mismo par. La mejor forma de lograrlo es emparejar el numero mas grande con el mas pequeno, el segundo mas grande con el segundo mas pequeno, y asi sucesivamente.

Pensemos en por que funciona. Si ordenamos el arreglo, el elemento mas grande `nums[n-1]` tiene que estar en algun par. Si lo emparejamos con cualquier elemento que no sea el mas pequeno, estamos desperdiciando al elemento pequeno: lo vamos a emparejar con otro elemento que podria haber absorbido parte del peso del mas grande. Al emparejar `nums[0]` con `nums[n-1]`, estamos usando el elemento mas pequeno para compensar al mas grande, distribuyendo la carga de la forma mas uniforme posible.

Este razonamiento se aplica recursivamente: una vez que `nums[0]` y `nums[n-1]` estan emparejados, el mismo argumento aplica para `nums[1]` y `nums[n-2]`, y asi sucesivamente. El resultado es que la suma maxima de pares se minimiza cuando emparejamos simetricamente desde los extremos hacia el centro.

## El Algoritmo
1. **Ordenar** el arreglo de menor a mayor.
2. **Usar dos punteros**: uno al inicio (`i`) y otro al final (`j`) del arreglo.
3. **En cada paso**, calcular la suma del par `nums[i] + nums[j]` y actualizar el maximo si esta suma es mayor que el maximo actual.
4. **Avanzar** `i` hacia adelante y `j` hacia atras hasta que se crucen.

## Solucion en C

La implementacion incluye un quicksort manual ya que la biblioteca estandar de C no ofrece una funcion de ordenamiento con interfaz tan directa para arreglos de enteros. Una vez ordenado el arreglo, el ciclo con dos punteros recorre los extremos y rastrea la suma maxima de pares.

```c
#include <stdio.h>

void swap(int* a, int* b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

int partition(int arr[], int low, int high) {
    int pivot = arr[high];
    int i = low - 1;

    for (int j = low; j <= high - 1; j++) {
        if (arr[j] <= pivot) {
            i++;
            swap(&arr[i], &arr[j]);
        }
    }
    swap(&arr[i+1], &arr[high]);
    return i + 1;
}

void quicksort(int arr[], int low, int high) {
    if (low < high) {
        int pi = partition(arr, low, high);
        quicksort(arr, low, pi - 1);
        quicksort(arr, pi + 1, high);
    }
}

int minPairSum(int* nums, int numsSize){
    quicksort(nums, 0, numsSize - 1);

    int max_sum = 0;

    int i = 0;
    int j = numsSize - 1;

    while(i < j) {
        int current_sum = nums[i] + nums[j];

        if (current_sum > max_sum) max_sum = current_sum;

        i++;
        j--;
    }

    return max_sum;
}
```

## Conclusion

La complejidad temporal es $O(n \log n)$ dominada por el ordenamiento, ya que el recorrido con dos punteros es lineal. El espacio es $O(1)$ si no contamos la pila de recursion del quicksort (o $O(\log n)$ si la contamos). Lo satisfactorio de este problema es como la intuicion greedy -- emparejar el mas grande con el mas pequeno -- se justifica de forma tan natural: cualquier otro emparejamiento solo puede empeorar la suma maxima, nunca mejorarla. Es un ejemplo clasico de como ordenar transforma un problema de optimizacion combinatoria en un simple recorrido lineal.
