use std::collections::{hash_set, BTreeSet, HashMap, HashSet};

use itertools::Itertools;

fn main() {
    advent_of_code_24::solve("day23", parse, part1, part2);
}

type Conns = Vec<(String, String)>;

fn parse(s: &str) -> Conns {
    s.lines()
        .map(|l| l.split('-').map(String::from).collect_tuple().unwrap())
        .collect_vec()
}

fn part1(conns: &Conns) -> String {
    let neighbours = neighbour_set(conns);

    let mut t_chains = HashSet::new();
    for (a, bs) in neighbours.iter() {
        for b in bs {
            // Use the fact that we have an undirected graph. So we can always assume the
            // t is present in either at the (left) end or in the center.
            if a.chars().next().unwrap() != 't' && b.chars().next().unwrap() != 't' {
                continue;
            }

            for c in neighbours
                .get(b)
                .unwrap()
                .intersection(neighbours.get(a).unwrap())
            {
                let mut chain = [a.clone(), b.clone(), c.clone()];
                chain.sort();
                t_chains.insert(chain);
            }
        }
    }
    t_chains.len().to_string()
}

fn part2(conns: &Conns) -> String {
    let neighbours = neighbour_set(conns);

    let mut largest_net = HashSet::new();
    let mut queue = BTreeSet::from_iter(neighbours.keys());

    // The largest net can be discovered from any of the nodes that are part of it.
    while let Some(root) = queue.pop_first() {
        let mut curr_net = HashSet::new();
        curr_net.insert(root.clone());

        let mut edge = vec![root];
        while let Some(curr) = edge.pop() {
            for cand in neighbours.get(curr).unwrap() {
                // Check if candidate is fully connected with the whole current net.
                if neighbours.get(cand).unwrap().is_superset(&curr_net) {
                    // New candidate is fully connected. Add it to the net.
                    curr_net.insert(cand.clone());
                    edge.push(cand);
                }
            }
        }

        // We can remove all elements from this net from the queue
        // This is probably the crux.
        for n in curr_net.iter() {
            queue.remove(n);
        }

        if curr_net.len() > largest_net.len() {
            largest_net = curr_net;
        }
    }
    largest_net.into_iter().sorted().join(",")
}

fn neighbour_set(conns: &Conns) -> HashMap<String, HashSet<String>> {
    let mut neibours: HashMap<String, HashSet<String>> = HashMap::new();
    for (a, b) in conns {
        neibours.entry(a.clone()).or_default().insert(b.clone());
        neibours.entry(b.clone()).or_default().insert(a.clone());
    }
    neibours
}

#[allow(dead_code)]
const TEST_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

#[test]
fn test_part1() {
    advent_of_code_24::test1(TEST_INPUT, "7".to_string(), parse, part1);
}

#[test]
fn test_part2() {
    advent_of_code_24::test1(TEST_INPUT, "co,de,ka,ta".to_string(), parse, part2);
}
