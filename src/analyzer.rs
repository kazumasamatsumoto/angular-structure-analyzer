// src/analyzer.rs
use anyhow::{ Context, Result };
use colored::*;
use indicatif::{ ProgressBar, ProgressStyle };
use regex::Regex;
use walkdir::{ DirEntry, WalkDir };
use std::fs;
use std::path::{ Path, PathBuf };

use crate::models::*;

pub struct Analyzer {
    root_path: PathBuf,
    include_tests: bool,
    include_styles: bool,
    include_node_modules: bool,
    max_depth: usize,
}

impl Analyzer {
    pub fn new(
        root_path: PathBuf,
        include_tests: bool,
        include_styles: bool,
        include_node_modules: bool,
        max_depth: usize
    ) -> Self {
        Self {
            root_path,
            include_tests,
            include_styles,
            include_node_modules,
            max_depth,
        }
    }

    pub fn analyze_structure(&mut self) -> Result<ProjectStructure> {
        println!("{} Analyzing project structure...", "INFO:".blue().bold());

        let root_name = self.root_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("root")
            .to_string();

        let mut root = DirectoryNode {
            name: root_name,
            path: self.root_path.clone(),
            directories: Vec::new(),
            files: Vec::new(),
        };

        self.scan_directory(&self.root_path, &mut root, 0)?;

        Ok(ProjectStructure { root })
    }

    pub fn analyze_components(&mut self) -> Result<Vec<Component>> {
        println!("{} Analyzing components...", "INFO:".blue().bold());

        let mut components = Vec::new();
        let progress = self.create_progress_bar("Scanning for components");

        // Find all component files
        for entry in self.walk_project_files() {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                if file_name.ends_with(".component.ts") && !file_name.ends_with(".spec.ts") {
                    let component = self.parse_component(path)?;
                    components.push(component);
                    progress.inc(1);
                }
            }
        }

        progress.finish_with_message(format!("Found {} components", components.len()));

