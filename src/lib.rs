// inside lib.rs, only the following line should be in here
pub mod instruction;
pub mod error;
pub mod processor;

// Turn off our entrypoint because spl-token crate has its own entrypoint.
// We can only have one entry point. This allows others to import our program.
#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
