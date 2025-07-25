// Import all commonly used items from the anchor_lang prelude
use anchor_lang::prelude::*;

// Define an account struct for Anchor with InitSpace derive macro
#[account]
#[derive(InitSpace)]
pub struct Config {
    // A seed value for PDA derivation or other purposes
    pub seed: u64,
    // Optional authority public key
    pub authority: Option<Pubkey>,
    // Public key of the first mint (token X)
    pub mint_x: Pubkey,
    // Public key of the second mint (token Y)
    pub mint_y: Pubkey,
    // Fee value (basis points or similar)
    pub fee: u16,
    // Indicates if the config is locked
    pub locked: bool,
    // Bump seed for the config PDA
    pub config_bump: u8,
    // Bump seed for the LP token PDA
    pub lp_bump: u8,
}