        Ok(components)
    }

    pub fn analyze_services(&mut self) -> Result<Vec<Service>> {
        println!("{} Analyzing services...", "INFO:".blue().bold());

        let mut services = Vec::new();
        let progress = self.create_progress_bar("Scanning for services");

        // Find all service files
        for entry in self.walk_project_files() {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                if file_name.ends_with(".service.ts") && !file_name.ends_with(".spec.ts") {
                    let service = self.parse_service(path)?;
                    services.push(service);
                    progress.inc(1);
                }
            }
        }

        progress.finish_with_message(format!("Found {} services", services.len()));

        Ok(services)
    }

    pub fn analyze_modules(&mut self) -> Result<Vec<Module>> {
        println!("{} Analyzing modules...", "INFO:".blue().bold());

        let mut modules = Vec::new();
        let progress = self.create_progress_bar("Scanning for modules");

        // Find all module files
        for entry in self.walk_project_files() {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                if file_name.ends_with(".module.ts") {
                    let module = self.parse_module(path)?;
                    modules.push(module);
                    progress.inc(1);
                }
            }
        }

        progress.finish_with_message(format!("Found {} modules", modules.len()));

        Ok(modules)
    }

    pub fn analyze_dependencies(&mut self) -> Result<Vec<Dependency>> {
        println!("{} Analyzing dependencies...", "INFO:".blue().bold());

        let mut dependencies = Vec::new();
        let progress = self.create_progress_bar("Scanning for dependencies");

        // Find all TypeScript files
        for entry in self.walk_project_files() {
            let path = entry.path();
            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                if extension == "ts" {
                    let file_deps = self.parse_file_dependencies(path)?;
                    dependencies.extend(file_deps);
                    progress.inc(1);
                }
            }
        }

        progress.finish_with_message(format!("Found {} dependencies", dependencies.len()));

        Ok(dependencies)
    }

    pub fn analyze_routes(&mut self) -> Result<Vec<Route>> {
        println!("{} Analyzing routes...", "INFO:".blue().bold());

        let mut routes = Vec::new();
        let progress = self.create_progress_bar("Scanning for routing modules");

        // Find all routing module files
        for entry in self.walk_project_files() {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                if file_name.contains("routing") && file_name.ends_with(".module.ts") {
                    let module_routes = self.parse_routes(path)?;
                    routes.extend(module_routes);
                    progress.inc(1);
                }
            }
        }

        progress.finish_with_message(format!("Found {} routes", routes.len()));

        Ok(routes)
    }

    // Private helper methods

    fn scan_directory(
        &self,
        dir_path: &Path,
        parent_node: &mut DirectoryNode,
        depth: usize
    ) -> Result<()> {
        if depth >= self.max_depth {
            return Ok(());
        }

        // Skip excluded directories
        if
            !self.include_node_modules &&
            dir_path.file_name().and_then(|n| n.to_str()) == Some("node_modules")
        {
            return Ok(());
        }

        if let Some(name) = dir_path.file_name().and_then(|n| n.to_str()) {
            if name == ".angular" || name == ".vscode" || name == ".git" {
                return Ok(());
            }
        }

        let entries = fs
            ::read_dir(dir_path)
            .context(format!("Failed to read directory '{}'", dir_path.display()))?;

        for entry in entries {
            let entry = entry.context(format!("Failed to read entry in '{}'", dir_path.display()))?;
            let path = entry.path();

            if path.is_dir() {
                let dir_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let mut dir_node = DirectoryNode {
                    name: dir_name,
                    path: path.clone(),
                    directories: Vec::new(),
                    files: Vec::new(),
                };

                self.scan_directory(&path, &mut dir_node, depth + 1)?;

                // Only add the directory if it contains files or other directories
                if !dir_node.files.is_empty() || !dir_node.directories.is_empty() {
                    parent_node.directories.push(dir_node);
                }
            } else {
                if let Some(file_node) = self.create_file_node(&path)? {
                    parent_node.files.push(file_node);
                }
            }
        }

        // Sort directories and files by name
        parent_node.directories.sort_by(|a, b| a.name.cmp(&b.name));
        parent_node.files.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(())
    }

    fn create_file_node(&self, path: &Path) -> Result<Option<FileNode>> {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Skip excluded files
        if !self.include_tests && file_name.contains(".spec.") {
            return Ok(None);
        }

        if !self.include_styles && is_style_file(path) {
            return Ok(None);
        }

        let file_type = determine_file_type(path);

        Ok(
            Some(FileNode {
                name: file_name,
                path: path.to_path_buf(),
                file_type,
            })
        )
    }

    fn walk_project_files(&self) -> impl Iterator<Item = DirEntry> {
        WalkDir::new(&self.root_path)
            .follow_links(true)
            .into_iter()
            .filter_entry(move |e| self.is_included_entry(e))
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
    }

    fn is_included_entry(&self, entry: &DirEntry) -> bool {
        let path = entry.path();

        // Skip node_modules if not included
        if !self.include_node_modules {
            let is_node_modules = path.components().any(|c| {
                if let std::path::Component::Normal(os_str) = c {
                    match os_str.to_str() {
                        Some("node_modules") | Some(".angular") | Some(".vscode") | Some(".git") => true,
                        _ => false,
                    }
                } else {
                    false
                }
            });

            if is_node_modules {
                return false;
            }
        }

        // Skip test files if not included
        if !self.include_tests {
            if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                if file_name.contains(".spec.") {
                    return false;
                }
            }
        }

        // Skip style files if not included
        if !self.include_styles && is_style_file(path) {
            return false;
        }

        true
    }

    fn parse_component(&self, path: &Path) -> Result<Component> {
        let content = fs
            ::read_to_string(path)
            .context(format!("Failed to read file '{}'", path.display()))?;

        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .replace(".component", "");

        let component_name = extract_class_name(&content).unwrap_or_else(||
            format!("{}Component", pascal_case(&file_stem))
        );

        let selector = extract_selector(&content);

        let parent_dir = path.parent().unwrap_or(Path::new(""));

        // Find related files
        let template_path = find_related_file(parent_dir, &file_stem, "component.html");
        let test_path = find_related_file(parent_dir, &file_stem, "component.spec.ts");

        // Find style files
        let mut style_paths = Vec::new();
        for extension in &["css", "scss", "sass", "less"] {
            if
                let Some(style_path) = find_related_file(
                    parent_dir,
                    &file_stem,
                    &format!("component.{}", extension)
                )
            {
                style_paths.push(style_path);
            }
        }

        Ok(Component {
            name: component_name,
            selector,
            path: path.to_path_buf(),
            template_path,
            style_paths,
            test_path,
        })
    }

    fn parse_service(&self, path: &Path) -> Result<Service> {
        let content = fs
            ::read_to_string(path)
            .context(format!("Failed to read file '{}'", path.display()))?;

        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .replace(".service", "");

        let service_name = extract_class_name(&content).unwrap_or_else(||
            format!("{}Service", pascal_case(&file_stem))
        );

        let injectable_scope = extract_injectable_scope(&content);

        let parent_dir = path.parent().unwrap_or(Path::new(""));
        let test_path = find_related_file(parent_dir, &file_stem, "service.spec.ts");

        Ok(Service {
            name: service_name,
            path: path.to_path_buf(),
            injectable_scope,
            test_path,
        })
    }

    fn parse_module(&self, path: &Path) -> Result<Module> {
        let content = fs
            ::read_to_string(path)
            .context(format!("Failed to read file '{}'", path.display()))?;

        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .replace(".module", "");

        let module_name = extract_class_name(&content).unwrap_or_else(||
            format!("{}Module", pascal_case(&file_stem))
        );

        // Extract module metadata
        let declarations = extract_array_property(&content, "declarations");
        let imports = extract_array_property(&content, "imports");
        let exports = extract_array_property(&content, "exports");
        let providers = extract_array_property(&content, "providers");
        let bootstrap = extract_array_property(&content, "bootstrap");

        Ok(Module {
            name: module_name,
            path: path.to_path_buf(),
            declarations,
            imports,
            exports,
            providers,
            bootstrap,
        })
    }

    fn parse_file_dependencies(&self, path: &Path) -> Result<Vec<Dependency>> {
        let content = fs
            ::read_to_string(path)
            .context(format!("Failed to read file '{}'", path.display()))?;

        let mut dependencies = Vec::new();

        // Find import statements
        let re = Regex::new(r#"import\s+\{([^}]+)\}\s+from\s+['"]([^'"]+)['"]"#).unwrap();

        for cap in re.captures_iter(&content) {
            let imports = cap[1]
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            let source = cap[2].trim();

            for import in imports {
                let import_type = determine_import_type(import);

                dependencies.push(Dependency {
                    source: path.to_path_buf(),
                    target: source.to_string(),
                    import_type,
                });
            }
        }

        Ok(dependencies)
    }

    fn parse_routes(&self, path: &Path) -> Result<Vec<Route>> {
        let content = fs
            ::read_to_string(path)
            .context(format!("Failed to read file '{}'", path.display()))?;

        let mut routes = Vec::new();

        // Find routes array
        if let Some(routes_content) = extract_routes_array(&content) {
            // Parse individual routes
            let route_objects = parse_route_objects(&routes_content);

            for route_obj in route_objects {
                if let Some(route) = parse_route_object(&route_obj) {
                    routes.push(route);
                }
            }
        }

        Ok(routes)
    }

    fn create_progress_bar(&self, message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.blue} {msg} ({elapsed})")
                .expect("Failed to set progress style")
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(std::time::Duration::from_millis(80));
        pb
    }
}

