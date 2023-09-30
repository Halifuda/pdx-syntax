//! # Syntax of paradox script
//!
//! Offenly paradox scripts are written in `.txt` format under
//! `${game_root_dir}/game/common/*`. The syntax is highly similar to of JSON,
//! however, with slight differences. Some main differences are:
//! - `=` is used instead of `:` to separate key and value.
//! - There exist some other kinds of separators, including `?=`.
//! - `,` is not used to separate key-value pairs.
//! - Keys and values are not enclosed in `""`.
//!
//! Nevertheless, it is still very similar to JSON. We can define a simple syntax
//! to parse the paradox scripts.
//!
use serde::{Serialize, Deserialize};

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(script_syn);

/// A script is a list of units.
#[derive(Serialize, Deserialize, Debug)]
pub struct Units(Vec<Unit>);

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
#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
    Ident(String),
    RawStr(String),
}

/// Key. An entry.
#[derive(Serialize, Deserialize)]
pub struct Key(Entry);

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
    pub use super::{Key, Unit, Units, Value, Entry};
}

/* ------------------------- impl Debug ------------------------- */

impl core::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Empty => write!(f, "{{}}"),
            Value::Primitive(ref v) => write!(f, "{:?}", v),
            Value::Units(ref v) => write!(f, "{:#?}", v),
        }
    }
}

impl core::fmt::Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::KeyValue(ref k, ref v) => write!(f, "{:?} = {:?}", k.0, v),
            Unit::SingleValue(ref v) => write!(f, "{:?}", v),
            Unit::Questioned(ref k, ref v) => write!(f, "{:?} ?= {:?}", k.0, v),
            Unit::Relation(ref k, ref r, ref v) => write!(f, "{:?} {} {:?}", k.0, r, v),
        }
    }
}
