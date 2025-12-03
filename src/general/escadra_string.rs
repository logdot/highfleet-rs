//! Defines a variable length string frequently used within Highfleet called an EscadraString.

use libc;
use serde::{Deserialize, Serialize};
use std::fmt;

/// An union that stores either a raw 16 char string or a pointer to a raw char string.
#[derive(Clone, Copy)]
union CharPointer {
    /// A raw 16 char string.
    /// Must be null terminated.
    chars: [u8; 16],
    /// A pointer to a raw char string.
    /// Must be null terminated.
    pointer: *mut u8,
}

/// An escadra string is a variable length string.
/// If the max-length of the escadra string (without the null terminator) exceeds 15 the string is stored as a pointer to memory.
/// Otherwise, the string is stored within the struct itself.
///
/// The string should always be null terminated.
/// The `max_length` is 15 by default.
#[repr(C)]
#[derive(Deserialize, Serialize)]
#[serde(from = "String")]
#[serde(into = "String")]
pub struct EscadraString {
    /// The \[u8;16\] char or the pointer to the char, depending on if max_length is either 15 or more.
    string: CharPointer,
    /// The length of the currently stored string
    length: u64,
    /// The maximum length of the string.
    /// By default it's 15 (15 for the chars + 1 for the null pointer completely filling up the default 16 char buffer).
    max_length: u64,
}

impl fmt::Debug for EscadraString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self.get_string();

        f.debug_struct("EscadraString")
            .field("string", &string)
            .field("length", &self.length)
            .field("max_length", &self.max_length)
            .finish()
    }
}

impl EscadraString {
    /// Creates an empty Escadra string.
    pub fn new() -> Self {
        Self {
            string: CharPointer { chars: [b'\0'; 16] },
            length: 0,
            max_length: 15,
        }
    }

    /// Writes the given string into the `EscadraString`.
    pub fn set_string(&mut self, string: &String) {
        if self.max_length > 15 || string.len() > 15 {
            unsafe {
                if self.max_length > 15 {
                    libc::free(self.string.pointer as _);
                }

                let mut size: usize = (self.max_length + 1).try_into().unwrap();
                while size <= string.len() {
                    size *= 2;
                }
                let size = size;

                self.string.pointer = libc::malloc(size) as *mut u8;
                libc::memcpy(self.string.pointer as _, string.as_ptr() as _, string.len());

                *self.string.pointer.add(string.len()) = b'\0';

                self.max_length = (size - 1) as u64;
            }
        } else {
            let mut buffer = [0u8; 16];
            buffer[..string.len()].copy_from_slice(string.as_bytes());
            self.string.chars = buffer;
        }

        self.length = string.len() as _;
    }

    /// Returns the string inside of the `EscadraString`.
    pub fn get_string(&self) -> &str {
        if self.max_length > 15 {
            unsafe {
                let buf: &[u8] = core::slice::from_raw_parts(self.string.pointer, self.length as _);
                return std::str::from_utf8(buf).unwrap();
            }
        }

        unsafe { std::str::from_utf8(&self.string.chars[0..self.length as _]).unwrap() }
    }
}

impl From<String> for EscadraString {
    fn from(value: String) -> Self {
        let mut es = EscadraString::new();
        es.set_string(&value);
        es
    }
}

impl From<EscadraString> for String {
    fn from(val: EscadraString) -> Self {
        val.get_string().to_string()
    }
}

impl From<&str> for EscadraString {
    fn from(value: &str) -> Self {
        let mut es = EscadraString::new();
        es.set_string(&value.to_string());
        es
    }
}

impl PartialEq for EscadraString {
    fn eq(&self, other: &Self) -> bool {
        self.get_string() == other.get_string()
    }
}

impl Eq for EscadraString {}

impl std::cmp::PartialOrd for EscadraString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for EscadraString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_string().cmp(other.get_string())
    }
}

impl std::hash::Hash for EscadraString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_string().hash(state);
    }
}

impl Clone for EscadraString {
    fn clone(&self) -> Self {
        let string = self.get_string().to_string();
        let mut es = EscadraString::new();
        es.set_string(&string);
        es
    }
}

impl Default for EscadraString {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for EscadraString {
    fn drop(&mut self) {
        if self.max_length > 15 {
            unsafe {
                libc::free(self.string.pointer as _);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_string_then_read_below_16_chars() {
        let mut es = EscadraString::new();

        let string = "Banana".to_string();

        es.set_string(&string);

        let result = es.get_string();

        assert_eq!(string, result);
    }

    #[test]
    fn set_string_below_16_chars_twice() {
        let mut es = EscadraString::new();

        let string = "Banana".to_string();

        es.set_string(&string);
        let result = es.get_string();
        assert_eq!(string, result);

        es.set_string(&string);
        let result = es.get_string();
        assert_eq!(string, result);
    }

    #[test]
    fn set_string_then_read_above_16_chars() {
        let mut es = EscadraString::new();

        let string = "Banana Banana Banana Banana".to_string();

        es.set_string(&string);

        let result = es.get_string();

        assert_eq!(string, result);
    }

    #[test]
    fn set_string_above_16_chars_twice() {
        let mut es = EscadraString::new();

        let string = "Banana Banana Banana Banana".to_string();
        es.set_string(&string);
        let result = es.get_string();
        assert_eq!(string, result);

        es.set_string(&string);
        let result = es.get_string();
        assert_eq!(string, result);
    }

    #[test]
    fn set_large_then_set_small_then_read() {
        let mut es = EscadraString::new();

        let long_string = "Banana Banana Banana Banana".to_string();
        let short_string = "Banana".to_string();

        es.set_string(&long_string);
        es.set_string(&short_string);

        let result = es.get_string();

        assert_eq!(short_string, result);
    }

    #[test]
    fn set_16_len_string_then_read() {
        let mut es = EscadraString::new();

        let string = "BananBananBanana".to_string();

        es.set_string(&string);

        let result = es.get_string();

        assert_eq!(string, result);
    }

    #[test]
    fn set_15_len_string_then_read() {
        let mut es = EscadraString::new();

        let string = "BananBananBanan".to_string();

        es.set_string(&string);

        let result = es.get_string();

        assert_eq!(string, result);
    }

    #[test]
    fn pointer_is_null_terminated() {
        let mut es = EscadraString::new();

        let string = "Banana Banana Banana".to_string();
        es.set_string(&string);
        unsafe {
            assert!(*es.string.pointer.add(string.len()) == b'\0');
        }

        let string = "Banana".to_string();
        es.set_string(&string);
        unsafe {
            assert!(*es.string.pointer.add(string.len()) == b'\0');
        }
    }

    #[test]
    fn char_array_is_null_terminated() {
        let mut es = EscadraString::new();

        let string = "Banana".to_string();
        es.set_string(&string);
        unsafe {
            assert!(es.string.chars[string.len()] == b'\0');
        }

        let string = "Ban".to_string();
        es.set_string(&string);
        unsafe {
            assert!(es.string.chars[string.len()] == b'\0');
        }
    }
}
