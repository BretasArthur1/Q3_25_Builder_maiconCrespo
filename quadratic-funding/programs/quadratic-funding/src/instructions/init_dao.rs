use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(name:String)]

pub struct InitDao<'info>{
    #[account(mut)]
    pub creator: Signer<'info>

    #[account(
        init,
        payer = creator,
        space = 8+Dao::INIT_SPAC,
        seeds = [b"dao", creator.key().as_ref()],
        bump,
    )]
    pub dao_account:Account<'info,Dao>,

    pub system_program: Program<'info, System>,
    pub dao_account: Account<'info, Dao>,


}

pub fn init_dao(ctx:Context<init_dao>,name: String)->Result<()>{

    let dao_acount = &mut ctx.accounts.dao_account;

    dao_account.set_inner(
        Dao{
            name,
            authority: ctx.accounts.creator.key(),
            praposal_count: 0,
            bump: ctx.bumps.get("dao_account").unwrap(),
        }
    )
}