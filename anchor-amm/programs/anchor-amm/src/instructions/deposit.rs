use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{spl_token::instruction::AuthorityType, Mint, Token, TokenAccount},
};

use crate::state::Config;

#[derive(Account)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        has_one=mint_x,
        has_one=mint_y,
        seeds = [b"config",config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,

    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"lp",config.key.asref()],
        bump = c,


    )]
    pub mint_lp: Account<'info, Mint>,
}
