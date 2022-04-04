//! This crate is an unofficial implementation of the [Imgur](https://imgur.com) API in Rust.
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
//! imgurs = "0.7.1"
//! ```
//!
//! # Example Usage
//!
//! ## Create new ImgurClient
//! ```
//! use imgurs::ImgurClient;
//!
//! let client = ImgurClient::new("client id");
//! ```
//!
//! ## Image Upload
//! ```
//! // From URL
//! let info = client.upload_image("https://cdn.magicuser.cf/lFaGr1x.png").await?;
//!
//! // From File
//! let info = client.upload_image("path/to/image.png").await?;
//! ```
//!
//! ## Delete Image
//! ```
//! client.delete_image("SuPeRsEcReTDeLeTeHaSh").await?; // delete hash
//! ```
//!
//! ## Get Image Info
//! ```
//! let info = client.image_info("lFaGr1x").await?; // image id
//!
//! println!("{:?}", info);
//! ```
//!
//! ## Get Client RateLimit
//! ```
//! let info = client.rate_limit.await?;
//!
//! println!("{:?}", info);
//! ```

mod api;

pub use api::*;
