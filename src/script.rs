//! # Syntax of paradox script
//!
//! Offenly paradox scripts are written in `.txt` format. The syntax is highly similar to JSON,
//! however, with slight differences. Some main differences are:
//! - `=` is used instead of `:` to separate keys and values.
//! - There exist some other kinds of separators, including `?=` and `<=` (operators).
//! - `,` is not used to separate key-value pairs.
//! - Single value without key is allowed.
//! - Keys and values are usually not enclosed in `""`, except for some special cases like 
//! specifying file paths.
//!
//! Nevertheless, it is still very similar to JSON. We can define a simple syntax
//! to parse the paradox scripts.
//!
use serde::{Deserialize, Serialize};

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(script_syn);

/// A script is a list of units.
#[derive(Serialize, Deserialize, Debug)]
pub struct Units(Vec<Unit>);

impl core::ops::Deref for Units {
    type Target = Vec<Unit>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Units {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Unit. May be:
/// - a key-value pair: `key = value`;
/// - a single value: `value`;
/// - a questioned value: `key ?= value`;
/// - a relation: `key > value`, `key < value`, `key >= value`, `key <= value`, `key == value`.
#[derive(Serialize, Deserialize)]
pub enum Unit {
    KeyValue(Key, Value),
    SingleValue(Value),
    Questioned(Key, Value),
    Relation(Key, String, Value),
}

/// Entry. Can be:
/// - an identifier: we define a name that is not enclosed in quotes is an identifier;
/// - a raw string: a string enclosed in quotes.
/// 
/// Entries can appear in keys and values. Parsing the exact and specific meaning of an entry
/// is not the job of the parser, for Paradox uses the script almost everywhere, and the 
/// meaning of an entry depends on the context.
#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
    Ident(String),
    RawStr(String),
}

/// Key. An entry.
#[derive(Serialize, Deserialize)]
pub struct Key(Entry);

impl core::ops::Deref for Key {
    type Target = Entry;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Key {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Value. Can be:
/// - empty: `{}`;
/// - primitive: a single entry;
/// - recursive: `{ <units> }`.
#[derive(Serialize, Deserialize)]
pub enum Value {
    Empty,
    Primitive(Entry),
    Units(Box<Units>),
}

super::fn_parse_file!(Units, script_syn::UnitsParser);

pub mod types {
    //! Expose the types ([`Key`], [`Unit`], [`Value`], [`Entry`]) to the outside.
    //! For convenience.
    //!
    pub use super::{Entry, Key, Unit, Units, Value};
}

/* ------------------------- impl Debug ------------------------- */

impl core::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Empty => write!(f, "{{}}"),
            Value::Primitive(v) => write!(f, "{:?}", v),
            Value::Units(v) => write!(f, "{:#?}", v),
        }
    }
}

impl core::fmt::Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::KeyValue(k, v) => write!(f, "{:?} = {:?}", k.0, v),
            Unit::SingleValue(v) => write!(f, "{:?}", v),
            Unit::Questioned(k, v) => write!(f, "{:?} ?= {:?}", k.0, v),
            Unit::Relation(k, r, v) => write!(f, "{:?} {} {:?}", k.0, r, v),
        }
    }
}

/* ------------------------ impl Display ------------------------ */
// [`Display`] is used to dump the AST back to files.

impl core::fmt::Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.recursive_write(f, 0)
    }
}

impl Units {
    pub fn recursive_write(&self, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
        for unit in &self.0 {
            unit.recursive_write(f, level)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Unit {
    fn recursive_write(&self, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
        write!(f, "{}", "    ".repeat(level))?;
        match self {
            Unit::KeyValue(k, v) => {
                write!(f, "{} = ", k.0)?;
                v.recursive_write(f, level + 1)
            }
            Unit::SingleValue(v) => v.recursive_write(f, level + 1),
            Unit::Questioned(k, v) => {
                write!(f, "{} ?= ", k.0)?;
                v.recursive_write(f, level + 1)
            }
            Unit::Relation(k, r, v) => {
                write!(f, "{} {} ", k.0, r)?;
                v.recursive_write(f, level + 1)
            }
        }
    }
}

impl Value {
    fn recursive_write(&self, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
        match self {
            Value::Empty => write!(f, "{{}}"),
            Value::Primitive(v) => write!(f, "{}", v),
            Value::Units(v) => {
                writeln!(f, "{{")?;
                v.recursive_write(f, level + 1)?;
                write!(f, "{}}}", "    ".repeat(level.saturating_sub(1)))
            }
        }
    }
}

impl core::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Entry::Ident(s) | Entry::RawStr(s) => write!(f, "{}", s),
        }
    }
}
