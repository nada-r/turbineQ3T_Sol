use anchor_lang::prelude::*;

// 10h15 mintA is the maker token, that he is going to deposit
//mintb is the token that maker is expecting to receive

#[account]
#[derive(Initspace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub amount: u64,
    pub is_initialized: bool,
}