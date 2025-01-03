"""
// This Source Code Form is subject to the terms of The GNU General Public License v3.0
// Copyright 2024 - Guilherme Santos. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://www.gnu.org/licenses/gpl-3.0.html
"""

import random
import networkx as nx
from networkx.generators.community import LFR_benchmark_graph as benchmark

if __name__ == "__main__":
    base_n = 500  # Base number of nodes
    tau1 = 2.0
    tau2 = 3.5
    min_degree_factor = 100
    max_degree_factor = 10
    min_community_factor = 50
    max_community_factor = 20

    random.seed(42)

    for i, mu in enumerate([round(0.1 * x, 1) for x in range(1, 10)]):
        n = base_n + i * 50  # Increment the number of nodes for each iteration
        min_community = max(30, n // min_community_factor)
        max_community = max(80, n // max_community_factor)
        min_degree = max(10, n // min_degree_factor)
        max_degree = min(50, n // max_degree_factor)

        try:
            G = benchmark(
                n,
                tau1,
                tau2,
                mu,
                min_degree=min_degree,
                max_degree=max_degree,
                min_community=min_community,
                max_community=max_community,
                seed=42,
            )

            save_path = f"src/graphs/artificials/mu-{mu}_n-{n}.edgelist"
            nx.write_edgelist(
                G,
                save_path,
                delimiter=",",
            )

        except Exception as inst:
            print(f"Error generating graph for mu={mu}, n={n}: {inst}")
