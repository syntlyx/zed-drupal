use zed_extension_api as zed;

pub fn is_drupal_project(worktree: &zed::Worktree) -> bool {
    // Check for composer.json with drupal/core dependency
    if let Some(content) = worktree.read_text_file("composer.json").ok() {
        if content.contains("drupal/core") {
            eprintln!("[Drupal] Detected via composer.json with drupal/core");
            return true;
        }
    }

    // Check for Drupal core file in root
    if worktree.read_text_file("core/lib/Drupal.php").is_ok() {
        eprintln!("[Drupal] Detected via core/lib/Drupal.php in root");
        return true;
    }

    // Check for Drupal in standard subdirectories
    let core_locations = ["web/core/lib/Drupal.php", "docroot/core/lib/Drupal.php"];

    for location in core_locations {
        if worktree.read_text_file(location).is_ok() {
            eprintln!("[Drupal] Detected via {}", location);
            return true;
        }
    }

    eprintln!("[Drupal] Not detected in {}", worktree.root_path().as_str());
    false
}

pub fn detect_drupal_root(worktree: &zed::Worktree) -> String {
    let candidates = [
        ("web", "web/core/lib/Drupal.php"),
        ("docroot", "docroot/core/lib/Drupal.php"),
        (".", "core/lib/Drupal.php"),
    ];

    for (root, drupal_php) in &candidates {
        if worktree.read_text_file(drupal_php).is_ok() {
            eprintln!("[Drupal] Root directory: {}", root);
            return root.to_string();
        }
    }

    eprintln!("[Drupal] Root directory not found, defaulting to 'web'");
    "web".to_string()
}
