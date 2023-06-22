# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->

## [Unreleased]

## [0.11.2] - 2023-06-22
- Updated dependencies
- Fix deprecation warning of base64 crate.

## [0.11.1] - 2022-12-11
### Fixed
- `album_title` can be null, `account_id` can be null, string or number #92, thanks to @NotNorom

### Changed
- Updated dependencies

## [0.11.0] - 2022-11-07
### Added
- Added `with_http_client` method to ImgurClient #87, thanks to @NotNorom

### Changed
- Updated crate arboard to v3
- Updated crate simple_logger to v4

## [0.10.0] - 2022-10-01
- add configuration for tls (rustls-tls or native-tls)

## [0.9.1] - 2022-09-22
- delete debug info from cli in release build

## [0.9.0] - 2022-09-05
- moved cli to other crate
- added get_album function

## [0.8.1] - 2022-06-18
- fix tests
- add missing doc
- use `serde` instead of `serde_derive`

## [0.8.0] - 2022-06-13
- add custom `Error` type
- move api requests to `requests/` mod
- comment code

## [0.7.4] - 2022-05-18
### HOTFIX
- fixed built on macos and windows

## [0.7.3] - 2022-05-18
### Library
- add code comments and tests
- change `String` to `&str` in ImgurClient functions

### Other
- bump deps
- use `anyhow::Result<...>` instead `Result<..., Error>`

## [0.7.2] - 2022-04-05
### HotFix
- fix upload image from file

## [0.7.1] - 2022-04-04
- fix build on what is not linux

## [0.7.0] - 2022-04-03
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
[Unreleased]: https://github.com/MedzikUser/imgurs/compare/v0.11.1...HEAD
[0.11.1]: https://github.com/MedzikUser/imgurs/commits/v0.11.1
[0.11.0]: https://github.com/MedzikUser/imgurs/commits/v0.11.0
[0.10.0]: https://github.com/MedzikUser/imgurs/commits/v0.10.0
[0.9.1]: https://github.com/MedzikUser/imgurs/commits/v0.9.1
[0.9.0]: https://github.com/MedzikUser/imgurs/commits/v0.9.0
[0.8.1]: https://github.com/MedzikUser/imgurs/commits/v0.8.1
[0.8.0]: https://github.com/MedzikUser/imgurs/commits/v0.8.0
[0.7.4]: https://github.com/MedzikUser/imgurs/commits/v0.7.4
[0.7.3]: https://github.com/MedzikUser/imgurs/commits/v0.7.3
[0.7.2]: https://github.com/MedzikUser/imgurs/commits/v0.7.2
[0.7.1]: https://github.com/MedzikUser/imgurs/commits/v0.7.1
[0.7.0]: https://github.com/MedzikUser/imgurs/commits/v0.7.0
[0.6.0]: https://github.com/MedzikUser/imgurs/commits/v0.6.0
[0.5.1]: https://github.com/MedzikUser/imgurs/commits/v0.5.1
[0.5.0]: https://github.com/MedzikUser/imgurs/commits/v0.5.0
[0.4.0]: https://github.com/MedzikUser/imgurs/commits/v0.4.0
[0.3.0]: https://github.com/MedzikUser/imgurs/commits/v0.3.0
[0.2.0]: https://github.com/MedzikUser/imgurs/commits/v0.2.0
[0.1.0]: https://github.com/MedzikUser/imgurs/commits/v0.1.0
