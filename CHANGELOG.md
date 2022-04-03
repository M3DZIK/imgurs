# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->

## [Unreleased]

## [0.7.0] - 2022-04-04
### CLI
- completions: changed type from String to Shell
- removed `&` from `cli.commands` (line 54 in [parse.rs](./src/cli/parse.rs))

### Library
- removed `.map_err(anyhow::Error::new)` when function returns error

### Added
- commands in the code
- api functions to `impl` in `ImgurClient`
- documentation (example usage)

### Breaking Changes
- lib: moved everything to the main package with api submodules (before `imgurs::api::ImgurClient`, after `imgurs::api::ImgurClient`)

## [0.6.0] - 2022-03-14
### CLI
- webhook: added url in title
- cli: change image domain to your own (set in config)
- if the configuration file cannot be open, ask the user whether to overwrite the file instead of overwriting it without asking
- logger: set `max_level_debug` in debug binary

## [0.5.1] - 2022-03-08
### Cli
- change webhook to discord-webhook (to use rustls)

## [0.5.0] - 2022-03-07
### CLI
- clipboard: add support for xclip and termux
- webhook: send webhook to discord if image uploaded ([example](https://i.imgur.com/CPpHEec.png))

### Library
- if body length is greater than 30, return message `body is too length`

## [0.4.0] - 2022-02-27
### CLI
- update logger
- added clipboard
- added manpage
- added completion for elvish
- if failed to upload image send notify with error message

### Library
- added Clone derive
- if body length is > 30 return body is too length

## [0.3.0] - 2022-01-28
### CLI
- SimpleLogger init error handling
- better panic
- panic instead of send log error
- add url validate

### Library
- The returned error in the Result is from now on anyhow::Error and not String.
- Do not exit program if send_api_request error
- rename ImgurHandle -> ImgurClient

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
- api rate limit (error decoding response body: invalid value: integer \`200\`, expected i8 at line 1 column 140)

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
[Unreleased]: https://github.com/MedzikUser/imgurs/compare/v0.7.0...HEAD
[0.7.0]: https://github.com/MedzikUser/imgurs/commits/v0.7.0
[0.6.0]: https://github.com/MedzikUser/imgurs/commits/v0.6.0
[0.5.1]: https://github.com/MedzikUser/imgurs/commits/v0.5.1
[0.5.0]: https://github.com/MedzikUser/imgurs/commits/v0.5.0
[0.4.0]: https://github.com/MedzikUser/imgurs/commits/v0.4.0
[0.3.0]: https://github.com/MedzikUser/imgurs/commits/v0.3.0
[0.2.0]: https://github.com/MedzikUser/imgurs/commits/v0.2.0
[0.1.0]: https://github.com/MedzikUser/imgurs/commits/v0.1.0
