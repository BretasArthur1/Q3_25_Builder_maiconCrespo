use anchor_lang::prelude::*;

declare_id!("FVAFbbSSdWVcFAZ5N5UAA5UCwnL9BacrfR7jv1Jom5Ud");

#[program]
pub mod quadratic_funding {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
