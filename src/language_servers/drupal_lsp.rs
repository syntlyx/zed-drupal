use super::NpmLanguageServerManager;
use serde_json::Value;
use std::env;
use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree};

const SERVER_PATH: &str = "node_modules/drupal-lsp-server/out/server.js";
const PACKAGE_NAME: &str = "drupal-lsp-server";

pub struct DrupalLspManager {
    id: LanguageServerId,
    did_find_server: bool,
}

impl DrupalLspManager {
    pub fn new(id: LanguageServerId) -> Self {
        Self {
            id,
            did_find_server: false,
        }
    }

    pub fn command(&mut self, worktree: &Worktree) -> Result<Command> {
        if let Some(path) = worktree.which("drupal-lsp-server") {
            eprintln!(
                "[Drupal] Using globally installed drupal-lsp-server at: {}",
                path
            );
            return Ok(Command {
                command: path,
                args: vec!["--stdio".to_string()],
                env: Default::default(),
            });
        }

        eprintln!("[Drupal] Using local npm installation of drupal-lsp-server");
        let server_path = self.server_script_path()?;
        let full_path = env::current_dir()
            .unwrap()
            .join(&server_path)
            .to_string_lossy()
            .to_string();

        eprintln!("[Drupal] Starting drupal-lsp-server at: {}", full_path);

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![full_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }

    pub fn build_options(&self, _worktree: &Worktree, _root: &str) -> Value {
        serde_json::json!({
            "drupal-lsp-server": {
                "enable": true,
                "phpcs": {
                    "enable": true
                }
            }
        })
    }
}

impl NpmLanguageServerManager for DrupalLspManager {
    fn language_server_id(&self) -> &LanguageServerId {
        &self.id
    }

    fn package_name(&self) -> &str {
        PACKAGE_NAME
    }

    fn server_path(&self) -> &str {
        SERVER_PATH
    }

    fn did_find_server(&self) -> bool {
        self.did_find_server
    }

    fn set_did_find_server(&mut self, value: bool) {
        self.did_find_server = value;
    }
}
