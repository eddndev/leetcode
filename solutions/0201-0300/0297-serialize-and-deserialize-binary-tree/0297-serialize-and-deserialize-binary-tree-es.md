---
title: "0297 Serialize and Deserialize Binary Tree - ES"
problemUrl: "https://leetcode.com/problems/serialize-and-deserialize-binary-tree/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["tree", "binary-tree", "design", "string", "dfs", "recursion"]
complexity:
  time: "O(N) donde N es el numero de nodos del arbol"
  space: "O(N) para almacenar la cadena serializada y la pila de recursion"
---

# Empaquetando Arboles en Cadenas y Resucitandolos

## El Problema
Disenar un algoritmo para serializar y deserializar un arbol binario. La serializacion es el proceso de convertir una estructura de datos en una secuencia de bits para que pueda almacenarse o transmitirse y reconstruirse posteriormente. No hay restriccion sobre como debe funcionar el algoritmo de serializacion/deserializacion, solo se requiere que un arbol binario pueda ser serializado a una cadena y que esta cadena pueda ser deserializada de vuelta a la estructura original del arbol.

## La Tension entre Estructura y Texto

Un arbol binario es inherentemente una estructura bidimensional: cada nodo se ramifica hacia la izquierda y hacia la derecha, formando una jerarquia que no tiene una representacion lineal obvia. El desafio de este problema no es simplemente recorrer el arbol, sino codificarlo de tal manera que la *forma exacta* del arbol -- incluyendo donde estan los nodos nulos -- pueda recuperarse sin ambiguedad a partir de una cadena plana.

Mi primer impulso fue usar un recorrido BFS nivel por nivel, como la representacion clasica de LeetCode. Pero reflexionando, un recorrido preorden (DFS) resulta mas elegante: la raiz aparece primero, seguida recursivamente por el subarbol izquierdo completo y luego el subarbol derecho completo. La clave es registrar explicitamente los nodos nulos con un centinela, porque sin ellos seria imposible distinguir arboles con la misma secuencia de valores pero diferente estructura.

## La Estrategia: Preorden con Centinelas

### Serializacion

La idea es directa: recorro el arbol en preorden (raiz, izquierda, derecha). Para cada nodo existente, escribo su valor seguido de un espacio. Para cada posicion nula, escribo `#` seguido de un espacio. El espacio actua como delimitador universal.

Consideremos el arbol:

```
    1
   / \
  2   3
     / \
    4   5
```

El recorrido preorden con centinelas produce: `1 2 # # 3 4 # # 5 # #`

Cada nodo contribuye exactamente un token, y cada posicion nula contribuye un `#`. Esta representacion es *completa*: no hay ambiguedad posible. Dos arboles diferentes siempre produciran cadenas diferentes, y una cadena valida siempre reconstruye exactamente un arbol.

### Deserializacion

Aqui es donde la magia del preorden brilla. Creo un iterador sobre los tokens separados por espacios y consumo uno por uno. Para cada token:

1. Si es `#`, devuelvo `None` -- esta posicion es un nodo nulo.
2. Si es un numero, creo un `TreeNode` con ese valor, y recursivamente deserializo su hijo izquierdo y luego su hijo derecho.

La belleza de este enfoque es que el iterador mantiene el estado implicitamente. No necesito indices, no necesito calcular posiciones. Cada llamada recursiva simplemente consume el siguiente token disponible, y el orden preorden garantiza que los tokens se alinean perfectamente con la estructura del arbol.

### Reconstruyendo el Ejemplo

Con la cadena `1 2 # # 3 4 # # 5 # #`:

```
Consumo "1" -> creo nodo(1)
  Izquierda: consumo "2" -> creo nodo(2)
    Izquierda: consumo "#" -> None
    Derecha: consumo "#" -> None
  Derecha: consumo "3" -> creo nodo(3)
    Izquierda: consumo "4" -> creo nodo(4)
      Izquierda: consumo "#" -> None
      Derecha: consumo "#" -> None
    Derecha: consumo "5" -> creo nodo(5)
      Izquierda: consumo "#" -> None
      Derecha: consumo "#" -> None
```

El arbol original se reconstruye perfectamente, nodo por nodo, sin necesidad de calcular relaciones padre-hijo explicitamente.

## Por Que Preorden y No Otro Recorrido

Un recorrido inorden no funcionaria aqui porque la posicion de la raiz no es predecible sin informacion adicional. Un recorrido postorden podria funcionar, pero la deserializacion seria menos intuitiva -- tendria que leer los tokens en reversa o usar una pila explicita. El preorden es la eleccion natural porque el primer token siempre es la raiz, lo que permite una reconstruccion top-down que se alinea perfectamente con la recursion.

## Solucion en Rust

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::new();
        self.r_serialize(root, &mut result);
        result
    }

    fn r_serialize(&self, root: Option<Rc<RefCell<TreeNode>>>, out: &mut String) {
        match root {
            Some(node) => {
                let n = node.borrow();
                out.push_str(&n.val.to_string());
                out.push(' ');

                self.r_serialize(n.left.clone(), out);
                self.r_serialize(n.right.clone(), out);
            }
            None => {
                out.push_str("# ");
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut iter = data.split_whitespace();
        self.r_deserialize(&mut iter)
    }

    fn r_deserialize(&self, iter: &mut std::str::SplitWhitespace) -> Option<Rc<RefCell<TreeNode>>> {
        match iter.next() {
            Some(val_str) => {
                if val_str == "#" {
                    return None;
                }

                if let Ok(val) = val_str.parse::<i32>() {
                    let mut node = TreeNode::new(val);
                    node.left = self.r_deserialize(iter);
                    node.right = self.r_deserialize(iter);

                    Some(Rc::new(RefCell::new(node)))
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
```

La implementacion en Rust usa `Rc<RefCell<TreeNode>>` para la propiedad compartida y la mutabilidad interior que requiere la estructura de arbol de LeetCode. En la serializacion, `r_serialize` recibe una referencia mutable a un `String` para evitar asignaciones innecesarias -- cada nodo simplemente agrega su representacion al buffer existente. El metodo `borrow()` sobre el `RefCell` obtiene una referencia inmutable al nodo, y `n.left.clone()` clona el `Rc` (incrementando el contador de referencias, no copiando el nodo) para pasarlo a la llamada recursiva. En la deserializacion, `split_whitespace()` produce un iterador perezoso que se pasa por referencia mutable a traves de la recursion, lo que permite que cada llamada consuma exactamente un token sin necesidad de un indice externo. El `if let Ok(val)` maneja defensivamente el caso de un token no numerico, aunque por construccion la cadena serializada solo contendra numeros enteros y el centinela `#`. El patron `match` sobre `iter.next()` gestiona naturalmente el fin del iterador, devolviendo `None` si la cadena se agota prematuramente.

## Conclusion

Serialize and Deserialize Binary Tree es un problema de diseno que parece abierto pero tiene una solucion elegantemente restrictiva. El recorrido preorden con centinelas nulos captura la estructura completa del arbol en una cadena lineal, y la deserializacion la reconstruye consumiendo tokens secuencialmente con recursion. No se necesitan indices, no se necesitan colas, no se necesitan calculos de posicion. Solo la confianza en que el orden preorden y los centinelas contienen toda la informacion estructural necesaria. La simetria entre serializar y deserializar -- ambas funciones recursivas que procesan raiz, izquierda, derecha en el mismo orden -- es lo que hace que esta solucion sea a la vez correcta por construccion y facil de razonar.
