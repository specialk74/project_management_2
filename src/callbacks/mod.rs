//! UI callback handlers.
//!
//! This module contains all the callback handlers for UI interactions,
//! organized into separate files for better maintainability.

pub mod rebuild_project;
pub mod on_save_file;
pub mod on_new_project;
pub mod on_set_dev_effort;
pub mod on_changed_effort;
pub mod on_search;
pub mod on_add_row;

// Re-export commonly used functions
pub use rebuild_project::rebuild_project;
pub use on_save_file::register_on_save_file;
pub use on_new_project::register_on_new_project;
pub use on_set_dev_effort::register_on_set_dev_effort;
pub use on_changed_effort::register_on_changed_effort;
pub use on_search::register_on_search;
pub use on_add_row::register_on_add_row;
