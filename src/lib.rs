use daggy::*;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::cmp::min;
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Pref {
    pub size: u32,
    pub iterations: u32,
    pub seed: Option<u64>,
    pub front_prop: f64,
    pub max_forks: u64,
    pub min_forks: u64,
}

pub fn generate_dag(pref: &Pref) -> Dag<(), ()> {
    let mut akk = generate_tree(&pref);

    for _ in 0..pref.iterations {
        let tree = generate_tree(&pref);
        akk = merge_dags(&akk, &tree);
    }

    return akk;
}

fn merge_dags(dag1: &Dag<(), ()>, dag2: &Dag<(), ()>) -> Dag<(), ()> {
    let mut dag = dag1.clone();
    let edges = dag2.raw_edges().to_owned();
    let transformed_edges = edges
        .iter()
        .map(|edge| (edge.source(), edge.target(), edge.weight))
        .filter(|edge| dag1.find_edge(edge.0, edge.1) == Option::None);

    match dag.extend_with_edges(transformed_edges) {
        Ok(_) => {}
        Err(_) => panic!("Couldn't merge generated trees!"),
    }

    return dag;
}

fn generate_tree(args: &Pref) -> Dag<(), (), u32> {
    let size = args.size as usize;
    let mut tree = Dag::with_capacity(size, size - 1);

    let mut rng = get_random_generator(args);

    for _ in 0..size {
        tree.add_node(());
    }

    let mut queue: VecDeque<usize> = VecDeque::new();

    let mut next: usize = 1;
    queue.push_back(0);

    while !queue.is_empty() {
        let front_prop: f64 = rng.gen();

        let current = if front_prop < args.front_prop {
            queue.pop_front().unwrap()
        } else {
            queue.pop_back().unwrap()
        };

        let amount_forks: usize = min(
            rng.gen_range((args.min_forks as usize)..(args.max_forks as usize + 1)),
            size - next,
        );

        for _ in 0..amount_forks {
            tree.add_edge(NodeIndex::new(current), NodeIndex::new(next), ())
                .expect("Can't add edge to tree!");
            queue.push_back(next);
            next += 1;
        }

        if next >= size {
            break;
        }
    }

    return tree;
}

fn get_random_generator(args: &Pref) -> ChaCha20Rng {
    let x = match args.seed {
        Some(seed) => ChaCha20Rng::seed_from_u64(seed),
        None => ChaCha20Rng::from_entropy(),
    };

    return x;
}
