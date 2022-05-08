#![allow(dead_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, warn(rustdoc::missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]

//! mockd
//!
//! # Build the request and verify
//!
//! Provide mock data that conforms to the expected format as test inputs.
//!
//! Providing randomly generated data is a best practice.
//!
//! ```rust
//! use mockd::image;
//!
//!     let data = image::url(500, 500); // url: https://picsum.photos/500/500
//! ```

#[cfg(feature = "address")]
pub mod address;
pub mod animal;
pub mod beer;
pub mod bool_rand;
pub mod color;
#[cfg(feature = "company")]
pub mod company;
#[cfg(feature = "contact")]
pub mod contact;
pub mod currency;
pub(crate) mod data;
pub mod datetime;
pub mod file;
#[cfg(feature = "generator")]
pub mod generator;
#[cfg(feature = "hacker")]
pub mod hacker;
#[cfg(feature = "hipster")]
pub mod hipster;
pub mod image;
#[cfg(feature = "internet")]
pub mod internet;
#[cfg(feature = "job")]
pub mod job;
pub mod language;
pub mod log_level;
#[cfg(feature = "misc")]
pub(crate) mod misc;
#[cfg(feature = "name")]
pub mod name;
pub mod password;
#[cfg(feature = "payment")]
pub mod payment;
#[cfg(feature = "person")]
pub mod person;
pub mod status_code;
pub(crate) mod testify;
pub mod unique;
pub mod user_agent;
#[cfg(feature = "vehicle")]
pub mod vehicle;
#[cfg(feature = "words")]
pub mod words;
