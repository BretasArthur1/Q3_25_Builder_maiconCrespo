#![allow(unexpected_cfgs,unused_imports)]
use anchor_lang::prelude::*;

mod state;
use state::*;

mod context;
use context::*;

mod error;
use error::*;

declare_id!("HYxi42pNZDn3dpnF8HPNeFurSLQSpcYWdvRSkfuqkkui");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn listing(ctx: Context<List>, price: u64) ->Result<()>{
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;
        Ok(())
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist_nft()?;
       
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.pay()?;
        ctx.accounts.transfer_nft()?;
       // ctx.accounts.receive_rewards()?;
       // ctx.accounts.close_mint_vault()?;
        Ok(())
   }
}


