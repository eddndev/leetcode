---
title: "0124 Binary Tree Maximum Path Sum - ES"
problemUrl: "https://leetcode.com/problems/binary-tree-maximum-path-sum/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["binary-tree", "depth-first-search", "dynamic-programming"]
complexity:
  time: "O(N)"
  space: "O(H)"
---

# El Sendero de Oro Escondido en el Arbol

## El Problema
Dado el nodo `root` de un arbol binario, devolver la suma maxima de camino de cualquier camino **no vacio** en el arbol. Un camino se define como una secuencia de nodos donde cada par de nodos adyacentes tiene una arista que los conecta. Un nodo solo puede aparecer en el camino como maximo una vez. El camino no necesita pasar por la raiz.

## La Trampa Sutil

A primera vista, uno podria pensar que esto se resuelve buscando el camino mas largo o el camino desde la raiz hasta alguna hoja. Pero el problema es mas libre y mas traicionero: el camino puede empezar y terminar en *cualquier* nodo del arbol, y puede ir "hacia arriba" a traves del padre de un nodo. Eso significa que el camino optimo podria ser un subarbol completo donde un nodo actua como "techo" conectando su rama izquierda con su rama derecha.

Mi primer instinto fue pensar en recorrer todos los caminos posibles, pero eso es exponencial. La observacion clave es que en un arbol binario, cualquier camino tiene exactamente un nodo que es el "punto mas alto" -- el ancestro comun de los dos extremos del camino. Ese nodo es donde el camino "dobla". Si pensamos desde la perspectiva de cada nodo como posible punto de inflexion, el problema se descompone limpiamente.

## La Dualidad del DFS

La estrategia se basa en un DFS post-orden donde cada llamada recursiva tiene **dos responsabilidades** simultaneas:

1. **Actualizar el maximo global:** Para el nodo actual como "techo" del camino, la mejor suma es `nodo.val + ganancia_izquierda + ganancia_derecha`, donde cada ganancia es el maximo entre la contribucion del subarbol correspondiente y cero (porque siempre podemos elegir no tomar una rama negativa).

2. **Reportar hacia arriba:** Cuando el nodo devuelve un valor a su padre, solo puede ofrecer *una* de sus dos ramas, no ambas. Un camino no puede bifurcarse -- si el padre va a usar este nodo, el camino debe continuar en linea recta. Asi que devolvemos `nodo.val + max(ganancia_izquierda, ganancia_derecha)`.

Esta separacion es el corazon de la solucion. El maximo global considera al nodo como vertice del camino completo (usando ambas ramas). El valor de retorno lo trata como un nodo de paso (usando solo la mejor rama). Sin esta distincion, seria imposible capturar caminos que "doblan" en algun nodo intermedio.

### Un Ejemplo Concreto

Consideremos el arbol `[-10, 9, 20, null, null, 15, 7]`:

```
    -10
    / \
   9   20
      / \
     15   7
```

- En el nodo `9` (hoja): ganancia izquierda = 0, ganancia derecha = 0. Camino local: 9. Retorna 9.
- En el nodo `15` (hoja): camino local: 15. Retorna 15.
- En el nodo `7` (hoja): camino local: 7. Retorna 7.
- En el nodo `20`: ganancia izquierda = 15, ganancia derecha = 7. Camino local: 20 + 15 + 7 = 42. Retorna 20 + max(15, 7) = 35.
- En el nodo `-10`: ganancia izquierda = 9, ganancia derecha = 35. Camino local: -10 + 9 + 35 = 34. Pero el maximo global ya es 42.

La respuesta es **42**: el camino `15 -> 20 -> 7`, donde el nodo `20` es el punto de inflexion. Notemos que este camino nunca pasa por la raiz.

### ¿Por que `max(..., 0)`?

Aplicar `max(ganancia, 0)` a las contribuciones de los hijos es equivalente a decir "si un subarbol tiene suma negativa, simplemente no lo incluimos en el camino". Esto maneja elegantemente los nodos con valores negativos: el camino optimo podria ser un solo nodo positivo rodeado de nodos negativos.

## Solucion en Rust

```rust
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut global_max = i32::MIN;

        Self::dfs(&root, &mut global_max);

        global_max
    }

    fn dfs(node_opt: &Option<Rc<RefCell<TreeNode>>>, global_max: &mut i32) -> i32 {
        if let Some(node_rc) = node_opt {
            let node = node_rc.borrow();

            let left_gain = cmp::max(Self::dfs(&node.left, global_max), 0);
            let right_gain = cmp::max(Self::dfs(&node.right, global_max), 0);

            let current_path_sum = node.val + left_gain + right_gain;
            *global_max = cmp::max(*global_max, current_path_sum);

            node.val + cmp::max(left_gain, right_gain)
        } else {
            0
        }
    }
}
```

La implementacion en Rust captura la dualidad de forma cristalina. El `global_max` se pasa como referencia mutable a traves de toda la recursion, actualizandose cada vez que un nodo descubre un camino "en forma de arco" mejor que el mejor conocido. El valor de retorno, en cambio, solo ofrece la mejor rama unica hacia el padre. Inicializar `global_max` con `i32::MIN` es crucial: todos los nodos pueden tener valores negativos, y necesitamos que el primer nodo visitado siempre actualice el maximo. El `if let Some` es la forma idiomatica de Rust para manejar la opcion del nodo nulo, devolviendo 0 cuando el subarbol no existe -- un caso base limpio que se integra naturalmente con la logica de `max(..., 0)`.

## Conclusion

Este problema revela una idea poderosa: a veces la funcion recursiva necesita hacer dos cosas a la vez -- reportar un valor hacia arriba para que el padre lo use, y al mismo tiempo calcular algo mas amplio que queda registrado en un estado global. Esa tension entre "lo que devuelvo" y "lo que calculo" es lo que hace que este problema sea Hard. Una vez que internalizas la distincion entre el nodo como vertice del camino y el nodo como punto de paso, la solucion fluye naturalmente, y el arbol entero se procesa en un unico recorrido lineal con espacio proporcional a la altura.
