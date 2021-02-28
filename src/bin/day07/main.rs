use algo::has_path_connecting;
use petgraph::{algo, Directed, Direction};
use petgraph::graphmap::GraphMap;
use petgraph::prelude::DiGraphMap;
use regex::Regex;
use anyhow::Result;

fn nested_bag_cost<'a>(graph: &GraphMap<&'a str, usize, Directed>, index: &'a str) -> usize {
    let children = graph.neighbors_directed(index, Direction::Outgoing);
    children.map(|x| {
        let weight = graph[(index, x)];
        weight * nested_bag_cost(graph, x)
    }).sum::<usize>() + 1
}

fn main() -> Result<()> {
    let re = Regex::new(r"(.+?) bags contain (?:(\d.+)|no other bags)\.")?;
    let s = include_str!("input.txt");
    let captures = re.captures_iter(s);
    let re_rule = Regex::new(r"(?:(\d+) (.+?) bag)")?;

    let edges: Vec<_> = captures.filter_map(|x| {
        let from = x.get(1)?.as_str();
        x.get(2).map(|rhs| {
            re_rule.captures_iter(rhs.as_str()).map(move |m| {
                let to = m.get(2).unwrap().as_str();
                let weight = m[1].parse().unwrap();
                (from, to, weight)
            })
        })
    }).flatten().collect();

    let g = DiGraphMap::from_edges(&edges);
    let mut y: Vec<_> = g.nodes().filter(|x| *x != "shiny gold" && has_path_connecting(&g, x, "shiny gold", None)).collect();
    y.sort();
    y.dedup();
    println!("Problem 1: {}", y.len());
    println!("Problem 2: {}", nested_bag_cost(&g, "shiny gold") - 1);
    Ok(())
}
