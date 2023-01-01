// This file is part of "xrlib"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use std::fs::File;
use crate::Buffer;
use crate::Display;

/// Enables a connection to the X window system.
#[derive(Debug)]
pub struct Connection {
    /// File descriptor
    fd: File,
    buffer: Buffer<u8>,
    /// Pointer to connection
    pc: *const u8,
}

impl Connection {
    /// Connects to X window system.
    pub fn connect(display: Display) -> Self {
        todo!()
    }

    pub fn disconnect(&mut self) {
        todo!()
    }
}
