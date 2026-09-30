pub mod auth;
pub mod config;
pub mod error;
pub mod server;
pub mod client;
pub mod commands;
mod uds;

#[cfg(test)]
pub mod test{
macro_rules! test_resource {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/resources/tests/", $fname)
    };
}
pub (crate) use test_resource;
}
