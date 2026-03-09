---
title: "0332 Reconstruct Itinerary - EN"
problemUrl: "https://leetcode.com/problems/reconstruct-itinerary/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["graph", "dfs", "eulerian-path", "backtracking", "hash-map"]
complexity:
  time: "O(E log E) where E is the number of tickets"
  space: "O(E)"
---

# Non-Stop Passenger: Unraveling the Eulerian Path

## The Problem
Given a list of airline tickets represented as pairs `[from, to]`, reconstruct the itinerary in order. All tickets belong to a person who departs from `"JFK"`. The itinerary must use all tickets exactly once. If there are multiple valid itineraries, return the one with the smallest lexicographic order.

## The Initial Intuition

When I first looked at this problem, I thought about traversing a directed graph where each ticket is an edge and each airport is a node. I need to find a path that uses every edge exactly once -- this is precisely an Eulerian path. The fact that we must start from `"JFK"` fixes our starting point, and the lexicographic constraint determines how to break ties among multiple choices.

The key insight is that I'm not looking for a path that visits every node once (Hamiltonian, which would be NP-hard), but a path that uses every edge once. This distinction is fundamental, because Eulerian paths can be found in polynomial time using Hierholzer's algorithm.

## Hierholzer's Algorithm Adapted

The elegant trick behind this solution is Hierholzer's algorithm for finding Eulerian circuits. The idea is simple yet powerful: I run DFS from `"JFK"`, and every time I reach a node with no available outgoing edges, I add it to the result. At the end, I reverse the result to obtain the correct itinerary.

But why does reversing work? When the DFS reaches a dead end, that node is necessarily the last one in the itinerary. As I backtrack, each node gets added in reverse finishing order, which is exactly the correct traversal order when flipped.

## The Lexicographic Ordering

To guarantee the smallest lexicographic order, I need to visit destinations alphabetically. But since I'll be consuming destinations using `pop()` (which removes from the end of the vector), I sort each adjacency list in reverse order. This way, the lexicographically smallest destination sits at the end of the vector and is the first one extracted by `pop()`.

This detail is subtle but crucial. If I sorted smallest-to-largest and used `pop()`, I'd visit the lexicographically largest destination first. By reversing the sort, `pop()` gives me the smallest, which is exactly what I need.

## The DFS Mechanics

The DFS operates as a loop that continuously extracts destinations from the current airport. Each time I extract a destination, I make a recursive call to explore it. The `pop()` is destructive -- it consumes the edge from the graph, ensuring each ticket is used exactly once. When no more destinations are available for an airport, that airport gets added to the route.

The recursion unwinds naturally. Dead-end nodes get added to the route first, followed by the nodes that lead to them, building the itinerary from back to front.

## Rust Solution

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

The implementation first builds the adjacency graph from the tickets, extracting `to` and `from` via `pop()` on each ticket vector. It then sorts each destination list in descending order so that `pop()` extracts the lexicographically smallest one. The `dfs` function loops, popping destinations from the current airport and recursing on each one. When an airport runs out of destinations, the loop breaks and the airport gets pushed onto `route`. Finally, `route.reverse()` transforms the post-finishing order into the correct itinerary.

The use of `HashMap<String, Vec<String>>` as an adjacency list is natural in Rust. The `get_mut` pattern followed by `pop()` ensures the mutability needed to consume edges, and `or_default()` when building the graph elegantly handles airports appearing for the first time.

## Conclusion

Reconstruct Itinerary is a classic example of how recognizing the underlying structure of a problem transforms its difficulty. What looks like an expensive backtracking problem turns out to be an Eulerian path solvable with Hierholzer's algorithm in `O(E log E)` time, where the logarithmic factor comes solely from sorting. The trick of sorting in reverse to exploit `pop()`, combined with the reverse construction of the itinerary, produces a compact and elegant solution that handles both the single-use edge constraint and the lexicographic requirement simultaneously.
