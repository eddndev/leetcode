---
title: "0332 Reconstruct Itinerary - ES"
problemUrl: "https://leetcode.com/problems/reconstruct-itinerary/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["graph", "dfs", "eulerian-path", "backtracking", "hash-map"]
complexity:
  time: "O(E log E) donde E es el numero de boletos"
  space: "O(E)"
---

# Pasajero sin Escalas: Desenredando el Camino de Euler

## El Problema
Dado un conjunto de boletos de avion representados como pares `[from, to]`, reconstruir el itinerario en orden. Todos los boletos pertenecen a una persona que parte desde `"JFK"`. El itinerario debe usar todos los boletos exactamente una vez. Si existen multiples itinerarios validos, devolver el que tenga el menor orden lexicografico.

## La Intuicion Inicial

Cuando vi este problema por primera vez, pense en el recorrido de un grafo dirigido donde cada boleto es una arista y cada aeropuerto un nodo. Necesito encontrar un camino que use cada arista exactamente una vez -- esto es precisamente un camino euleriano. El hecho de que debamos partir desde `"JFK"` fija nuestro punto de partida, y la restriccion de orden lexicografico determina como desempatar entre multiples opciones.

La idea clave es que no estoy buscando un camino que visite cada nodo una vez (hamiltoniano, que seria NP-hard), sino un camino que use cada arista una vez. Esta distincion es fundamental, porque los caminos eulerianos se pueden encontrar en tiempo polinomial usando el algoritmo de Hierholzer.

## El Algoritmo de Hierholzer Adaptado

El truco elegante detras de esta solucion es el algoritmo de Hierholzer para encontrar circuitos eulerianos. La idea es simple pero poderosa: hago DFS desde `"JFK"`, y cada vez que llego a un nodo sin aristas salientes disponibles, lo agrego al resultado. Al final, invierto el resultado para obtener el itinerario correcto.

Pero, por que funciona invertir? Cuando el DFS llega a un callejon sin salida, ese nodo necesariamente es el ultimo del itinerario. Al retroceder, cada nodo se agrega en orden inverso de finalizacion, que es exactamente el orden correcto del recorrido cuando se invierte.

## El Orden Lexicografico

Para garantizar el menor orden lexicografico, necesito visitar los destinos en orden alfabetico. Pero como voy a consumir los destinos usando `pop()` (que remueve del final del vector), ordeno las listas de adyacencia en orden inverso. De esta forma, el destino lexicograficamente menor queda al final del vector y es el primero en ser extraido con `pop()`.

Este detalle es sutil pero crucial. Si ordenara de menor a mayor y usara `pop()`, visitaria primero el destino lexicograficamente mayor. Al invertir el ordenamiento, `pop()` me da el menor, que es exactamente lo que necesito.

## La Mecanica del DFS

El DFS opera como un ciclo que extrae destinos continuamente del aeropuerto actual. Cada vez que extraigo un destino, hago una llamada recursiva para explorarlo. El `pop()` es destructivo -- consume la arista del grafo, asegurando que cada boleto se use exactamente una vez. Cuando ya no hay destinos disponibles para un aeropuerto, ese aeropuerto se agrega a la ruta.

La recursion se desenrolla naturalmente. Los nodos sin salida se agregan primero a la ruta, seguidos de los nodos que conducen a ellos, construyendo el itinerario de atras hacia adelante.

## Solucion en Rust

```rust
use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        for mut ticket in tickets {
            let to = ticket.pop().unwrap();
            let from = ticket.pop().unwrap();
            graph.entry(from).or_default().push(to);
        }

        for destinations in graph.values_mut() {
            destinations.sort_by(|a, b| b.cmp(a));
        }

        let mut route = Vec::new();

        Self::dfs("JFK".to_string(), &mut graph, &mut route);

        route.reverse();
        route
    }

    fn dfs(airport: String, graph: &mut HashMap<String, Vec<String>>, route: &mut Vec<String>) {
        loop {
            let next_dest = if let Some(dests) = graph.get_mut(&airport) {
                dests.pop()
            } else {
                None
            };

            if let Some(next) = next_dest {
                Self::dfs(next, graph, route);
            } else {
                break;
            }
        }

        // Agregamos el aeropuerto actual a la ruta solo cuando ya no podemos avanzar más.
        route.push(airport);
    }
}
```

La implementacion construye primero el grafo de adyacencia a partir de los boletos, extrayendo `to` y `from` mediante `pop()` del vector de cada boleto. Luego ordena cada lista de destinos en orden descendente para que `pop()` extraiga el menor lexicograficamente. La funcion `dfs` itera en un ciclo, extrayendo destinos del aeropuerto actual y recursando sobre cada uno. Cuando un aeropuerto queda sin destinos, el ciclo termina y el aeropuerto se agrega a `route`. Finalmente, `route.reverse()` transforma el orden de post-finalizacion en el itinerario correcto.

El uso de `HashMap<String, Vec<String>>` como lista de adyacencia es natural en Rust. El patron `get_mut` seguido de `pop()` garantiza la mutabilidad necesaria para consumir aristas, y el `or_default()` al construir el grafo maneja elegantemente aeropuertos que aparecen por primera vez.

## Conclusion

Reconstruct Itinerary es un ejemplo clasico de como reconocer la estructura subyacente de un problema transforma su dificultad. Lo que parece un problema de backtracking costoso resulta ser un camino euleriano que se resuelve con el algoritmo de Hierholzer en tiempo `O(E log E)`, donde el factor logaritmico proviene unicamente del ordenamiento. El truco de ordenar en reversa para explotar `pop()`, combinado con la construccion inversa del itinerario, produce una solucion compacta y elegante que maneja tanto la unicidad de uso de aristas como la restriccion lexicografica de forma simultanea.
