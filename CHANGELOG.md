# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->

## [Unreleased]
- SimpleLogger init error handling

### Library
- The returned error in the Result is from now on anyhow::Error and not String.

## [0.2.0] - 2022-01-23
### Added
#### CLI
- create default config, if not exits
- when the image uploaded, send a notification (can be turn off in config)
- shell completions

#### Library
- change OpenSSL to RusTLS
- move api request to fn send_api_request

### Fixed
* api rate limit (error decoding response body: invalid value: integer \`200\`, expected i8 at line 1 column 140)

## [0.1.0] - 2022-01-23
### CLI
- commands
    - credits
    - delete
    - info
    - upload
- toml config parser

### Library
- image info
- rate limit
- image info
- delete image
- upload image

<!-- next-url -->
[Unreleased]: https://github.com/MedzikUser/imgurs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/MedzikUser/imgurs/commits/v0.2.0
[0.1.0]: https://github.com/MedzikUser/imgurs/commits/v0.1.0
