//! Defines the data type for the TLL (triply linked list) type

use core::fmt;
use std::collections::{HashMap, HashSet};

use super::EscadraString;

/// Struct used when exploring the TLL.
/// It holds the *mut TLL pointers for the a, b, and c fields for a given TLL.
pub struct TLLRef {
    /// The a pointer for the TLL.
    pub a: *mut TLL,
    /// The b pointer for the TLL.
    pub b: *mut TLL,
    /// The c pointer for the TLL.
    pub c: *mut TLL,
}

/// Represents an element in a triply linked list.
/// The only current known use is to hold Airplane loadout information and for keyboard input information.
#[repr(C)]
pub struct TLL {
    a: *mut TLL,
    b: *mut TLL,
    c: *mut TLL,
    end: bool,
    flag: bool,
    padding_1ah: u16,
    index: u32,
    /// String held by the TLL.
    pub string: EscadraString,
    unknown_40h: u32,
    padding_44h: u32,
    data1: *mut u8,
    data2: *mut u8,
    data3: *mut u8,
}

impl fmt::Debug for TLL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TLL")
            // .field("a", &self.a)
            // .field("b", &self.b)
            // .field("c", &self.c)
            .field("end", &self.end)
            .field("flag", &self.flag)
            .field("padding_1ah", &self.padding_1ah)
            .field("index", &self.index)
            // .field("string", &self.string)
            .field("unknown_40h", &self.unknown_40h)
            .field("padding_44h", &self.padding_44h)
            .field("data1", &self.data1)
            .field("data2", &self.data2)
            .field("data3", &self.data3)
            .finish()
    }
}

impl TLL {
    /// Recursively explores a TLL. Returns a Hashmap of TLL pointers and their a, b, c pointers in a TLLRef.
    pub fn explore(&self) -> HashMap<*const TLL, TLLRef> {
        let mut visited = HashSet::new();
        let mut result = HashMap::new();

        self.explore_internal(&mut visited, &mut result);
        result
    }

    fn explore_internal(
        &self,
        visited: &mut HashSet<*mut TLL>,
        result: &mut HashMap<*const TLL, TLLRef>,
    ) {
        visited.insert(self as *const TLL as *mut TLL);

        result.insert(
            self as *const TLL,
            TLLRef {
                a: self.a,
                b: self.b,
                c: self.c,
            },
        );

        unsafe {
            if !self.a.is_null() && !visited.contains(&self.a) {
                (*self.a).explore_internal(visited, result);
            }
            if !self.b.is_null() && !visited.contains(&self.b) {
                (*self.b).explore_internal(visited, result);
            }
            if !self.c.is_null() && !visited.contains(&self.c) {
                (*self.c).explore_internal(visited, result);
            }
        }
    }

    /// Recursively prints the TLL and all of its children.
    pub fn print(&self) {
        let mut visited = HashSet::new();
        visited.insert(self as *const TLL as *mut TLL);
        self.print_internal(0, &mut visited);
    }

    /// Internal function to print the TLL, avoiding already visited pointers.
    fn print_internal(&self, depth: usize, visited: &mut HashSet<*mut TLL>) {
        let indent = "  ".repeat(depth);

        println!("{}TLL {:p} {{", indent, self as *const TLL as *mut TLL);
        println!("{}  a: {:p}", indent, self.a);
        println!("{}  b: {:p}", indent, self.b);
        println!("{}  c: {:p}", indent, self.c);
        println!("{}  end: {}", indent, self.end);
        println!("{}  flag: {}", indent, self.flag);
        println!("{}  padding_1ah: {}", indent, self.padding_1ah);
        println!("{}  index: {}", indent, self.index);
        println!("{}  string: {:?}", indent, self.string);
        println!("{}  unknown_40h: {}", indent, self.unknown_40h);
        println!("{}  padding_44h: {}", indent, self.padding_44h);
        println!("{}  data1: {:p}", indent, self.data1);
        println!("{}  data2: {:p}", indent, self.data2);
        println!("{}  data3: {:p}", indent, self.data3);
        println!("{}}}", indent);

        unsafe {
            if !self.a.is_null() && !visited.contains(&self.a) {
                visited.insert(self.a);
                (*self.a).print_internal(depth + 1, visited);
            }
            if !self.b.is_null() && !visited.contains(&self.b) {
                visited.insert(self.b);
                (*self.b).print_internal(depth + 1, visited);
            }
            if !self.c.is_null() && !visited.contains(&self.c) {
                visited.insert(self.c);
                (*self.c).print_internal(depth + 1, visited);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tll_size() {
        assert_eq!(std::mem::size_of::<TLL>(), 0x60);
    }
}
