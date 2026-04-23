mod language_servers;

use zed_extension_api::{
    self as zed, CodeLabel, DebugAdapterBinary, DebugConfig, DebugRequest, DebugScenario,
    DebugTaskDefinition, LanguageServerId, Result, StartDebuggingRequestArguments,
    StartDebuggingRequestArgumentsRequest, Worktree,
    lsp::{Completion, Symbol},
    serde_json::{self, Value, json},
};

use crate::language_servers::{Dexter, ElixirLs, Expert, Lexical, NextLs};

struct ElixirExtension {
    expert: Option<Expert>,
    elixir_ls: Option<ElixirLs>,
    dexter: Option<Dexter>,
    next_ls: Option<NextLs>,
    lexical: Option<Lexical>,
}

impl zed::Extension for ElixirExtension {
    fn new() -> Self {
        Self {
            expert: None,
            elixir_ls: None,
            dexter: None,
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
            Dexter::LANGUAGE_SERVER_ID => self
                .dexter
                .get_or_insert_with(Dexter::new)
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
            Dexter::LANGUAGE_SERVER_ID => self
                .dexter
                .get_or_insert_with(Dexter::new)
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
            Dexter::LANGUAGE_SERVER_ID => self
                .dexter
                .get_or_insert_with(Dexter::new)
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
            Dexter::LANGUAGE_SERVER_ID => self.dexter.as_ref()?.label_for_completion(completion),
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
            Dexter::LANGUAGE_SERVER_ID => self.dexter.as_ref()?.label_for_symbol(symbol),
            NextLs::LANGUAGE_SERVER_ID => self.next_ls.as_ref()?.label_for_symbol(symbol),
            Lexical::LANGUAGE_SERVER_ID => self.lexical.as_ref()?.label_for_symbol(symbol),
            _ => None,
        }
    }

    fn get_dap_binary(
        &mut self,
        _adapter_name: String,
        config: DebugTaskDefinition,
        user_provided_debug_adapter_path: Option<String>,
        worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String> {
        let binary_path = if let Some(path) = user_provided_debug_adapter_path {
            path
        } else {
            self.elixir_ls
                .get_or_insert_with(ElixirLs::new)
                .get_debug_adapter_path(worktree)?
        };

        let request = serde_json::from_str::<Value>(&config.config)
            .ok()
            .and_then(|v| {
                v.get("request")
                    .and_then(|r| r.as_str())
                    .map(str::to_string)
            })
            .unwrap_or_else(|| "launch".to_string());

        let request_kind = if request == "attach" {
            StartDebuggingRequestArgumentsRequest::Attach
        } else {
            StartDebuggingRequestArgumentsRequest::Launch
        };

        Ok(DebugAdapterBinary {
            command: Some(binary_path),
            arguments: vec![],
            envs: vec![],
            cwd: None,
            connection: None,
            request_args: StartDebuggingRequestArguments {
                configuration: config.config,
                request: request_kind,
            },
        })
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        config: Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest, String> {
        match config.get("request").and_then(|v| v.as_str()) {
            Some("attach") => Ok(StartDebuggingRequestArgumentsRequest::Attach),
            _ => Ok(StartDebuggingRequestArgumentsRequest::Launch),
        }
    }

    fn dap_config_to_scenario(&mut self, config: DebugConfig) -> Result<DebugScenario, String> {
        let adapter_config = match config.request {
            DebugRequest::Launch(launch) => {
                let env: serde_json::Map<String, Value> = launch
                    .envs
                    .into_iter()
                    .map(|(k, v)| (k, Value::String(v)))
                    .collect();

                let mut cfg = json!({
                    "type": "mix_task",
                    "request": "launch",
                    "task": launch.program,
                    "taskArgs": launch.args,
                    "env": env,
                });

                if let Some(cwd) = launch.cwd {
                    cfg["projectDir"] = Value::String(cwd);
                }

                cfg
            }
            DebugRequest::Attach(_) => json!({
                "type": "mix_task",
                "request": "attach",
            }),
        };

        Ok(DebugScenario {
            label: config.label,
            adapter: config.adapter,
            build: None,
            config: adapter_config.to_string(),
            tcp_connection: None,
        })
    }
}

zed::register_extension!(ElixirExtension);
