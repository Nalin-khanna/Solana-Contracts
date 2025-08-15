use anchor_lang::prelude::*;

#[account]
pub struct TaskAccount {
    pub reward_amount: u64,
    pub total_work_units: u64,
    pub task_uri : String,
    pub status: TaskStatus,
    pub total_submissions: u32,
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Created,
    SubmissionsOpen,
    UnderReview,
    Finalized,
}