---
title: "0002 Add Two Numbers - ES"
problemUrl: "https://leetcode.com/problems/add-two-numbers/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["linked-list", "math", "recursion"]
complexity:
  time: "O(max(n, m))"
  space: "O(max(n, m))"
---

# Add Two Numbers: Aritmética Elemental con Listas Enlazadas

## El Problema
Se nos dan dos listas enlazadas no vacías que representan dos números enteros no negativos. Los dígitos están almacenados en **orden inverso**, y cada nodo contiene un solo dígito. Debemos sumar los dos números y devolver el resultado como una lista enlazada.

Por ejemplo, si tenemos `l1 = [2, 4, 3]` y `l2 = [5, 6, 4]`, representan los números 342 y 465 respectivamente. La suma es 807, así que el resultado sería `[7, 0, 8]`.

Parece sencillo, pero hay una sutileza que lo hace interesante: el **acarreo** (carry).

## La Intuición: Sumar Como en la Escuela
Cuando vi este problema, lo primero que me vino a la mente fue la suma en columna que aprendimos de niños. Escribes los dos números uno debajo del otro, sumas dígito a dígito de derecha a izquierda, y si la suma supera 9, "te llevas uno".

Y resulta que el problema ya nos da los dígitos en el orden perfecto para esto: **de derecha a izquierda** (el dígito menos significativo primero). No necesitamos invertir nada. Solo recorrer ambas listas simultáneamente, sumar los valores correspondientes junto con el acarreo, y construir la lista resultado nodo a nodo.

## La Clave: No Olvidar el Acarreo Final
El error más común en este problema es olvidar que al terminar de recorrer ambas listas, puede quedar un acarreo pendiente. Si sumamos `[9, 9]` y `[1]`, obtenemos `[0, 0, 1]` (99 + 1 = 100). Si no verificamos el carry al final, perderíamos ese último dígito.

La condición del bucle debe cubrir tres casos: que queden nodos en `l1`, que queden nodos en `l2`, o que quede un carry distinto de cero. Mientras cualquiera de estas condiciones sea verdadera, seguimos iterando.

## El Algoritmo
1. Creamos un nodo dummy como cabecera de la lista resultado.
2. Inicializamos el acarreo (`carry`) en 0.
3. Recorremos ambas listas mientras alguna tenga nodos o el carry sea distinto de cero:
   - Sumamos los valores de los nodos actuales (si existen) más el carry.
   - El nuevo dígito es `sum % 10`.
   - El nuevo carry es `sum / 10`.
   - Creamos un nuevo nodo con el dígito y lo enlazamos.
4. Retornamos `dummy.next`.

### Implementación en C
En C, usamos un nodo dummy en el stack para evitar tratar el primer nodo como caso especial. Cada nuevo dígito se crea con `malloc`.

```c
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
#include <stdlib.h>

struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2) {
    struct ListNode dummy;
    dummy.val = 0;
    dummy.next = NULL;

    struct ListNode* current = &dummy;
    int carry = 0;

    while (l1 != NULL || l2 != NULL || carry) {
        int sum = carry;

        if (l1 != NULL) {
            sum += l1->val;
            l1 = l1->next;
        }

        if (l2 != NULL) {
            sum += l2->val;
            l2 = l2->next;
        }

        carry = sum / 10;

        struct ListNode* newNode = (struct ListNode*)malloc(sizeof(struct ListNode));
        newNode->val = sum % 10;
        newNode->next = NULL;

        current->next = newNode;
        current = newNode;
    }

    return dummy.next;
}
```

### Implementación en Rust
En Rust, el manejo de listas enlazadas con `Option<Box<ListNode>>` es más verboso, pero el patrón es el mismo. Usamos referencias inmutables para recorrer las listas de entrada y construimos la lista resultado mutando el puntero `current`.

```rust
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;
        let mut carry = 0;
        let mut p1 = &l1;
        let mut p2 = &l2;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(node) = p1 {
                sum += node.val;
                p1 = &node.next;
            }

            if let Some(node) = p2 {
                sum += node.val;
                p2 = &node.next;
            }

            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
        }

        dummy.next
    }
}
```

## Conclusión
Este problema es un excelente ejercicio para practicar el recorrido simultáneo de listas enlazadas y el manejo de acarreo. La clave no está en una estructura de datos sofisticada ni en un truco algorítmico oculto, sino en la atención al detalle: manejar correctamente las listas de diferente longitud y no olvidar el carry final. Es uno de esos problemas donde la elegancia reside en la simplicidad.