// Helper functions

fn determine_file_type(path: &Path) -> FileType {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    if file_name.ends_with(".action.ts") {
        return FileType::NgRxAction;
    } else if file_name.ends_with(".html") {
        return FileType::Template;
    } else if file_name.ends_with(".reducer.ts") {
        return FileType::NgRxReducer;
    } else if file_name.ends_with(".effects.ts") {
        return FileType::NgRxEffect;
    } else if file_name.ends_with(".selector.ts") {
        return FileType::NgRxSelector;
    } else if file_name.ends_with(".ngrx.ts") {
        return FileType::NgRxOther;
    } else if file_name.ends_with(".component.ts") {
        FileType::Component
    } else if file_name.ends_with(".service.ts") {
        FileType::Service
    } else if file_name.ends_with(".module.ts") {
        FileType::Module
    } else if file_name.ends_with(".directive.ts") {
        FileType::Directive
    } else if file_name.ends_with(".pipe.ts") {
        FileType::Pipe
    } else if file_name.ends_with(".guard.ts") {
        FileType::Guard
    } else if file_name.ends_with(".resolver.ts") {
        FileType::Resolver
    } else if file_name.ends_with(".model.ts") || file_name.ends_with(".interface.ts") {
        FileType::Model
    } else if file_name == "tsconfig.json" || file_name == "angular.json" {
        FileType::Config
    } else if file_name.ends_with(".spec.ts") {
        FileType::Test
    } else if is_style_file(path) {
        FileType::Style
    } else {
        FileType::Other
    }
}

fn is_style_file(path: &Path) -> bool {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    matches!(extension, "css" | "scss" | "sass" | "less")
}

fn extract_class_name(content: &str) -> Option<String> {
    let re = Regex::new(r"export\s+class\s+([A-Za-z0-9_]+)").ok()?;
    re.captures(content).map(|cap| cap[1].to_string())
}

