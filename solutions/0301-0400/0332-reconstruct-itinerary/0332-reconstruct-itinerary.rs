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
