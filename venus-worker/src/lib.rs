#![deny(missing_docs)]
//! venus-worker is used to seal sectors based on local resources

pub(crate) mod config;
pub(crate) mod infra;
pub(crate) mod metadb;
pub(crate) mod rpc;
pub(crate) mod signal;
pub(crate) mod types;
pub(crate) mod watchdog;

pub(crate) mod sealing;

mod run;

pub use config::Config;
pub use infra::objstore;
pub use run::{start_deamon, start_mock};
pub use sealing::seal::external::sub::{run_c2, run_pc2};
pub use sealing::store;
pub use sealing::util as seal_util;

pub mod client;
pub mod logging;
