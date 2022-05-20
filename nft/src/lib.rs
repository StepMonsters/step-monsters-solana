pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;
// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("8RRS1XGbGYQdZUr754Lv3kcpj1t67KtjyemA7wxngiW8");