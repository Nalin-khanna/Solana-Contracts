use anchor_lang::prelude::*;

#[account]
pub struct ContributionAccount {
    pub submission_uri : String,
    pub work_units: u64,
    pub approved : bool,
    pub created_at: i64,
    pub bump: u8,
}