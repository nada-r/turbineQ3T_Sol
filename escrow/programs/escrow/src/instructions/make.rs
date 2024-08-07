use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TransferChecked, TokenInterface};

//10h17PM

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Make<'info> {
    pub fn init_escrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint.key(),
            mint_b: self.mint.key(),
            receive,
            bump: bumps.escrow, 
        })
        Ok(())
    }
}

pub fn deposit(&mut self, amount: u64) -> Result<()> {
    let cpi_program: AccountInfo = self.token_program.to_account_info();

    let cpi_accounts = TransferChecked {
        from: self.maker_ata_a.to_account_info(),
        to: self.vault.to_account_info(),
        authority: self.maker.to_account_info(),
        mint: self.mint_a.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    transfer_checked(cpi_ctx, amount, self.mint_a.decimals, false)?;

    Ok(()).
}


/*
For those who are confused about ATA vs. Token Account:

In short,

An ATA is a Token Account.

The major difference between an ATA and a random Token Account is:

'How is the address of the token account derived?'

A Token Account can have any random address, while an ATA is deterministically derived from the user's address + mint. */

//token interface https://solana.stackexchange.com/questions/4185/token-2022-in-anchor-program/10397#10397