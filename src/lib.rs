//! [![github]](https://github.com/M3DZIK/imgurs)
//! [![crates-io]](https://crates.io/crates/imgurs)
//! [![docs-rs]](https://docs.rs/imgurs)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! This crate is an unofficial implementation of the [Imgur API](https://imgur.com) in Rust.
//!
//! # Installation
//!
//! ## Requirements
//! - Rust 1.58 (earlier versions are not tested (only the latest stable version is tested!))
//! - Network connection
//!
//! ## Importing
//! The driver is available on [crates.io](https://crates.io/crates/imgurs). To use the driver in
//! your application, simply add it to your project's `Cargo.toml`.
//! ```toml
//! [dependencies]
//! imgurs = "0.11.3"
//! ```
//!
//! # Example Usage
//!
//! ## Create new ImgurClient
//! ```
//! use imgurs::ImgurClient;
//!
//! let client = ImgurClient::new("client_id");
//! ```
//!
//! ## Image Upload
//! ```no_run
//! use imgurs::ImgurClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = ImgurClient::new("client_id");
//!
//!     // From URL
//!     let info = client
//!         .upload_image("https://i.imgur.com/lFaGr1x.png")
//!         .await
//!         .unwrap();
//!     println!("{:?}", info);
//!
//!     // From File
//!     let info = client.upload_image("path/to/file.png").await.unwrap();
//!     println!("{:?}", info);
//! }
//! ```
//!
//! ## Delete Image
//! ```no_run
//! use imgurs::ImgurClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = ImgurClient::new("client_id");
//!
//!     client.delete_image("delete_hash").await.unwrap(); // delete hash
//! }
//! ```
//!
//! ## Get Image Info
//! ```no_run
//! use imgurs::ImgurClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = ImgurClient::new("client_id");
//!
//!     let info = client.image_info("lFaGr1x").await.unwrap(); // image id
//!
//!     println!("{:?}", info);
//! }
//! ```
//!
//! ## Get Client RateLimit
//! ```no_run
//! use imgurs::ImgurClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = ImgurClient::new("client_id");
//!
//!     let info = client.rate_limit().await.unwrap();
//!
//!     println!("{:?}", info);
//! }
//! ```

mod error;
pub use error::*;

#[cfg(feature = "imgur")]
mod imgur;

#[cfg(feature = "imgur")]
pub use imgur::*;
