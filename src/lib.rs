pub mod shell;  // make the module public
pub use shell::run_shell;

pub mod server;
pub use server::{get_fn, set_fn, unset_fn};

pub mod ferrodb;
pub use ferrodb::FerroDB;