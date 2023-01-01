// This file is part of "xrlib"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

#[derive(Debug, Clone)]
pub struct Buffer<T> {
    data: *mut T,
    size: i32,
    /// Number of bytes of data currently stored in the connection buffer.
    count: i32,
    /// Pointer to the end of the buffer
    end: *mut T,
}

impl<T> Buffer<T> {

}
