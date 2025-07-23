use anchor_lang::prelude::*;

#[account]
pub struct Escrow{
    pub maker : Pubkey,
    
}