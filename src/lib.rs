#![allow(unused)]

use zed_extension_api::{self as zed, Command, Extension, LanguageServerId, Result, Worktree};

struct FountainExtension;

impl Extension for FountainExtension {
    fn new() -> Self {
        FountainExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Command> {
        Ok(Command::new("fountain-lsp"))
    }
}

zed::register_extension!(FountainExtension);