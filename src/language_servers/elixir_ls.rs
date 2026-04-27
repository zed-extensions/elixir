use std::fs;

use zed_extension_api::{
    self as zed, CodeLabel, CodeLabelSpan, LanguageServerId, Result, Worktree,
    lsp::{Completion, CompletionKind, Symbol, SymbolKind},
    serde_json::{Value, json},
};

use crate::language_servers::{config, util};

struct ElixirLsBinary {
    path: String,
    args: Vec<String>,
}

pub struct ElixirLs {
    cached_binary_path: Option<String>,
}

impl ElixirLs {
    pub const LANGUAGE_SERVER_ID: &'static str = "elixir-ls";

    pub fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: binary.path,
            args: binary.args,
            env: Default::default(),
        })
    }

    pub fn get_debug_adapter_path(&mut self, worktree: &Worktree) -> Result<String> {
        let (_, debug_adapter_path) = self.download_elixir_ls(None, worktree)?;
        Ok(debug_adapter_path)
    }

    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<ElixirLsBinary> {
        let binary_settings = config::get_binary_settings(Self::LANGUAGE_SERVER_ID, worktree);
        let args = config::get_binary_args(&binary_settings).unwrap_or_default();
        let (language_server_path, _) =
            self.download_elixir_ls(Some(language_server_id), worktree)?;
        Ok(ElixirLsBinary {
            path: language_server_path,
            args,
        })
    }

    fn download_elixir_ls(
        &mut self,
        language_server_id: Option<&LanguageServerId>,
        worktree: &Worktree,
    ) -> Result<(String, String)> {
        let (platform, _arch) = zed::current_platform();
        let extension = match platform {
            zed::Os::Mac | zed::Os::Linux => "sh",
            zed::Os::Windows => "bat",
        };

        let binary_name = format!("language_server.{extension}");
        let debug_adapter_name = format!("debug_adapter.{extension}");
        let launch_script = format!("launch.{extension}");
        let binary_settings = config::get_binary_settings(Self::LANGUAGE_SERVER_ID, worktree);

        if let Some(language_server_path) = config::get_binary_path(&binary_settings) {
            let debug_adapter_path = std::path::Path::new(&language_server_path)
                .parent()
                .map(|p| p.join(&debug_adapter_name).to_string_lossy().to_string())
                .ok_or_else(|| "failed to determine debug adapter path".to_string())?;
            return Ok((language_server_path, debug_adapter_path));
        }

        if let Some(language_server_path) = worktree.which(Self::LANGUAGE_SERVER_ID) {
            let debug_adapter_path = std::path::Path::new(&language_server_path)
                .parent()
                .map(|p| p.join(&debug_adapter_name).to_string_lossy().to_string())
                .ok_or_else(|| "failed to determine debug adapter path".to_string())?;
            return Ok((language_server_path, debug_adapter_path));
        }

        if let Some(language_server_path) = &self.cached_binary_path
            && fs::metadata(language_server_path).is_ok_and(|stat| stat.is_file())
        {
            let debug_adapter_path = std::path::Path::new(language_server_path)
                .parent()
                .map(|p| p.join(&debug_adapter_name).to_string_lossy().to_string())
                .ok_or_else(|| "failed to determine debug adapter path".to_string())?;
            return Ok((language_server_path.clone(), debug_adapter_path));
        }

        if let Some(id) = language_server_id {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::CheckingForUpdate,
            );
        }

        let release = match zed::latest_github_release(
            "elixir-lsp/elixir-ls",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            Ok(release) => release,
            Err(_) => {
                if let Some(ls_path) =
                    util::find_existing_binary(Self::LANGUAGE_SERVER_ID, &binary_name)
                {
                    let absolute_language_server_path = fs::canonicalize(format!("./{ls_path}"))
                        .map(|p| p.to_string_lossy().to_string())
                        .map_err(|e| format!("failed to get absolute language server path: {e}"))?;
                    let absolute_debug_adapter_path =
                        std::path::Path::new(&absolute_language_server_path)
                            .parent()
                            .map(|p| p.join(&debug_adapter_name).to_string_lossy().to_string())
                            .ok_or_else(|| "failed to determine debug adapter path".to_string())?;
                    self.cached_binary_path = Some(absolute_language_server_path.clone());
                    return Ok((absolute_language_server_path, absolute_debug_adapter_path));
                }
                return Err("failed to download latest github release".to_string());
            }
        };

        let asset_name = format!(
            "{}-{version}.zip",
            Self::LANGUAGE_SERVER_ID,
            version = release.version,
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("{}-{}", Self::LANGUAGE_SERVER_ID, release.version);
        let binary_path = format!("{version_dir}/{binary_name}");
        let launch_path = format!("{version_dir}/{launch_script}");
        let debug_path = format!("{version_dir}/{debug_adapter_name}");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            if let Some(id) = language_server_id {
                zed::set_language_server_installation_status(
                    id,
                    &zed::LanguageServerInstallationStatus::Downloading,
                );
            }

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;
            zed::make_file_executable(&launch_path)?;
            zed::make_file_executable(&debug_path)?;

            util::remove_outdated_versions(Self::LANGUAGE_SERVER_ID, &version_dir)?;
        }

        let absolute_language_server_path = fs::canonicalize(format!("./{binary_path}"))
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|e| format!("failed to get absolute language server path: {e}"))?;
        let absolute_debug_adapter_path = fs::canonicalize(format!("./{debug_path}"))
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|e| format!("failed to get absolute debug adapter path: {e}"))?;

        self.cached_binary_path = Some(absolute_language_server_path.clone());
        Ok((absolute_language_server_path, absolute_debug_adapter_path))
    }

    pub fn language_server_initialization_options(
        &mut self,
        _worktree: &Worktree,
    ) -> Result<Option<Value>> {
        Ok(None)
    }

    pub fn language_server_workspace_configuration(
        &mut self,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let settings = config::get_workspace_configuration(Self::LANGUAGE_SERVER_ID, worktree)
            .unwrap_or_default();

        Ok(Some(json!({
            "elixirLS": settings
        })))
    }

    pub fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        let name = &completion.label;
        let detail = completion
            .detail
            .filter(|detail| detail != "alias")
            .map(|detail| format!(": {detail}"))
            .unwrap_or("".to_string());

        let detail_span = CodeLabelSpan::literal(detail, Some("comment.unused".to_string()));

        match completion.kind? {
            CompletionKind::Module | CompletionKind::Class | CompletionKind::Struct => {
                let defmodule = "defmodule ";
                let alias = completion
                    .label_details
                    .and_then(|details| details.description)
                    .filter(|description| description.starts_with("alias"))
                    .map(|description| format!(" ({description})"))
                    .unwrap_or("".to_string());

                let code = format!("{defmodule}{name}{alias}");
                let name_start = defmodule.len();
                let name_end = name_start + name.len();

                Some(CodeLabel {
                    code,
                    spans: vec![
                        CodeLabelSpan::code_range(name_start..name_end),
                        detail_span,
                        CodeLabelSpan::code_range(name_end..(name_end + alias.len())),
                    ],
                    filter_range: (0..name.len()).into(),
                })
            }
            CompletionKind::Interface => Some(CodeLabel {
                code: name.to_string(),
                spans: vec![CodeLabelSpan::code_range(0..name.len()), detail_span],
                filter_range: (0..name.len()).into(),
            }),
            CompletionKind::Field => Some(CodeLabel {
                code: name.to_string(),
                spans: vec![
                    CodeLabelSpan::literal(name, Some("function".to_string())),
                    detail_span,
                ],
                filter_range: (0..name.len()).into(),
            }),
            CompletionKind::Function | CompletionKind::Constant => {
                let detail = completion
                    .label_details
                    .clone()
                    .and_then(|details| details.detail)
                    .unwrap_or("".to_string());

                let description = completion
                    .label_details
                    .clone()
                    .and_then(|details| details.description)
                    .map(|description| format!(" ({description})"))
                    .unwrap_or("".to_string());

                let def = "def ";
                let code = format!("{def}{name}{detail}{description}");

                let name_start = def.len();
                let name_end = name_start + name.len();
                let detail_end = name_end + detail.len();
                let description_end = detail_end + description.len();

                Some(CodeLabel {
                    code,
                    spans: vec![
                        CodeLabelSpan::code_range(name_start..name_end),
                        CodeLabelSpan::code_range(name_end..detail_end),
                        CodeLabelSpan::code_range(detail_end..description_end),
                    ],
                    filter_range: (0..name.len()).into(),
                })
            }
            CompletionKind::Operator => {
                let def_a = "def a ";
                let code = format!("{def_a}{name} b");

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(
                        def_a.len()..def_a.len() + name.len(),
                    )],
                    filter_range: (0..name.len()).into(),
                })
            }
            _ => None,
        }
    }

    pub fn label_for_symbol(&self, symbol: Symbol) -> Option<CodeLabel> {
        let name = &symbol.name;

        let (code, filter_range, display_range) = match symbol.kind {
            SymbolKind::Module | SymbolKind::Class | SymbolKind::Interface | SymbolKind::Struct => {
                let defmodule = "defmodule ";
                let code = format!("{defmodule}{name}");
                let filter_range = 0..name.len();
                let display_range = defmodule.len()..defmodule.len() + name.len();
                (code, filter_range, display_range)
            }
            SymbolKind::Function | SymbolKind::Constant => {
                let def = "def ";
                let code = format!("{def}{name}");
                let filter_range = 0..name.len();
                let display_range = def.len()..def.len() + name.len();
                (code, filter_range, display_range)
            }
            _ => return None,
        };

        Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(display_range)],
            filter_range: filter_range.into(),
            code,
        })
    }
}
