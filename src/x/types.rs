// This file is part of "xrlib"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Definitions for the X protocol types.
//! https://www.x.org/releases/X11R7.5/doc/x11proto/proto.pdf (page 5)

// TYPES ///////////////////////////////////////////////////////////////////////

pub type Window = i32;
pub type Pixmap = i32;
pub type Cursor = i32;
pub type Font = i32;
pub type GContext = i32;
pub type Atom = i32;
pub type VisualID = i32;
pub type Value = i32;
pub type Timestamp = u32;
pub type KeySym = i32;
pub type Keycode = u8;
pub type Button = u8;
pub type ColorMap = i32;

// ENUMERATIONS ////////////////////////////////////////////////////////////////

/// Either a `Window` or a `Pixmap`.
pub enum Drawable {
    Window(Window),
    Pixmap(Pixmap),
}

/// Either a `Font` or a `GContext`.
pub enum Fontable {
    Font(Font),
    GContext(GContext),
}

pub enum BitGravity {
    Forget, 
    Static, 
    NorthWest, 
    North, 
    NorthEast, 
    West, 
    Center,
    East, 
    SouthWest, 
    South, 
    SouthEast
}

pub enum WinGravity {
    Unmap, 
    Static, 
    NorthWest, 
    North, 
    NorthEast, 
    West, 
    Center,
    East, 
    SouthWest, 
    South, 
    SouthEast
}

pub enum Event {
    KeyPress, 
    KeyRelease, 
    OwnerGrabButton, 
    ButtonPress,
    ButtonRelease, 
    EnterWindow, 
    LeaveWindow, 
    PointerMotion,
    PointerMotionHint, 
    Button1Motion, 
    Button2Motion,
    Button3Motion, 
    Button4Motion, 
    Button5Motion, 
    ButtonMotion,
    Exposure, 
    VisibilityChange, 
    StructureNotify, 
    ResizeRedirect,
    SubstructureNotify, 
    SubstructureRedirect, 
    FocusChange,
    PropertyChange, 
    ColormapChange, 
    KeymapState
}

pub enum PointerEvent {
    ButtonPress, 
    ButtonRelease, 
    EnterWindow, 
    LeaveWindow,
    PointerMotion, 
    PointerMotionHint, 
    Button1Motion,
    Button2Motion, 
    Button3Motion, 
    Button4Motion, 
    Button5Motion,
    ButtonMotion, 
    KeymapState
}

pub enum DeviceEvent {
    KeyPress, 
    KeyRelease, 
    ButtonPress, 
    ButtonRelease,
    PointerMotion, 
    Button1Motion, 
    Button2Motion, 
    Button3Motion,
    Button4Motion, 
    Button5Motion, 
    ButtonMotion
}

pub enum KeyMask {
    Shift,
    Lock,
    Control,
    Mod1,
    Mod2,
    Mod3,
    Mod4,
    Mod5,
}

pub enum ButMask {
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
}

pub enum KeyButMask {
    KeyMask(KeyMask),
    ButMask(ButMask),
}

pub enum HostFamily {
    Internet,
    InternetV6,
    ServerInterpreted,
    DECnet,
    Chaos,
}

// STRUCTURES //////////////////////////////////////////////////////////////////

pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub struct Rectangle {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

pub struct Arc {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub angle1: u16,
    pub angle2: u16,
}

pub struct Host {
    pub family: HostFamily,
    pub address: [i8], 
}
