# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->

## [Unreleased]
### Added
#### CLI
- create default config, if not exits
- when the image uploaded, send a notification (can be turn off in config)
- added completions

#### Library
- change OpenSSL to RusTLS

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
[Unreleased]: https://github.com/MedzikUser/imgurs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/MedzikUser/imgurs/commits/v0.1.0
