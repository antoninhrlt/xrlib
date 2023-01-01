// This file is part of "xrlib"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use std::env;
use crate::ResultError;

/// A valid display format looks like one of these : 
/// - `:x` 
/// - `:x.y` 
/// - `id:x` 
/// - `id:x.y`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Display(Option<String>, i32, i32);

impl Display {
    /// The default way to define the display.
    /// 
    /// If the `display` string is empty, we use the environment `DISPLAY` 
    /// variable. If it's not defined, use the default display value.
    pub fn new(display: &str) -> crate::Result<Self> {
        if display.is_empty() {
            return Ok(
                match Self::env() {
                    Ok(env_display) => env_display,
                    Err(_) => Self::default()
                }
            )
        }

        let mut parts = display.split(|c| { c == ':' || c == '.' });
        
        // The string just before ":" can be empty, but ":" remains mandatory
        let hostname: Option<String> = match parts.next() {
            Some(s) => if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
            None => return Err(ResultError::InvalidDisplayName),
        };
        
        // Mandatory
        let display_n: i32 = match parts.next() {
            Some(s) => match s.parse::<i32>() {
                Ok(n) => n,
                Err(_) => return Err(ResultError::InvalidDisplayNumber),
            },
            None => return Err(ResultError::InvalidDisplayName)
        };

        // Not mandatory, 0 by default if not found
        let screen_n: i32 = match parts.next() {
            Some(s) => match s.parse::<i32>() {
                Ok(n) => n,
                Err(_) => return Err(ResultError::InvalidScreenNumber),
            },
            None => 0,
        };

        Ok(Self(hostname, display_n, screen_n))
    }

    /// Retrieves the display from the environment variables.
    pub fn env() -> crate::Result<Self> {
        match env::var("DISPLAY") {
            Ok(s) => Display::new(s.as_str()),
            Err(e) => Err(ResultError::String(e.to_string())),
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Display::new(":0").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_display_name() {
        assert_eq!(Display(None, 1, 0), Display::new(":1").unwrap());
        assert_eq!(Display(Some("localhost".to_string()), 1, 0), Display::new("localhost:1.0").unwrap());
        assert_eq!(Display(None, 0, 2), Display::new(":0.2").unwrap());
        Display::new(":0.").unwrap(); // panics
    }
}
