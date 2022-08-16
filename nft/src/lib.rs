pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;
pub mod utils_mint;
pub mod utils_config;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("CRVdA6LCjMCJeajgUKS2rKB7mGu4P1CsvUkCJkw2n5dN");