// This file is part of "xrlib"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Definitions for the X protocol errors.
//! https://www.x.org/releases/X11R7.5/doc/x11proto/proto.pdf (page 7)

pub enum Error {
    Access,
    Alloc,
    Atom,
    ColorMap,
    Cursor,
    Drawable,
    Font,
    GContext,
    IDChoice,
    Implementation,
    Length,
    Match,
    Name,
    Pixmap,
    Request,
    Value,
    Window,
}
