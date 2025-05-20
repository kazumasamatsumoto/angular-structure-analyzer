// src/formatter/json.rs
use anyhow::Result;
use serde_json;

use crate::models::*;

pub fn output_structure(structure: &ProjectStructure) -> Result<()> {
    let json = serde_json::to_string_pretty(structure)?;
    println!("{}", json);
    Ok(())
}

pub fn output_components(components: &[Component]) -> Result<()> {
    let json = serde_json::to_string_pretty(components)?;
    println!("{}", json);
    Ok(())
}

pub fn output_services(services: &[Service]) -> Result<()> {
    let json = serde_json::to_string_pretty(services)?;
    println!("{}", json);
    Ok(())
}

pub fn output_modules(modules: &[Module]) -> Result<()> {
    let json = serde_json::to_string_pretty(modules)?;
    println!("{}", json);
    Ok(())
}

pub fn output_dependencies(dependencies: &[Dependency]) -> Result<()> {
    let json = serde_json::to_string_pretty(dependencies)?;
    println!("{}", json);
    Ok(())
}

pub fn output_routes(routes: &[Route]) -> Result<()> {
    let json = serde_json::to_string_pretty(routes)?;
    println!("{}", json);
    Ok(())
}
