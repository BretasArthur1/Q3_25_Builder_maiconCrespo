user anchor_lang::prelude::*;

#[account]
#[derive(Debug,InitSpace)]
pub struct Dao{
    pub name:String,
    pub authority: Pubkey,
    pub praposal_count:u64,
    pub bump:u8,

}

#[account]
#[derive(Debug,InitSpace)]
pub struct Poroposal{
    
    pub authority: Pubkey,
    metadata
    yes_vote_count
    pub bump:u8,

}