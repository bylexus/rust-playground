// import privately:
mod dir;
mod file;
mod config;

// re-export, but with cut module name, so that we do not
// have to repeat dir::dir in the outer module:
pub use dir::Dir;
pub use file::File;
pub use config::Config;

