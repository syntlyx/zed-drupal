# Drupal Extension for Zed

Enhanced Drupal development support for [Zed](https://zed.dev) editor.

## Features

- **Automatic Drupal project detection** via composer.json
- **PHP code standards integration** with PHPCS/PHPCBF
- **Intelephense configuration** with Drupal-specific stubs
- **Drupal LSP server** for advanced features:
  - Service autocompletion in YAML files
  - Go to definition for services and classes
  - Real-time validation of .services.yml files

## Installation

Currently, the extension must be installed manually. We plan to publish it to the Zed extensions marketplace in the future.

### Manual Installation

1. **Clone the repository:**
  ```bash
  git clone https://github.com/syntlyx/zed-drupal.git
  ```

2. **Install as dev extension in Zed:**
   - Press `Cmd + Shift + X` (or `Ctrl + Shift + X` on Linux/Windows)
   - Click "Install Dev Extension"
   - Select the cloned repository folder
   - Zed will build and install the extension automatically

## Configuration

Extension works out of the box. For advanced configuration, check Zed's language server settings.

## License

[MIT](LICENSE)
