#![allow(unused)]

use std::path::PathBuf;
use zed_extension_api::{self as zed, Command, Extension, LanguageServerId, Result, Worktree};

struct FountainExtension;

fn find_fountain_lsp() -> String {
    if let Ok(path) = std::env::var("FOUNTAIN_LSP_PATH") {
        if !path.is_empty() {
            return path;
        }
    }

    let possible_paths = [
        "/usr/local/bin/fountain-lsp",
        "/opt/homebrew/bin/fountain-lsp",
        "~/.cargo/bin/fountain-lsp",
        "~/.local/bin/fountain-lsp",
    ];

    for path in possible_paths {
        if PathBuf::from(path).exists() {
            return path.to_string();
        }
    }

    "/usr/local/bin/fountain-lsp".to_string()
}

impl Extension for FountainExtension {
    fn new() -> Self {
        FountainExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Command> {
        Ok(Command::new(find_fountain_lsp()))
    }
}

zed::register_extension!(FountainExtension);