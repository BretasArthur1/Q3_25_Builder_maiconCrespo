use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    // token::{transfer_checked, TransferChecked},
    token_interface::{
        Mint, TokenAccount, TokenInterface,
    close_account, transfer_checked, TransferChecked, CloseAccount,
    },
};

use crate::state::{Listing, Marketplace};



#[derive(Accounts)]
 pub struct Delist<'info>{
    #[account(mut)]
    pub maker:Signer<'info>,

    pub maker_mint: InterfaceAccount<'info, Mint>, // The NFT mint being listed

      #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>, // The marketplace configuration account
   

     #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
        associated_token::token_program= token_program
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>, // Token account holding the NFT

#[account(
        mut,    
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
        associated_token::token_program= token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>, // Escrow account for the NFT during listing


 #[account(
        mut,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump= listing.bump,
        close=maker
            )]
    pub listing: Account<'info, Listing>, // Account to store listing information



     pub collection_mint: InterfaceAccount<'info, Mint>,
     
      // Collection the NFT belongs to
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>, // NFT metadata to verify collection
    
    #[account(
        seeds = [
            b"metadata", 
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>, // Master edition to verify it's an NFT
    
    pub metadata_program: Program<'info, Metadata>, // Metaplex program
    pub associated_token_program: Program<'info, AssociatedToken>, // For creating ATAs
    pub system_program: Program<'info, System>, // For creating accounts
    pub token_program: Interface<'info, TokenInterface> // For token operations
 }

impl<'info>Delist<'info>{

    pub fn delist_nft(&mut self)->Result<()>{

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked{
            from: self.vault.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.maker_mint.to_account_info(),    
        };

        let seeds = &[
            b"list",
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx =  CpiContext::new_with_signer( cpi_program, cpi_accounts, signer_seeds );

        transfer_checked(cpi_ctx, 1, 0)?;   

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let close_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            signer_seeds,
        );

        close_account(close_ctx)?;
        Ok(())


    }
}