fn extract_selector(content: &str) -> Option<String> {
    let re = Regex::new(r#"selector\s*:\s*['"]([^'"]+)['"]"#).ok()?;
    re.captures(content).map(|cap| cap[1].to_string())
}

fn extract_injectable_scope(content: &str) -> Option<String> {
    let re = Regex::new(r#"providedIn\s*:\s*['"]([^'"]+)['"]"#).ok()?;
    re.captures(content).map(|cap| cap[1].to_string())
}

fn extract_array_property(content: &str, property_name: &str) -> Vec<String> {
    let mut result = Vec::new();

    let re_pattern = format!(r"{}\s*:\s*\[(.*?)\]", property_name);
    let re = Regex::new(&re_pattern).unwrap();

    if let Some(cap) = re.captures(content) {
        let items_str = &cap[1];

        // Split by commas, ignoring commas inside nested brackets
        let mut items = Vec::new();
        let mut start = 0;
        let mut bracket_count = 0;

        for (i, c) in items_str.char_indices() {
            match c {
                '{' | '[' | '(' => {
                    bracket_count += 1;
                }
                '}' | ']' | ')' => {
                    bracket_count -= 1;
                }
                ',' if bracket_count == 0 => {
                    items.push(&items_str[start..i]);
                    start = i + 1;
                }
                _ => {}
            }
        }

        // Add the last item
        if start < items_str.len() {
            items.push(&items_str[start..]);
        }

        for item in items {
            let item = item.trim();
            if !item.is_empty() {
                // Extract class names from the items
                if let Some(class_name) = extract_item_name(item) {
                    result.push(class_name);
                }
            }
        }
    }

    result
}

fn extract_item_name(item: &str) -> Option<String> {
    // Try to extract class name (SimpleClass)
    let re = Regex::new(r"([A-Za-z0-9_]+)").ok()?;
    re.captures(item).map(|cap| cap[1].to_string())
}

fn determine_import_type(import: &str) -> ImportType {
    if import.ends_with("Component") {
        ImportType::Component
    } else if import.ends_with("Service") {
        ImportType::Service
    } else if import.ends_with("Module") {
        ImportType::Module
    } else if import.ends_with("Directive") {
        ImportType::Directive
    } else if import.ends_with("Pipe") {
        ImportType::Pipe
    } else if import.ends_with("Guard") {
        ImportType::Guard
    } else if import.ends_with("Resolver") {
        ImportType::Resolver
    } else if import.ends_with("Model") || import.ends_with("Interface") {
        ImportType::Model
    } else {
        ImportType::Other
    }
}

fn extract_routes_array(content: &str) -> Option<String> {
    // Find the routes array
    let re = Regex::new(r"const\s+routes\s*:\s*Routes\s*=\s*\[([\s\S]*?)\];").ok()?;
    re.captures(content).map(|cap| cap[1].trim().to_string())
}

fn parse_route_objects(routes_content: &str) -> Vec<String> {
    let mut routes = Vec::new();
    let mut start = 0;
    let mut bracket_count = 0;
    let mut in_object = false;

    for (i, c) in routes_content.char_indices() {
        match c {
            '{' => {
                if bracket_count == 0 {
                    start = i;
                    in_object = true;
                }
                bracket_count += 1;
            }
            '}' => {
                bracket_count -= 1;
                if bracket_count == 0 && in_object {
                    routes.push(routes_content[start..=i].to_string());
                    in_object = false;
                }
            }
            _ => {}
        }
    }

    routes
}

fn parse_route_object(route_obj: &str) -> Option<Route> {
    // Extract path
    let path_re = Regex::new(r#"path\s*:\s*['"]([^'"]+)['"]"#).ok()?;
    let path = path_re
        .captures(route_obj)
        .map(|cap| cap[1].to_string())
        .unwrap_or_else(|| "".to_string());

    // Extract component
    let component_re = Regex::new(r"component\s*:\s*([A-Za-z0-9_]+)").ok()?;
    let component = component_re.captures(route_obj).map(|cap| cap[1].to_string());

    // Extract lazy loading module
    let lazy_re = Regex::new(r#"loadChildren\s*:\s*['"](.*?)['"]"#).ok()?;
    let lazy_module = lazy_re.captures(route_obj).map(|cap| cap[1].to_string());

    // Extract children routes
    let children_re = Regex::new(r"children\s*:\s*\[([\s\S]*?)\]").ok()?;
    let children = if let Some(cap) = children_re.captures(route_obj) {
        let children_content = cap[1].trim();
        let child_objects = parse_route_objects(children_content);

        child_objects
            .iter()
            .filter_map(|child_obj| parse_route_object(child_obj))
            .collect()
    } else {
        Vec::new()
    };

    Some(Route {
        path,
        component,
        children,
        lazy_module,
    })
}

fn find_related_file(dir: &Path, base_name: &str, extension: &str) -> Option<PathBuf> {
    let target_file = format!("{}.{}", base_name, extension);
    let path = dir.join(&target_file);

    if path.exists() {
        Some(path)
    } else {
        None
    }
}

fn pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '-' || c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}
