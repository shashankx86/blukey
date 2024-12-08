# Blukey

A CLI to set your custom key combo to perform custom key actions.

## Installation

```bash
curl -s https://raw.githubusercontent.com/shashankx86/blukey/refs/heads/main/install.sh | bash
```

Alternatively, install using Cargo:
```bash
cargo install blukey
```
Note: After installing with Cargo, you might need to add Cargo's binary path to your PATH environment variable to use with sudo. Typically, the Cargo bin path is ~/.cargo/bin.

## Usage
```bash
blukey <COMMAND>

Commands:
  start         Start the keyboard monitor
  stop          Stop the daemon
  key           Manage key combinations
  config-path   Show config file location

Key Commands:
  key new       Register a new key combination
  key list      List all registered key combinations
```
### Register a New Shortcut
```bash
sudo blukey key new
```
When you run `sudo blukey key new`, you will be prompted:
```bash
Press the key combination (press Enter when done):
```
Press your desired key combination. After releasing the keys, you'll be asked:
```bash
Enter command to execute for '<KEY_COMBINATION>':
```
Enter the command you want to execute when the key combination is pressed.

## Examples

### Register a new shortcut:
```bash
sudo blukey key new
# Follow the prompts to press keys and enter command
```
Start the monitor:
```bash
sudo blukey start
```
List registered shortcuts:
```bash
blukey key list
```

## Configuration

Configuration is stored in `~/.blukey.json`. The default configuration is:
```json
{
  "DAEMON": true,
  "SUDOLOCK": true,
  "KEYS": {}
}
```

### Configuration Fields

- DAEMON: (boolean) If true, blukey will run as a background daemon.
- SUDOLOCK: (boolean) If true, commands will be executed as the non-root user who invoked sudo. This adds an extra layer of security by preventing commands from running with root privileges.
- KEYS: (object) A mapping of key combinations to commands.

## Example:
```json
{
  "DAEMON": true,
  "SUDOLOCK": true,
  "KEYS": {
    "KEY_LEFTCTRL+KEY_C": "echo 'Ctrl+C pressed'"
  }
}
```

## License

MIT License - See LICENSE for details.
