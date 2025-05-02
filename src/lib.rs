#![allow(dead_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
// #![cfg_attr(docsrs, warn(rustdoc::missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]

//! mockd
//!
//! # Build the request and verify
//!
//! Provide mock data that conforms to the expected format as test inputs.
//!
//! Providing randomly generated data is a best practice.
//!
//! ## Examples
//!
//! ### Using default features
//!
//! ```rust
//! use mockd::file;
//!
//!     let data = file::mime_type(); // mime_type: text/x-fortran
//!     let data = file::extension(); // extension: aspx
//! ```
//!
//! ### Using image feature
//!
//! ```toml
//! [dependencies]
//! mockd = "0.4.47"
//! ```
//!
//! ```no_compile
//! use mockd::image;
//!
//!     let data = image::url(500, 500); // url: https://picsum.photos/500/500
//! ```
//!
//! # Features
//!
//! To access the related generators the feature must be enabled. The following
//! features are available:
//! * default - includes file feature only
//! * all - Enables all features
//! * address
//! * animal
//! * beer
//! * company
//! * contact
//! * currency
//! * datetime
//! * file
//! * generator
//! * hacker
//! * hipster
//! * image
//! * internet
//! * job
//! * language
//! * log-level
//! * name
//! * password
//! * payment
//! * person
//! * random-bool
//! * status-code
//! * unique
//! * user-agent
//! * vehicle
//! * words
//!

#[cfg(feature = "address")]
pub mod address;
#[cfg(feature = "animal")]
pub mod animal;
#[cfg(feature = "beer")]
pub mod beer;
#[cfg(feature = "random-bool")]
pub mod bool_rand;
#[cfg(feature = "colour")]
pub mod colour;
#[cfg(feature = "company")]
pub mod company;
#[cfg(feature = "contact")]
pub mod contact;
#[cfg(feature = "currency")]
pub mod currency;
#[cfg(feature = "datetime")]
pub mod datetime;
#[cfg(feature = "file")]
pub mod file;
#[cfg(feature = "generator")]
pub mod generator;
#[cfg(feature = "hacker")]
pub mod hacker;
#[cfg(feature = "hipster")]
pub mod hipster;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "internet")]
pub mod internet;
#[cfg(feature = "job")]
pub mod job;
#[cfg(feature = "language")]
pub mod language;
#[cfg(feature = "log-level")]
pub mod log_level;
#[cfg(feature = "misc")]
pub(crate) mod misc;
#[cfg(feature = "name")]
pub mod name;
#[cfg(feature = "password")]
pub mod password;
#[cfg(feature = "payment")]
pub mod payment;
#[cfg(feature = "person")]
pub mod person;
#[cfg(feature = "status-code")]
pub mod status_code;
#[cfg(feature = "unique")]
pub mod unique;
#[cfg(feature = "user-agent")]
pub mod user_agent;
#[cfg(feature = "vehicle")]
pub mod vehicle;
#[cfg(feature = "words")]
pub mod words;

#[cfg(test)]
pub(crate) mod testify;
