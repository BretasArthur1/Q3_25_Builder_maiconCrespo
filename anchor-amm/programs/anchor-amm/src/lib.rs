use anchor_lang::prelude::*;

declare_id!("CCpTvJaqt8P7wJAx9RZLvJvFpHrooSfW8mB2X6qvHpcK");

#[program]
pub mod anchor_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
