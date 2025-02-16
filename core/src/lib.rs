//! Core components for MEV Bot infrastructure
//!
//! Provides essential functionality for:
//! - Blockchain interaction
//! - Transaction simulation
//! - Risk management
//! - MEV strategy execution

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod blockchain;
pub mod circuit_breaker;
pub mod mempool;
pub mod middleware;
pub mod risk;
pub mod security;
pub mod simulation;
