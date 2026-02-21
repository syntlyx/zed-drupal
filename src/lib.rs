mod detection;
mod language_servers;

use detection::DrupalProject;
use language_servers::drupal_lsp::DrupalLspManager;

use serde_json::Value;
use zed_extension_api::{self as zed, Extension, LanguageServerId, Result, Worktree};

pub struct DrupalExtension {
    drupal_lsp: Option<DrupalLspManager>,
    /// Cached detection result. `None` before first detection attempt.
    /// After detection: `Some(Some(project))` for Drupal projects, `Some(None)` for non-Drupal.
    drupal: Option<Option<DrupalProject>>,
}

impl DrupalExtension {
    /// Returns the cached Drupal project, running detection on first call.
    fn project(&mut self, worktree: &Worktree) -> Option<&DrupalProject> {
        if self.drupal.is_none() {
            self.drupal = Some(detection::detect(worktree));
        }
        self.drupal.as_ref().and_then(|d| d.as_ref())
    }
}

impl Extension for DrupalExtension {
    fn new() -> Self {
        Self {
            drupal_lsp: None,
            drupal: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        if self.project(worktree).is_none() {
            return Err("Not a Drupal project".to_string());
        }

        match language_server_id.as_ref() {
            "drupal-lsp-server" => self
                .drupal_lsp
                .get_or_insert(DrupalLspManager::new(language_server_id.clone()))
                .command(worktree),
            _ => Err(format!("Unknown LSP: {}", language_server_id)),
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let Some(project) = self.project(worktree) else {
            return Ok(None);
        };

        let root = project.drupal_root.clone();

        match language_server_id.as_ref() {
            "drupal-lsp-server" => Ok(Some(
                self.drupal_lsp
                    .get_or_insert(DrupalLspManager::new(language_server_id.clone()))
                    .build_options(worktree, &root),
            )),
            _ => Ok(None),
        }
    }
}

zed_extension_api::register_extension!(DrupalExtension);
