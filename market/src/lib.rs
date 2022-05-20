pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub const PREFIX: &str = "fallen";

solana_program::declare_id!("2cZVDMdeRsuSmeD94PY8uotzqYfKTegbXPSAMbHaXyiF");
