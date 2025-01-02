use codespan_reporting::{
    diagnostic::Diagnostic,
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
        Chars,
    },
};
use colored::Colorize;
use std::path::{Path, PathBuf};
use tungsten_symbols::SymbolTable;

use anyhow::Result;
use tungsten_utils::guess_host_target_triple;

pub mod error_builders;

fn get_name(path: &Path) -> Result<&str> {
    let file_name = path.file_name().ok_or_else(|| {
        anyhow::format_err!("cannot get file name from path {:?}", path.as_os_str())
    })?;

    file_name.to_str().ok_or_else(|| {
        anyhow::format_err!(
            "cannot create file name with a non-unicode name: {:?}",
            file_name
        )
    })
}

#[derive(Debug, Clone)]
pub struct CompilerContext<'a> {
    pub symbols: SymbolTable,
    pub current_scope: Option<SymbolTable>,

    file: SimpleFile<&'a str, &'a str>,

    file_name: &'a str,
    artifact_dir: &'a Path,
    source_code: &'a str,
    errors: Vec<Diagnostic<()>>,
    /// Architecture
    target_architecture: String,
    optimization_level: u8,
    file_path: &'a Path,
}

impl<'a> CompilerContext<'a> {
    pub fn new(file_path: &'a Path, source_code: &'a str, artifact_dir: &'a Path) -> Self {
        let file_name = get_name(file_path).expect("failed to get name of path");

        Self {
            file_path,
            file: SimpleFile::new(file_name, source_code),
            file_name,
            source_code,
            artifact_dir,
            current_scope: None,
            target_architecture: guess_host_target_triple(),
            symbols: SymbolTable::new(None),
            errors: Vec::new(),
            optimization_level: 0,
        }
    }

    pub fn set_target_triple(&mut self, target_triple: String) -> &mut Self {
        self.target_architecture = target_triple;
        self
    }

    pub fn set_opt_level(&mut self, opt_level: u8) -> &mut Self {
        self.optimization_level = opt_level;
        self
    }

    pub fn add_error(&mut self, diag: Diagnostic<()>) {
        self.errors.push(diag);
    }

    // pub fn add_error(&mut self, err: &str) -> &mut Self {
    //     self.errors.push(err.to_string());
    //     self
    // }

    pub fn emit_errors(&mut self) {
        if self.errors.is_empty() {
            return;
        }

        let mut writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config {
            start_context_lines: 4,
            end_context_lines: 4,
            chars: Chars::ascii(),
            display_style: term::DisplayStyle::Rich,
            ..Default::default()
        };

        for error in self.errors.clone() {
            term::emit(&mut writer, &config, &self.file, &error).unwrap();
        }
    }

    pub fn source(&self) -> String {
        self.source_code.to_string()
    }

    pub fn name(&self) -> String {
        self.file_name.to_string()
    }

    pub fn path(&self) -> PathBuf {
        self.file_path.to_path_buf()
    }

    pub fn artifact_path(&self) -> PathBuf {
        self.artifact_dir.to_path_buf()
    }
}
