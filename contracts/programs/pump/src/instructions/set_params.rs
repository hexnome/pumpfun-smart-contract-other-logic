use crate::{constants::GLOBAL_STATE_SEED, error::*, state::Global};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetParams<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global: Box<Account<'info, Global>>,

    #[account(mut)]
    pub user: Signer<'info>,
}

pub fn set_params(
    ctx: Context<SetParams>,
    fee_recipient: Pubkey,
    owner_wallet: Pubkey,
    initial_virtual_token_reserves: u64,
    initial_virtual_sol_reserves: u64,
    initial_real_token_reserves: u64,
    token_total_supply: u64,
    fee_basis_points: u64,
    create_fee: u64,
    staging_pool_fee: u64,
) -> Result<()> {
    let global: &mut Box<Account<'_, Global>> = &mut ctx.accounts.global;
    require!(
        global.authority == ctx.accounts.user.key(),
        pumpCode::NotAuthorized
    );
    require!(
        fee_basis_points <= 10_000,
        pumpCode::InvalidFeeBasisPoints
    );
    require!(
        initial_virtual_token_reserves > 0,
        pumpCode::InvalidAmount
    );
    require!(initial_virtual_sol_reserves > 0, pumpCode::InvalidAmount);
    require!(token_total_supply > 0, pumpCode::InvalidSupply);

    global.fee_recipient = fee_recipient;
    global.owner_wallet = owner_wallet;
    global.initial_virtual_token_reserves = initial_virtual_token_reserves;
    global.initial_virtual_sol_reserves = initial_virtual_sol_reserves;
    global.initial_real_token_reserves = initial_real_token_reserves;
    global.token_total_supply = token_total_supply;
    global.fee_basis_points = fee_basis_points;
    global.create_fee = create_fee;
    global.staging_pool_fee = staging_pool_fee;

    Ok(())
}
