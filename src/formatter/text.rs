// src/formatter/text.rs
use anyhow::Result;
use colored::*;
use std::path::Path;
use crate::models::*;
use crate::models::FileType;
use crate::formatter::text::DirectoryNode;

pub fn output_structure(structure: &ProjectStructure) -> Result<()> {
    println!("\n{} Project Structure:", "STRUCTURE:".green().bold());
    print_directory(&structure.root, 0, &structure.root.path);
    println!();
    output_summary_from_structure(&structure.root);
    Ok(())
}

pub fn output_components(components: &[Component], detailed: bool) -> Result<()> {
    println!("\n{} Components ({}):", "COMPONENTS:".green().bold(), components.len());

    if components.is_empty() {
        println!("  No components found");
        return Ok(());
    }

    for component in components {
        println!("  {} ({})", component.name.yellow(), component.path.display());

        if detailed {
            if let Some(ref selector) = component.selector {
                println!("    Selector: {}", selector);
            }

            if let Some(ref template_path) = component.template_path {
                println!("    Template: {}", template_path.display());
            }

            if !component.style_paths.is_empty() {
                println!("    Styles:");
                for style_path in &component.style_paths {
                    println!("      {}", style_path.display());
                }
            }

            if let Some(ref test_path) = component.test_path {
                println!("    Test: {}", test_path.display());
            }

            println!();
        }
    }

    Ok(())
}

pub fn output_services(services: &[Service], detailed: bool) -> Result<()> {
    println!("\n{} Services ({}):", "SERVICES:".green().bold(), services.len());

    if services.is_empty() {
        println!("  No services found");
        return Ok(());
    }

    for service in services {
        println!("  {} ({})", service.name.yellow(), service.path.display());

        if detailed {
            if let Some(ref scope) = service.injectable_scope {
                println!("    Injectable scope: {}", scope);
            }

            if let Some(ref test_path) = service.test_path {
                println!("    Test: {}", test_path.display());
            }

            println!();
        }
    }

    Ok(())
}

pub fn output_modules(modules: &[Module], detailed: bool) -> Result<()> {
    println!("\n{} Modules ({}):", "MODULES:".green().bold(), modules.len());

    if modules.is_empty() {
        println!("  No modules found");
        return Ok(());
    }

    for module in modules {
        println!("  {} ({})", module.name.yellow(), module.path.display());

        if detailed {
            if !module.declarations.is_empty() {
                println!("    Declarations: {}", module.declarations.join(", "));
            }

            if !module.imports.is_empty() {
                println!("    Imports: {}", module.imports.join(", "));
            }

            if !module.exports.is_empty() {
                println!("    Exports: {}", module.exports.join(", "));
            }

            if !module.providers.is_empty() {
                println!("    Providers: {}", module.providers.join(", "));
            }

            if !module.bootstrap.is_empty() {
                println!("    Bootstrap: {}", module.bootstrap.join(", "));
            }

            println!();
        }
    }

    Ok(())
}

pub fn output_dependencies(dependencies: &[Dependency]) -> Result<()> {
    println!("\n{} Dependencies ({}):", "DEPENDENCIES:".green().bold(), dependencies.len());

    if dependencies.is_empty() {
        println!("  No dependencies found");
        return Ok(());
    }

    // Group dependencies by source file
    let mut grouped = std::collections::HashMap::new();

    for dep in dependencies {
        let source = dep.source.display().to_string();
        let entry = grouped.entry(source).or_insert_with(Vec::new);
        entry.push((dep.target.clone(), &dep.import_type));
    }

    // Sort sources
    let mut sources: Vec<_> = grouped.keys().collect();
    sources.sort();

    for source in sources {
        println!("  {}:", source);

        let deps = &grouped[source];
        for (target, import_type) in deps {
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

            println!("    {} -> {}", target, type_str);
        }

        println!();
    }

    Ok(())
}

pub fn output_routes(routes: &[Route]) -> Result<()> {
    println!("\n{} Routes ({}):", "ROUTES:".green().bold(), routes.len());

    if routes.is_empty() {
        println!("  No routes found");
        return Ok(());
    }

    for route in routes {
        print_route(route, 1);
    }

    Ok(())
}

fn print_directory(dir: &DirectoryNode, depth: usize, base_path: &Path) {
    let indent = "  ".repeat(depth);
    let name = if depth == 0 { dir.name.clone() } else { format!("{}/", dir.name) };

    println!("{}{}{}", indent, name.blue().bold(), "");

    // Print files
    for file in &dir.files {
        let file_type_indicator = match file.file_type {
            // —— Angular 本体 —— //
            FileType::Component => "C".magenta(), // コンポーネント
            FileType::Service => "S".green(), // サービス
            FileType::Module => "M".yellow(), // モジュール
            FileType::Directive => "D".cyan(), // ディレクティブ
            FileType::Pipe => "P".blue(), // パイプ
            FileType::Guard => "G".red(), // ガード
            FileType::Resolver => "R".bright_red(), // リゾルバ
            FileType::Model => "I".bright_blue(), // モデル／インタフェース
            FileType::Config => "CF".bright_yellow(), // 設定ファイル
            FileType::Style => "ST".bright_magenta(), // スタイル
            FileType::Test => "T".bright_green(), // テスト

            // —— HTML テンプレート —— //
            FileType::Template => "H".white(), // HTML ファイル

            // —— NgRx 細分類 —— //
            FileType::NgRxAction => "NGA".truecolor(255, 165, 0), // オレンジ系
            FileType::NgRxReducer => "NGR".truecolor(75, 0, 130), // インディゴ系
            FileType::NgRxEffect => "NGE".truecolor(60, 179, 113), // シーグリーン系
            FileType::NgRxSelector => "NGS".truecolor(220, 20, 60), // クリムゾン系
            FileType::NgRxOther => "NGO".bright_white(), // その他 NgRx

            // —— その他 —— //
            FileType::Other => "O".normal(),
        };

        println!("{}  [{}] {}", indent, file_type_indicator, file.name);
    }

    // Print subdirectories
    for subdir in &dir.directories {
        print_directory(subdir, depth + 1, base_path);
    }
}

fn print_route(route: &Route, depth: usize) {
    let indent = "  ".repeat(depth);
    let path = if route.path.is_empty() { "/" } else { &route.path };

    print!("{}{}", indent, path.green());

    if let Some(ref component) = route.component {
        print!(" -> {}", component.yellow());
    }

    if let Some(ref lazy_module) = route.lazy_module {
        print!(" (lazy: {})", lazy_module.cyan());
    }

    println!();

    for child in &route.children {
        print_route(child, depth + 1);
    }
}

fn collect_all_files(dir: &DirectoryNode, out: &mut Vec<FileType>) {
    for file in &dir.files {
        out.push(file.file_type.clone());
    }
    for sub in &dir.directories {
        collect_all_files(sub, out);
    }
}

fn output_summary_from_structure(root: &DirectoryNode) {
    let mut types = Vec::new();
    collect_all_files(root, &mut types);

    // カウント
    use std::collections::BTreeMap;
    let mut counts: BTreeMap<FileType, usize> = BTreeMap::new();
    for t in types {
        *counts.entry(t).or_insert(0) += 1;
    }

    // 出力
    println!("\n{} Summary:", "SUMMARY:".green().bold());
    for (file_type, cnt) in &counts {
        // FileType を文字列に変換するメソッドがあれば使ってください
        println!("  {:<15} {}", format!("{:?}", file_type), cnt);
    }
    let total: usize = counts.values().sum();
    println!("  {:<15} {}", "Total", total);
    println!();
}
