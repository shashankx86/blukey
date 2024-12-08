# Changelog

## [0.0.1-rc0] 

### Features
- Initial release candidate
- Keyboard event monitoring system
- Custom shortcut registration
- Command execution on key combinations
- Daemon mode support
- CLI interface with commands:
  - `start`: Start keyboard monitor
  - `stop`: Stop daemon
  - `key new`: Register new shortcuts
  - `key list`: List registered shortcuts
  - `config-path`: Show config location
- JSON configuration file support
- Root privilege handling
- Cross-platform Linux builds

## [0.0.2-rc0] 

### Features
- Added SUDOLOCK configuration option for safer command execution
- Improved keyboard device detection with better error handling

### Improvements
- Better terminal handling during key registration
- More detailed error messages for command execution
- Structured CLI interface 

### Bug Fixes
- Fixed privilege escalation issues with command execution
- Improved error handling for config file operations
- Better cleanup when stopping daemon