pub mod cli;
pub mod config;
mod req;
mod utils;

pub use req::diff;
pub use utils::{diff_text, highlight_text};
