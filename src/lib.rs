//! # Project Effort Tracker
//!
//! A Slint-based GUI application for tracking project efforts across different teams and weeks.
//!
//! ## Features
//!
//! - Track effort allocation by project, development team, and week
//! - Manage worker assignments and percentages
//! - Calculate totals and remaining effort
//! - Save/load data from JSON files
//! - Search and filter by worker name
//!
//! ## Modules
//!
//! - [`models`] - Data structures for projects, efforts, and conversions
//! - [`utils`] - Utility functions for calculations and parsing
//! - [`date_utils`] - Date and week manipulation functions
//! - [`file_io`] - File saving and loading operations
//! - [`callbacks`] - UI callback handlers

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

pub mod models;
pub mod utils;
pub mod date_utils;
pub mod file_io;
pub mod callbacks;
