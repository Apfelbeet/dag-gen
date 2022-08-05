use clap::Parser;
use dag_gen::*;
use daggy::*;
use petgraph::dot::{Config, Dot};

#[derive(Parser, Debug, Default)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    size: u32,

    #[clap(short, long, value_parser, default_value_t = 3)]
    iterations: u32,

    #[clap(long, value_parser)]
    seed: Option<u64>,

    #[clap(long, value_parser, default_value_t = 0.25)]
    front_prop: f64,

    #[clap(long, value_parser, default_value_t = 4)]
    max_forks: u64,

    #[clap(long, value_parser, default_value_t = 1)]
    min_forks: u64,
}

impl Into<Pref> for Args {
    fn into(self) -> Pref {
        return Pref {
            size: self.size,
            iterations: self.iterations,
            seed: self.seed,
            front_prop: self.front_prop,
            max_forks: self.max_forks,
            min_forks: self.min_forks,
        }
    }
}

fn main() {
    let args = Args::parse();
    let dag = generate_dag(&args.into());

    println!(
        "{:?}",
        Dot::with_config(dag.graph(), &[Config::EdgeNoLabel, Config::NodeIndexLabel])
    );
}

