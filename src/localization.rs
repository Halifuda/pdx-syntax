//! # Syntax of paradox localization file
//!
//! Paradox mainly uses `yml` as the format of their localizaiton files. However,
//! there is a weird syntax in files. Specifically, the ':' might be followed with
//! a number. For example: `ABC:0 "This is ABC"`. I haven't worked out the meaning
//! of the number, however, it adds no difficulty to the parser. Here the parser
//! just gives an ambiguous result.
//!
use serde::{Serialize, Deserialize};

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(localization_syn);

/// A localization file. Begin with a language specifier, following with a list of localization entries.
#[derive(Serialize, Deserialize, Debug)]
pub struct LocList(Loc, Vec<Entry>);

/// Language, e.g., `l_english:`.
/// 
/// This is always the first line of a localization file.
#[derive(Serialize, Deserialize)]
pub struct Loc(String);

/// Entry, e.g., `ABC:0 "This is ABC"`.
/// 
/// All non-empty lines except the first line are entries.
#[derive(Serialize, Deserialize)]
pub struct Entry(String, Option<u8>, String);

super::fn_parse_file!(LocList, localization_syn::LocListParser);

pub mod types {
    //! Expose the types ([`LocList`], [`Loc`], [`Entry`]) to the outside.
    //! For convenience.
    //! 
    pub use super::{LocList, Loc, Entry};
}

/* ------------------------- impl Debug ------------------------- */

impl core::fmt::Debug for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.0)
    }
}

impl core::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            Some(n) => write!(f, "{}:{} {}", self.0, n, self.2),
            None => write!(f, "{}: {}", self.0, self.2),
        }
    }
}
