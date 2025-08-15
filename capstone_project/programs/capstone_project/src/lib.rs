pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
use anchor_lang::prelude::*;
pub use constants::*;
pub use state::*;
pub use instructions::*;

declare_id!("GPtjsrW8gCZTbaMo7z1hx7YSanwo5AeecZpTWCZN4YDE");

#[program]
pub mod capstone_project {
    use super::*;

}
