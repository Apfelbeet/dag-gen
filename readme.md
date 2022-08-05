# dag-gen
-----
Generator for random [directed acyclic graphs (DAG)](https://en.wikipedia.org/wiki/Directed_acyclic_graph).

## Usage
-----
| Option/Flag | short | Description | Mandatory | Default
| --- | --- | --- | --- | --- |
| --size | -s | Number of nodes in the DAG | yes | -
| --iterations | -i | Number of nodes tree getting merged | no | 3
| --seed | | Number of nodes tree getting merged | no | random
| --min-forks | | Minimum number of children a node in a tree must have. | no | 1
| --max-forks | | Maximum number of children a node in a tree can have. | no | 4
| --front-prop | | Probability growing at a deeper node.  | no | 0.25

## Functionality
-----
The basic idea is to generate multiple trees (`--iterations` many) with `--size`
number of nodes. We then merge all generated edges of all trees into one graph.
The resulting tree will be a DAG.

The remaining arguments control certain behavior when generating the trees. 
