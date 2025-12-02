//! spust lib

#![warn(clippy::all, rust_2018_idioms)]
// #![deny(
//     missing_docs,
//     clippy::all,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc,
//     clippy::cargo
// )]
// #![warn(clippy::multiple_crate_versions)]

mod api;
mod errors;
mod server;
mod spust;

pub use spust::SpustArgs;
