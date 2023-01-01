// This file is part of "xrlib"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

mod buffer;
pub use buffer::Buffer;

mod connection;
pub use connection::Connection;

mod display;
pub use display::Display;

pub mod x;

/// Errors for the `Result` structure.
/// Note that they are not the packet errors !
#[derive(Debug, Clone)]
pub enum ResultError {
    InvalidDisplayName,
    InvalidDisplayNumber,
    InvalidScreenNumber,
    String(String),
}

pub type Result<T> = std::result::Result<T, ResultError>;
