
use std::fs::{self};
use std::path::Path;
use serde_json;
use std::env;
use std::fs::exists;
use std::time::{Duration, Instant};

mod graph;
mod algorithm;

use crate::graph::Graph;

use crate::algorithm::{
    genetic_algorithm,
    calculate_objectives
};

fn parse_args(args: &Vec<String>) -> (&str, bool) {
    if args.len() < 2 {
        println!("Usage: <program> <file_path> [ -d for debug ]");
        println!("Example: ./my_program ../graphs/artificial/karate.edgelist -d");
        return ("", false);
    }

    let file_path: &str = &args[1];
    if exists(file_path).is_err() {
        println!("Graph .edgelist file not found.");
        return ("", false);
    }

    let debug_mode: bool = args.len() > 2 && &args[2] == "-d";
    if debug_mode {
        println!("[Warning] Debug mode: {} | This may increase algorithm time", debug_mode);
    }

    (file_path, debug_mode)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let (file_path, 
        debug_mode
    ) = parse_args(&args);

    let start: Instant = Instant::now();

    let graph = Graph::from_edgelist(Path::new(file_path))?;

    let reading_time: Duration = start.elapsed();

    // Run genetic algorithm
    let (best_partition, _fitness_history) = 
    genetic_algorithm(&graph, 
        400, 
        100,
    debug_mode
    );

    let algorithm_time: Duration = start.elapsed();

    // Save best partition to JSON
    let json = serde_json::to_string_pretty(&best_partition)?;
    fs::write("best_partition.json", json)?;


    println!("Algorithm time {:?} | Reading time: {:?}", algorithm_time, reading_time);
    println!("Best partition saved to best_partition.json");
    println!("Final modularity: {}", calculate_objectives(&graph, &best_partition).modularity);
    

    Ok(())
}