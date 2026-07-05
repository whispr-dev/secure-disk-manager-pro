//! Safe Rust equivalents for the `Secure_DiskManager_Pro` C++ modules.
//!
//! The archive contains a mixture of useful local OS utilities, proof-of-concept
//! stubs, and modules labelled as stealth / kill-switch / self-delete. This crate
//! ports the useful local operators and exposes safe no-op/blocked equivalents
//! for covert networking, stealth identity mutation, hidden execution, and
//! destructive automation.

pub mod cli;
pub mod compression;
pub mod disk_management;
pub mod dpapi;
pub mod encryption;
pub mod entropy;
pub mod error;
pub mod file_transfer;
pub mod fs_monitor;
pub mod ghost_controller;
pub mod gpg_wrapper;
pub mod identity;
pub mod key_manager;
pub mod net_admin;
pub mod payload_dispatcher;
pub mod process_services;
pub mod quantum;
pub mod rng;
pub mod search;
pub mod secure_deletion;
pub mod stealth_mailer;
pub mod system_monitor;

pub use error::{Result, SdmError};
