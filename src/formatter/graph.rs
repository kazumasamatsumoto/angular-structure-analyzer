// src/formatter/graph.rs
use anyhow::Result;
use colored::*;
use std::collections::{HashMap, HashSet};
use crate::models::*;

pub fn output_dependencies(dependencies: &[Dependency]) -> Result<()> {
    println!("\n{} Dependency Graph:", "GRAPH:".green().bold());
    
    if dependencies.is_empty() {
        println!("  No dependencies found");
        return Ok(());
    }
    
    // Create a graph representation
    let mut graph = HashMap::new();
    let mut nodes = HashSet::new();
    
    for dep in dependencies {
        let source = dep.source.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
            
        let target = dep.target.clone();
        
        nodes.insert(source.clone());
        nodes.insert(target.clone());
        
        let entry = graph.entry(source).or_insert_with(HashSet::new);
        entry.insert((target, dep.import_type.clone()));
    }
    
    // Print the graph
    print_ascii_graph(&graph);
    
    Ok(())
}

fn print_ascii_graph(graph: &HashMap<String, HashSet<(String, ImportType)>>) {
    let mut sorted_sources: Vec<_> = graph.keys().collect();
    sorted_sources.sort();
    
    for source in sorted_sources {
        println!("  {} {}:", "Node:".cyan(), source.yellow());
        
        let deps = &graph[source];
        let mut sorted_deps: Vec<_> = deps.iter().collect();
        sorted_deps.sort_by(|a, b| a.0.cmp(&b.0));
        
        for (target, import_type) in sorted_deps {
            let type_str = match import_type {
                ImportType::Component => "Component".cyan(),
                ImportType::Service => "Service".green(),
                ImportType::Module => "Module".yellow(),
                ImportType::Directive => "Directive".magenta(),
                ImportType::Pipe => "Pipe".blue(),
                ImportType::Guard => "Guard".red(),
                ImportType::Resolver => "Resolver".bright_red(),
                ImportType::Model => "Model".bright_blue(),
                ImportType::Other => "Other".normal(),
            };
            
            println!("    └─→ {} ({})", target.green(), type_str);
        }
        
        println!();
    }
}
