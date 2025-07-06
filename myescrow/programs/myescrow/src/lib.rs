#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod instructions;
use instructions::*;
pub mod state;

declare_id!("6eUaYaReW47ZUiatopCy7f7e35iUJwPkeJT9DBow5ZBt");

#[program]
pub mod myescrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seeds: u64, recieve: u64, deposit: u64) -> Result<()> {
        ctx.accounts.init_escrow(seeds, recieve, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.withdraw_and_close_vault()?;
        Ok(())
    }
}
