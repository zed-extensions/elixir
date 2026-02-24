use std::fs;

use zed::lsp::{Completion, CompletionKind, Symbol};
use zed::{CodeLabel, CodeLabelSpan, LanguageServerId};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, Result};

use crate::language_servers::util;

pub struct LexicalBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
}

pub struct Lexical {
    cached_binary_path: Option<String>,
}

impl Lexical {
    pub const LANGUAGE_SERVER_ID: &'static str = "lexical";

    pub fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    pub fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<LexicalBinary> {
        let binary_settings = LspSettings::for_worktree("lexical", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(LexicalBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which("lexical") {
            return Ok(LexicalBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(LexicalBinary {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = match zed::latest_github_release(
            "lexical-lsp/lexical",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            Ok(release) => release,
            Err(_) => {
                if let Some(path) = util::find_existing_binary("lexical/bin/start_lexical.sh") {
                    self.cached_binary_path = Some(path.clone());
                    return Ok(LexicalBinary {
                        path,
                        args: binary_args,
                    });
                }
                return Err("failed to download latest github release".to_string());
            }
        };

        let asset_name = format!(
            "{}-{version}.zip",
            Self::LANGUAGE_SERVER_ID,
            version = release.version
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("{}-{}", Self::LANGUAGE_SERVER_ID, release.version);
        let binary_path = format!("{version_dir}/lexical/bin/start_lexical.sh");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;
            zed::make_file_executable(&format!("{version_dir}/lexical/bin/debug_shell.sh"))?;
            zed::make_file_executable(&format!("{version_dir}/lexical/priv/port_wrapper.sh"))?;

            util::remove_outdated_versions(Self::LANGUAGE_SERVER_ID, &version_dir)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(LexicalBinary {
            path: binary_path,
            args: binary_args,
        })
    }

    pub fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        match completion.kind? {
            CompletionKind::Module
            | CompletionKind::Class
            | CompletionKind::Interface
            | CompletionKind::Struct => {
                let name = completion.label;
                let defmodule = "defmodule ";
                let code = format!("{defmodule}{name}");

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(
                        defmodule.len()..defmodule.len() + name.len(),
                    )],
                    filter_range: (0..name.len()).into(),
                })
            }
            CompletionKind::Function | CompletionKind::Constant => {
                let name = completion.label;
                let def = "def ";
                let code = format!("{def}{name}");

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(def.len()..def.len() + name.len())],
                    filter_range: (0..name.len()).into(),
                })
            }
            CompletionKind::Operator => {
                let name = completion.label;
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

    pub fn label_for_symbol(&self, _symbol: Symbol) -> Option<CodeLabel> {
        None
    }
}
