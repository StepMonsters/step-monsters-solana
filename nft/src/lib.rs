pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;
// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("Dp3a5v5czgaQ8XQL7E5vhMRGJekQ6mqhQM2zEKdwDS1s");