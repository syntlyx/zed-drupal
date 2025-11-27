mod detection;
mod language_servers;

use language_servers::drupal_lsp::DrupalLspManager;
use language_servers::intelephense::IntelephenseManager;

use serde_json::Value;
use zed_extension_api::{self as zed, Extension, LanguageServerId, Result, Worktree};

pub struct DrupalExtension {
    intelephense: Option<IntelephenseManager>,
    drupal_lsp: Option<DrupalLspManager>,
    is_drupal: bool,
}

impl Extension for DrupalExtension {
    fn new() -> Self {
        Self {
            intelephense: None,
            drupal_lsp: None,
            is_drupal: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        if !self.is_drupal && !detection::is_drupal_project(worktree) {
            return Err("Not a Drupal project".to_string());
        }
        self.is_drupal = true;

        match language_server_id.as_ref() {
            "intelephense" => self
                .intelephense
                .get_or_insert(IntelephenseManager::new(language_server_id.clone()))
                .command(worktree),
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
        if !self.is_drupal && !detection::is_drupal_project(worktree) {
            return Ok(None);
        }
        self.is_drupal = true;

        let root = detection::detect_drupal_root(worktree);

        match language_server_id.as_ref() {
            "intelephense" => Ok(Some(
                self.intelephense
                    .get_or_insert(IntelephenseManager::new(language_server_id.clone()))
                    .build_options(&root),
            )),
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
