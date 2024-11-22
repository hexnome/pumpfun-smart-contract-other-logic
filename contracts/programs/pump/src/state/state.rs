use anchor_lang::prelude::*;

#[account]
pub struct Global {
    pub initialized: bool,
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub owner_wallet: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub create_fee: u64,
    pub staging_pool_fee: u64,
    pub backend_wallet: Pubkey,
}

#[account]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub token_mint: Pubkey,
    pub complete: bool,

    pub current_stage: u64,
    pub current_stage_complete: bool,
    pub number_stage: u64,
    pub stage_duration: i64,
    pub sell_tax_max: u64,
    pub sell_tax_min: u64,
    pub sell_tax_decay: u64,
    pub token_pool_destination: u64,
    pub at_start_stage: i64,
}
