use anchor_lang::prelude::*;

mod instructions;
mod state;

declare_id!("EMeRwbKyMSRhdEL8LuwxYXvSYpZePYLuL3LqG8VEMsmd");

#[program]
pub mod myamm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
