# Blukey

A CLI to set your custom key combo to perform custom key actions.

## Installation
```bash
cargo install blukey
```

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

## Examples
1. Register a new shortcut:
```bash
sudo blukey key new
# Follow the prompts to press keys and enter command
```
2. Start the monitor:
```bash
sudo blukey start
```
3. List registered shortcuts:
```bash
blukey key list
```

## Configuration

Configuration is stored in ~/.blukey.json. The format is:
```json
{
  "DEMON": false,
  "KEYS": {
    "KEY_LEFTCTRL+KEY_C": "echo 'Ctrl+C pressed'"
  }
}
```

## License
MIT License - See LICENSE for details.
