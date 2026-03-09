---
title: "1382 Balance a Binary Search Tree - ES"
problemUrl: "https://leetcode.com/problems/balance-a-binary-search-tree/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["binary-search-tree", "divide-and-conquer", "tree", "greedy", "depth-first-search"]
complexity:
  time: "O(n)"
  space: "O(n)"
---

# Balance a Binary Search Tree: Aplanar y Reconstruir

## El Problema
Dado el nodo raiz de un arbol binario de busqueda (BST), necesitamos devolver un arbol binario de busqueda **balanceado** con los mismos valores de nodo. Un arbol binario de busqueda esta balanceado si la profundidad de los dos subarboles de cada nodo nunca difiere en mas de 1. Si hay mas de una respuesta, cualquier respuesta valida es aceptada.

A primera vista, uno podria pensar en rotaciones como las que usan los arboles AVL o rojo-negro. Pero hay un camino mucho mas directo que aprovecha una propiedad fundamental de los BST.

## La Intuicion: El Recorrido Inorden ya Esta Ordenado

La propiedad clave de un BST es que su recorrido inorden (izquierda, raiz, derecha) produce los valores en orden ascendente. Esto significa que si aplanamos el arbol en un arreglo usando un recorrido inorden, obtenemos una secuencia ordenada de todos los valores.

Una vez que tenemos esa secuencia ordenada, el problema se transforma en algo que ya conocemos bien: **construir un BST balanceado a partir de un arreglo ordenado**. Y la forma de hacerlo es exactamente la misma idea detras de la busqueda binaria: tomamos el elemento del medio como raiz, y recursivamente construimos el subarbol izquierdo con la mitad izquierda y el subarbol derecho con la mitad derecha.

Al elegir siempre el punto medio, garantizamos que ambos subarboles tengan aproximadamente la misma cantidad de nodos, lo que produce un arbol con la minima altura posible.

## El Algoritmo
1. **Recorrido inorden**: Recorrer el BST original en orden (izquierda, nodo, derecha) y almacenar todos los valores en un vector.
2. **Construir el arbol balanceado**: Usar el arreglo ordenado para construir un nuevo BST. En cada paso, tomar el elemento del medio como raiz del subarbol actual, y aplicar recursivamente el mismo proceso a las mitades izquierda y derecha.

La recursion termina cuando `start > end`, lo que significa que no hay elementos para ese subarbol y devolvemos `None`.

## La Implementacion en Rust

En Rust, los arboles con propiedad compartida requieren `Rc<RefCell<TreeNode>>`. El recorrido inorden llena un `Vec<i32>` con los valores ordenados. La funcion `build` usa indices `i32` para los limites, lo que permite que `start > end` sirva como caso base de forma natural (cuando `end` es -1 al principio del rango, por ejemplo).

```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vals = Vec::new();
        Self::inorder(&root, &mut vals);
        Self::build(&vals, 0, vals.len() as i32 - 1)
    }

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            Self::inorder(&n.left, vals);
            vals.push(n.val);
            Self::inorder(&n.right, vals);
        }
    }

    fn build(vals: &Vec<i32>, start: i32, end: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if start > end {
            return None;
        }
        let mid = (start + end) / 2;
        let node = Rc::new(RefCell::new(TreeNode::new(vals[mid as usize])));

        node.borrow_mut().left = Self::build(vals, start, mid - 1);
        node.borrow_mut().right = Self::build(vals, mid + 1, end);

        Some(node)
    }
}
```

## Conclusion
La complejidad temporal es O(n) donde n es el numero de nodos: el recorrido inorden visita cada nodo una vez, y la construccion del arbol balanceado tambien procesa cada valor una vez. El espacio es O(n) para almacenar el vector de valores y la pila de recursion. Este problema es un recordatorio de que a veces la mejor forma de arreglar una estructura no es repararla pieza por pieza, sino deconstruirla hasta su esencia (los valores ordenados) y reconstruirla desde cero con la forma correcta.
