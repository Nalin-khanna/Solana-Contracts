use anchor_lang::prelude::*;

#[account]
pub struct ReviewVoteAccount {
    pub approve: bool,           // Vote decision
    pub created_at: i64,         // Timestamp
    pub bump: u8,                // PDA bump
}
