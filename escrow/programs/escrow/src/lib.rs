use anchor_lang::prelude::*;

//10h10 pm: look at the jumping around

declare_id!("C6eakXoy9RSWRT8KkHCdoRrBsSDw8459x3Ju2Qx9bVWf");

pub mod state;
pub use state::*;

mod instructions;
pub use instructions::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>, seed: u64, receive: u64, deposit: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }
}
