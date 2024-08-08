use anchor_lang::system_program::{transfer, Transfer}; //from Dmitry
use anchor_lang::prelude::*;
mod state;
mod instructions;

//10h10 pm: look at the jumping around

//escrow defines the rules of the exchange
//the vault holds the assets

use instructions::*;

declare_id!("C6eakXoy9RSWRT8KkHCdoRrBsSDw8459x3Ju2Qx9bVWf");

pub mod state;
pub use state::*;

mod instructions;
pub use instructions::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, deposit: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }


    
    //take

    pub fn take(ctx: Context<Make>, seed: u64, receive: u64, deposit: u64) -> Result<()> {
        //TODO
        //make a transfer and close escrow
        //ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        //ctx.accounts.deposit(deposit)?;
        Ok(())
    }


    //close
}
