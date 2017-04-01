//! Syncronize files and directories.
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_import_braces, unused_qualifications)]

#[macro_use]
extern crate error_chain;
extern crate libc;
extern crate tempdir;

mod errors;
mod utils;
