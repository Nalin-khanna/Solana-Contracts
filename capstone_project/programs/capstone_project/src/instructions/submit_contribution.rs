use anchor_lang::prelude::*;
use crate::{state::*};
use anchor_lang::system_program::{transfer };
use anchor_lang::system_program::{Transfer};

#[derive(Accounts)]
pub struct SubmitContribution<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(
        mut,
        seeds = [b"task", task_account.creator.as_ref(), task_account.task_seed.to_le_bytes().as_ref()],
        bump = task_account.task_bump,
    )]
    pub task_account: Account<'info, TaskAccount>,

    #[account(
        init,
        payer = contributor,
        seeds = [b"contribution", task_account.key().as_ref(), contributor.key().as_ref()],
        bump,
        space = 8 + ContributionAccount::INIT_SPACE
    )]
    pub contribution_account: Account<'info, ContributionAccount>,

    pub system_program: Program<'info, System>,
}
impl<'info> SubmitContribution<'info> {
    pub fn submit_contribution(
        &mut self,
        submission_uri: String,
        work_units: u64,
        bumps: &SubmitContributionBumps,
    ) -> Result<()> {
        self.contribution_account.set_inner(ContributionAccount{
            submission_uri,
            work_units,
            approved: false,
            created_at: Clock::get()?.unix_timestamp,
            bump: bumps.contribution_account,
        });

        self.task_account.total_submissions += 1;
        self.task_account.updated_at = Clock::get()?.unix_timestamp;

        Ok(())
    }
}