use std::str;
use std::ops::Deref;

use bytes::Bytes;

use clear::Clear;

/// Thin wrapper around `Bytes` which guarantees that bytes are valid UTF-8 string.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Chars(Bytes);

impl Chars {
    /// New empty object.
    pub fn new() -> Chars {
        Chars(Bytes::new())
    }

    /// Try convert from `Bytes`
    pub fn from_bytes(bytes: Bytes) -> Result<Chars, str::Utf8Error> {
        str::from_utf8(&bytes)?;

        Ok(Chars(bytes))
    }

    /// Len in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> From<&'a str> for Chars {
    fn from(src: &'a str) -> Chars {
        Chars(Bytes::from(src))
    }
}

impl Default for Chars {
    fn default() -> Self {
        Chars::new()
    }
}

impl Deref for Chars {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(&self.0)
        }
    }
}

impl Clear for Chars {
    fn clear(&mut self) {
        self.0.clear();
    }
}
