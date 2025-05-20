// src/main.rs
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::process;
use std::path::{Path, PathBuf};

mod analyzer;
mod formatter;
mod models;
mod utils;
use colored::control;
use atty::Stream;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to Angular project (defaults to current directory)
    #[arg(default_value = ".")]
    path: String,

    #[command(subcommand)]
    command: Option<Commands>,

    /// Include test files in the analysis
    #[arg(short = 't', long)]
    include_tests: bool,

    /// Include styles files in the analysis
    #[arg(short = 's', long)]
    include_styles: bool,

    /// Include node_modules in the analysis (not recommended)
    #[arg(long)]
    include_node_modules: bool,

    /// Maximum depth of the directory tree to display
    #[arg(short, long, default_value_t = 10)]
    max_depth: usize,

    /// JSON output format
    #[arg(short, long)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze components in the project
    Components {
        /// Show detailed information about each component
        #[arg(short, long)]
        detailed: bool,
    },
    /// Analyze services in the project
    Services {
        /// Show detailed information about each service
        #[arg(short, long)]
        detailed: bool,
    },
    /// Analyze modules in the project
    Modules {
        /// Show detailed information about each module
        #[arg(short, long)]
        detailed: bool,
    },
    /// Analyze dependencies between files
    Dependencies {
        /// Show dependencies as a graph
        #[arg(short, long)]
        graph: bool,
    },
    /// Analyze routes in the project
    Routes {},
}

fn main() -> Result<()> {
    let use_color = atty::is(Stream::Stdout);
    control::set_override(use_color);
    let cli = Cli::parse();
    
    // Set up the path to analyze
    let path = PathBuf::from(&cli.path);
    
    if !path.exists() {
        eprintln!("{} Path does not exist: {}", "ERROR:".red().bold(), path.display());
        process::exit(1);
    }
    
    // Check if this is an Angular project
    if !is_angular_project(&path) {
        eprintln!("{} The specified path does not appear to be an Angular project.", "WARNING:".yellow().bold());
        eprintln!("Continuing anyway, but results may not be accurate.");
    }
    
    // Create the analyzer with the specified options
    let mut analyzer = analyzer::Analyzer::new(
        path,
        cli.include_tests,
        cli.include_styles,
        false,
        cli.max_depth,
    );
    
    // Run the analyzer based on the command
    match cli.command {
        Some(Commands::Components { detailed }) => {
            let components = analyzer.analyze_components().context("Failed to analyze components")?;
            if cli.json {
                formatter::json::output_components(&components)?;
            } else {
                formatter::text::output_components(&components, detailed)?;
            }
        },
        Some(Commands::Services { detailed }) => {
            let services = analyzer.analyze_services().context("Failed to analyze services")?;
            if cli.json {
                formatter::json::output_services(&services)?;
            } else {
                formatter::text::output_services(&services, detailed)?;
            }
        },
        Some(Commands::Modules { detailed }) => {
            let modules = analyzer.analyze_modules().context("Failed to analyze modules")?;
            if cli.json {
                formatter::json::output_modules(&modules)?;
            } else {
                formatter::text::output_modules(&modules, detailed)?;
            }
        },
        Some(Commands::Dependencies { graph }) => {
            let dependencies = analyzer.analyze_dependencies().context("Failed to analyze dependencies")?;
            if cli.json {
                formatter::json::output_dependencies(&dependencies)?;
            } else if graph {
                formatter::graph::output_dependencies(&dependencies)?;
            } else {
                formatter::text::output_dependencies(&dependencies)?;
            }
        },
        Some(Commands::Routes {}) => {
            let routes = analyzer.analyze_routes().context("Failed to analyze routes")?;
            if cli.json {
                formatter::json::output_routes(&routes)?;
            } else {
                formatter::text::output_routes(&routes)?;
            }
        },
        None => {
            // Default command: show the full project structure
            let structure = analyzer.analyze_structure().context("Failed to analyze project structure")?;
            if cli.json {
                formatter::json::output_structure(&structure)?;
            } else {
                formatter::text::output_structure(&structure)?;
            }
        },
    }
    
    Ok(())
}

fn is_angular_project(path: &Path) -> bool {
    // Check for common Angular project files
    let angular_json = path.join("angular.json");
    let package_json = path.join("package.json");
    
    if angular_json.exists() {
        return true;
    }
    
    if package_json.exists() {
        // Check if package.json contains Angular dependencies
        if let Ok(content) = std::fs::read_to_string(package_json) {
            return content.contains("\"@angular/core\"") || content.contains("'@angular/core'");
        }
    }
    
    false
}
