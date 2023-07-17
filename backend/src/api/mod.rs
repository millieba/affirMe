// mod.rs - API module entry point

// Accessing the API modules in server.rs is easier as importing the api module will import all of its submodules.
// The individual submodules can be accessed using the "use api::{module_name}" syntax.

pub mod get_random_affirmation;

pub use get_random_affirmation::*;
