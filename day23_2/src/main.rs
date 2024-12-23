use petgraph::graph::UnGraph;
use petgraph::visit::{GetAdjacencyMatrix, IntoNeighbors, IntoNodeIdentifiers};
use regex::Regex;
use std::collections::HashMap;
use std::hash::Hash;
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

// Below two functions from https://github.com/petgraph/petgraph/pull/662/commits/cee17df1398adeae9f4696d3f4caba5662b37d63

fn bron_kerbosch_pivot<G>(
    g: G,
    adj_mat: &G::AdjMatrix,
    r: HashSet<G::NodeId>,
    mut p: HashSet<G::NodeId>,
    mut x: HashSet<G::NodeId>,
) -> Vec<HashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let mut cliques = Vec::with_capacity(1);
    if p.is_empty() {
        if x.is_empty() {
            cliques.push(r);
        }
        return cliques;
    }
    // pick the pivot u to be the vertex with max degree
    let u = p.iter().max_by_key(|&v| g.neighbors(*v).count()).unwrap();
    let mut todo = p
        .iter()
        .filter(|&v| *u == *v || !g.is_adjacent(adj_mat, *u, *v) || !g.is_adjacent(adj_mat, *v, *u)) //skip neighbors of pivot
        .cloned()
        .collect::<Vec<G::NodeId>>();
    while let Some(v) = todo.pop() {
        let neighbors = HashSet::from_iter(g.neighbors(v));
        p.remove(&v);
        let mut next_r = r.clone();
        next_r.insert(v);

        let next_p = p
            .intersection(&neighbors)
            .cloned()
            .collect::<HashSet<G::NodeId>>();
        let next_x = x
            .intersection(&neighbors)
            .cloned()
            .collect::<HashSet<G::NodeId>>();

        cliques.extend(bron_kerbosch_pivot(g, adj_mat, next_r, next_p, next_x));

        x.insert(v);
    }

    cliques
}

pub fn maximal_cliques<G>(g: G) -> Vec<HashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNodeIdentifiers + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let adj_mat = g.adjacency_matrix();
    let r = HashSet::new();
    let p = g.node_identifiers().collect::<HashSet<G::NodeId>>();
    let x = HashSet::new();
    return bron_kerbosch_pivot(g, &adj_mat, r, p, x);
}

// Above two functions from https://github.com/petgraph/petgraph/pull/662/commits/cee17df1398adeae9f4696d3f4caba5662b37d63

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    let edges = read_edges(&file_path)?;
    let (g, m) = build_graph(edges);

    let cliques = maximal_cliques(&g);

    let cc = cliques.into_iter().map(|c| {
        let names = c.into_iter().map(|i| m.get(&(i.index() as u32)).unwrap().clone()).collect::<Vec<String>>();
        names
    }).collect::<Vec<Vec<String>>>();

    if let Some(mut biggest_clique) = cc.into_iter().max_by_key(|v| v.len()) {
        biggest_clique.sort();
        println!("{}", biggest_clique.join(","));
    }

    Ok(())
}
