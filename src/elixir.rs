mod language_servers;

use zed_extension_api::{
    self as zed, CodeLabel, LanguageServerId, Result, Worktree,
    lsp::{Completion, Symbol},
    serde_json::Value,
};

use crate::language_servers::{ElixirLs, Expert, Lexical, NextLs};

struct ElixirExtension {
    expert: Option<Expert>,
    elixir_ls: Option<ElixirLs>,
    next_ls: Option<NextLs>,
    lexical: Option<Lexical>,
}

impl zed::Extension for ElixirExtension {
    fn new() -> Self {
        Self {
            expert: None,
            elixir_ls: None,
            next_ls: None,
            lexical: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            Expert::LANGUAGE_SERVER_ID => self
                .expert
                .get_or_insert_with(Expert::new)
                .language_server_command(language_server_id, worktree),
            ElixirLs::LANGUAGE_SERVER_ID => self
                .elixir_ls
                .get_or_insert_with(ElixirLs::new)
                .language_server_command(language_server_id, worktree),
            NextLs::LANGUAGE_SERVER_ID => self
                .next_ls
                .get_or_insert_with(NextLs::new)
                .language_server_command(language_server_id, worktree),
            Lexical::LANGUAGE_SERVER_ID => self
                .lexical
                .get_or_insert_with(Lexical::new)
                .language_server_command(language_server_id, worktree),
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        match language_server_id.as_ref() {
            Expert::LANGUAGE_SERVER_ID => self
                .expert
                .get_or_insert_with(Expert::new)
                .language_server_initialization_options(worktree),
            ElixirLs::LANGUAGE_SERVER_ID => self
                .elixir_ls
                .get_or_insert_with(ElixirLs::new)
                .language_server_initialization_options(worktree),
            NextLs::LANGUAGE_SERVER_ID => self
                .next_ls
                .get_or_insert_with(NextLs::new)
                .language_server_initialization_options(worktree),
            Lexical::LANGUAGE_SERVER_ID => self
                .lexical
                .get_or_insert_with(Lexical::new)
                .language_server_initialization_options(worktree),
            _ => Ok(None),
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        match language_server_id.as_ref() {
            Expert::LANGUAGE_SERVER_ID => self
                .expert
                .get_or_insert_with(Expert::new)
                .language_server_workspace_configuration(worktree),
            ElixirLs::LANGUAGE_SERVER_ID => self
                .elixir_ls
                .get_or_insert_with(ElixirLs::new)
                .language_server_workspace_configuration(worktree),
            NextLs::LANGUAGE_SERVER_ID => self
                .next_ls
                .get_or_insert_with(NextLs::new)
                .language_server_workspace_configuration(worktree),
            Lexical::LANGUAGE_SERVER_ID => self
                .lexical
                .get_or_insert_with(Lexical::new)
                .language_server_workspace_configuration(worktree),
            _ => Ok(None),
        }
    }

    fn label_for_completion(
        &self,
        language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        match language_server_id.as_ref() {
            Expert::LANGUAGE_SERVER_ID => self.expert.as_ref()?.label_for_completion(completion),
            ElixirLs::LANGUAGE_SERVER_ID => {
                self.elixir_ls.as_ref()?.label_for_completion(completion)
            }
            NextLs::LANGUAGE_SERVER_ID => self.next_ls.as_ref()?.label_for_completion(completion),
            Lexical::LANGUAGE_SERVER_ID => self.lexical.as_ref()?.label_for_completion(completion),
            _ => None,
        }
    }

    fn label_for_symbol(
        &self,
        language_server_id: &LanguageServerId,
        symbol: Symbol,
    ) -> Option<CodeLabel> {
        match language_server_id.as_ref() {
            Expert::LANGUAGE_SERVER_ID => self.expert.as_ref()?.label_for_symbol(symbol),
            ElixirLs::LANGUAGE_SERVER_ID => self.elixir_ls.as_ref()?.label_for_symbol(symbol),
            NextLs::LANGUAGE_SERVER_ID => self.next_ls.as_ref()?.label_for_symbol(symbol),
            Lexical::LANGUAGE_SERVER_ID => self.lexical.as_ref()?.label_for_symbol(symbol),
            _ => None,
        }
    }
}

zed::register_extension!(ElixirExtension);
