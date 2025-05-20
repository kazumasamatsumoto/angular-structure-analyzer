// src/models.rs
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct ProjectStructure {
    pub root: DirectoryNode,
}

#[derive(Debug, Serialize)]
pub struct DirectoryNode {
    pub name: String,
    pub path: PathBuf,
    pub directories: Vec<DirectoryNode>,
    pub files: Vec<FileNode>,
}

#[derive(Debug, Serialize)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub file_type: FileType,
}

// Hash トレイトを追加
#[derive(Debug, Serialize, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum FileType {
    Component,
    Service,
    Module,
    Directive,
    Pipe,
    Template,
    Guard,
    Resolver,
    Model,
    Config,
    Style,
    Test,
    NgRxAction,
    NgRxReducer,
    NgRxEffect,
    NgRxSelector,
    NgRxOther,
    Other,
}

#[derive(Debug, Serialize)]
pub struct Component {
    pub name: String,
    pub selector: Option<String>,
    pub path: PathBuf,
    pub template_path: Option<PathBuf>,
    pub style_paths: Vec<PathBuf>,
    pub test_path: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
pub struct Service {
    pub name: String,
    pub path: PathBuf,
    pub injectable_scope: Option<String>,
    pub test_path: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
pub struct Module {
    pub name: String,
    pub path: PathBuf,
    pub declarations: Vec<String>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub providers: Vec<String>,
    pub bootstrap: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Dependency {
    pub source: PathBuf,
    pub target: String,
    pub import_type: ImportType,
}

// Hash トレイトを追加
#[derive(Debug, Serialize, PartialEq, Eq, Clone, Hash)]
pub enum ImportType {
    Module,
    Component,
    Service,
    Directive,
    Pipe,
    Guard,
    Resolver,
    Model,
    Other,
}

#[derive(Debug, Serialize)]
pub struct Route {
    pub path: String,
    pub component: Option<String>,
    pub children: Vec<Route>,
    pub lazy_module: Option<String>,
}
