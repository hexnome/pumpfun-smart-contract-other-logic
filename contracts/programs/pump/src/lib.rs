pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use events::*;
use instructions::*;
pub use state::*;

declare_id!("DmK2k1vXApWgkP1yhcKhnjqkVD79x8VHFVRMqNZzK4WH");

#[program]
pub mod pump {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
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
        instructions::set_params(
            ctx,
            fee_recipient,
            owner_wallet,
            initial_virtual_token_reserves,
            initial_virtual_sol_reserves,
            initial_real_token_reserves,
            token_total_supply,
            fee_basis_points,
            create_fee,
            staging_pool_fee,
        )
    }

    pub fn create(
        ctx: Context<Create>,
        number_stage: u64,
        stage_duration: u64,
        sell_tax_min: u64,
        sell_tax_max: u64,
        token_pool_destination: u64,
        amount: u64,
    ) -> Result<()> {
        instructions::create(
            ctx,
            number_stage,
            stage_duration,
            sell_tax_min,
            sell_tax_max,
            token_pool_destination,
            amount,
        )
    }

    pub fn buy(ctx: Context<Buy>, amount: u64, max_sol_cost: u64) -> Result<()> {
        instructions::buy(ctx, amount, max_sol_cost)
    }

    pub fn sell(ctx: Context<Sell>, amount: u64, min_sol_output: u64) -> Result<()> {
        instructions::sell(ctx, amount, min_sol_output)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        instructions::withdraw(ctx)
    }

    pub fn stage_completed(ctx: Context<StageCompleted>) -> Result<()> {
        instructions::stage_completed(ctx)
    }

    pub fn claim(ctx: Context<Claim>, amount: u64) -> Result<()> {
        instructions::claim(ctx, amount)
    }
}
