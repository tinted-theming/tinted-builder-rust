// With the new split of tinted-builder-rust (cli) and
// tinted-builder (lib), tinted-builder is exported here for bakward
// compatibility for a time. Everyone should move to using
// tinted-builder as the rust library.

mod operations;
mod utils;

pub use tinted_builder::Scheme;
pub use tinted_builder::Template;
