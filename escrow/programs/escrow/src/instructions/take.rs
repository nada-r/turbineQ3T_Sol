use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface, CloseAccount};

//10h59 pm

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub maker: SystemAccount<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker, 
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        cose = maker,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

//11h10 pm

impl<'info> Take<'info> {
    pub fn deposit(&mut self) -> Result<()> {
        let cpi_program: AccountInfo = self.token_program.to_account_info();

        let cpi_accounts:TransferChecked = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
            mint: self.mint_b.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, amount, self.mint_a.decimals, false)?; 

        //transfer_checked(cpi_ctx, self.escrow.receive, self.mint_a.decimals) //Juan o Dmitry

        Ok(())
    }
/*/Dmitry + Juan start
    pub fn withdraw(&mut self) -> Result<()> {
        let signer_seeds: [
            &[&[u8]]; 1
        ] = [&[b"escrow", self.maker.to_account_info.key().as_ref(), self.escrow.seed.to_le_bytes()[..], &[self.escrow.bump] ], ];

        let accounts = TransferChecked{
            from: self.vault.to_account_info(),
            to : self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };

        let cpi_program: AccountInfo = self.token_program.to_account_info();

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, accounts, &signer_seeds);

        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        close_account(cpi_ctx)?;

        Ok(())
    }

//Dmitry + Juan end */







}


pub fn withdraw_and_close_vault(&mut self) -> Result<()> {
    let cpi_program: AccountInfo = self.token_program.to_account_info();

    let cpi_accounts: TransferChecked = TransferChecked {
        from: self.vault.to_account_info(),
        to: self.taker_ata_a.to_account_info(),
        authority: self.escrow.to_account_info(),
        mint: self.mint_a.to_account_info(),
    };

    let cpi_ctx : CpiContext<TransferChecked> = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds]);

    transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

    let accounts: CloseAccount = CloseAccount {
        account: self.vault.to_account_info(),
        destination: self.taker.to_account_info(),
        authority: self.escrow.to_account_info(),
    };

    let cpi_program: AccountInfo = self.system_program.to_account_info();

    let cpi_ctx: CpiContext<CloseAccount> = CpiContext::new_with_signer(cpi_program, accounts, &signer_seeds);
    close_account(cpi_ctx)?;

    Ok(())
}