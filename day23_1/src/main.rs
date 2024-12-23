use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;
use petgraph::visit::EdgeRef;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn read_edges(path: &Path) -> Result<Vec<(String, String)>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"([a-z]{2})-([a-z]{2})").unwrap();

    let mut result = vec![];

    for line in reader.lines() {
        let line = line?;
        let caps = re.captures(line.as_str()).unwrap();

        result.push((
            caps.get(1).unwrap().as_str().to_owned(),
            caps.get(2).unwrap().as_str().to_owned(),
        ));
    }

    Ok(result)
}

fn build_graph(edges: Vec<(String, String)>) -> (UnGraph<u32, ()>, HashMap<u32, String>) {
    let mut id_map = HashMap::new();
    let mut edge_ids: Vec<(u32, u32)> = vec![];
    let mut next_id = 0u32;
    for (a, b) in edges {
        let id_a = *id_map.entry(a.clone()).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });

        let id_b = *id_map.entry(b.clone()).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });

        edge_ids.push((id_a, id_b));
    }

    let mut nodemap = HashMap::new();
    for (key, &value) in id_map.iter() {
        nodemap.insert(value, key.clone());
    }

    let g = UnGraph::<u32, ()>::from_edges(&edge_ids);

    (g, nodemap)
}

fn find_triangles(graph: &UnGraph<u32, ()>) -> Vec<Vec<u32>> {
    let mut visited_edges = HashSet::new();
    let mut result = vec![];

    for edge in graph.edge_references() {
        let (a, b) = (edge.source(), edge.target());

        let neighbors_a: HashSet<NodeIndex> = graph.neighbors(a).collect();
        let neighbors_b: HashSet<NodeIndex> = graph.neighbors(b).collect();

        for &common in neighbors_a.intersection(&neighbors_b) {
            let mut cycle = vec![a, b, common];
            cycle.sort_unstable();
            if visited_edges.insert(cycle.clone()) {
                result.push(cycle.into_iter().map(|i| i.index() as u32).collect());
            }
        }
    }

    result
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    let edges = read_edges(&file_path)?;
    let (g, m) = build_graph(edges);
    let cycles = find_triangles(&g);

    for c in cycles {
        let cycle = c
            .into_iter()
            .map(|n| m.get(&n).unwrap().clone())
            .collect::<Vec<String>>();

        if cycle.iter().any(|i| i.starts_with("t")) {
            println!("{:?}", cycle);
        }
    }

    Ok(())
}
