#![warn(missing_debug_implementations)]
// Turn on clippy pedantic, but not all of them yet.
#![warn(clippy::pedantic)]
#![allow(clippy::struct_excessive_bools)] // we like our bools
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
// When we do wildcards in `use`, it's deliberate
#![allow(clippy::enum_glob_use)]
#![allow(clippy::wildcard_imports)]
// Turn on some rustc lints
#![warn(future_incompatible)]
#![warn(missing_copy_implementations)]
#![warn(noop_method_call)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
// This was causing a lot of warnings.
#![allow(clippy::too_many_lines)]
// The construction being warned about here is the best way to express
// validation of a field while handling the case of the field not existing.
#![allow(clippy::blocks_in_if_conditions)]

pub mod everything;
pub mod gamedir;
pub mod modfile;
pub mod report;

#[cfg(feature = "ck3")]
mod ck3;
#[cfg(feature = "vic3")]
mod vic3;

mod block;
mod config_load;
mod context;
mod data;
mod datatype;
mod date;
mod db;
mod dds;
mod desc;
mod effect;
mod fileset;
mod helpers;
mod item;
mod macrocache;
mod modif;
mod parse;
mod pathtable;
mod pdxfile;
mod rivers;
mod scopes;
mod scriptvalue;
mod token;
mod tooltipped;
mod trigger;
mod util;
mod validate;
