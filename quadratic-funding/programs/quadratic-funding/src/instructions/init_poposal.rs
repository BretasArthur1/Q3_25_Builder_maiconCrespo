use anchor_lang::prelude::*;

use crate::state::Dao;

#[derive(Accounts)]


pub struct InitPoposal<'info>{

    #[account(mut)]
    pub creator: Signer<'info>,

     #[account(mut)]
    pub dao_account: Account<'info,Dao>,

    #[account(
        init,
        payer = creator,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [b"proposal", dao_account.key().as_ref(),dao_account.praposal_count.to_le_bytes().as_ref()],
        bump,
    )]
    pub proposal: Account<'info, Proposal>,

    pub system_program: Program<'info, System>,
}