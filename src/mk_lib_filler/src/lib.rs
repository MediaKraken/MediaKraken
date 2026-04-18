//! # mk_lib_filler
//!
//! Intentionally a no-op crate. It exists so every service Dockerfile can
//! depend on the same set of `mk_lib_*` crates from the Kellnr registry
//! without conditionals: a service that would otherwise have no workspace
//! dependencies still pulls `mk_lib_filler` in, keeping the build graph and
//! the docker `.gitignore`s uniform.
//!
//! If a future refactor lets services drop unused `mk_lib_*` deps
//! individually, this crate can be retired. Until then, leave the body
//! empty — adding any real work here ships to every service.
pub mod mk_lib_filler;

pub use mk_lib_filler::mk_lib_filler;
