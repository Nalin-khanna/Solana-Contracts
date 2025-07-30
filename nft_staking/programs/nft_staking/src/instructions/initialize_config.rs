use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::state_config::StateConfig;
//  pub points_per_stake: u8,
//     pub max_stake: u8,
//     pub freeze_period: u32,
//     pub rewards_period: u32,
//     pub bump: u8,
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin : Signer<'info>,

    #[account(
        init ,
        payer = admin ,
        space = 8 + StateConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config : Account<'info , StateConfig>,

    #[account(
        init_if_needed , 
        payer = admin , 
        seeds = [b"rewards_mint",  config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config
    )]
    pub rewards_mint : Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeConfig>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
