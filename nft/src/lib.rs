pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;
pub mod utils_mint;
pub mod utils_config;
pub mod utils_battle;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("5HS5v5SE1nKoBZB6FzswSUQU6rhpWPw8fPLKaBBc2hnW");