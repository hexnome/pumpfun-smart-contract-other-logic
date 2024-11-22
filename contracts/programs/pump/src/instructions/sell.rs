use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{
    constants::{BONDING_CURVE, GLOBAL_STATE_SEED, REWARD_STATE_SEED, SOL_VAULT_SEED},
    error::*,
    events::*,
    state::{BondingCurve, Global},
};
use solana_program::{program::invoke_signed, system_instruction};

#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global: Box<Account<'info, Global>>,

    #[account(
        mut,
        seeds = [REWARD_STATE_SEED],
        bump
    )]
    pub reward_recipient: AccountInfo<'info>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = reward_recipient,
    )]
    
    pub associated_reward_recipient: Box<Account<'info, TokenAccount>>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub fee_recipient: SystemAccount<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [SOL_VAULT_SEED, mint.key().as_ref()],
        bump
    )]
    /// CHECK: this should be set by admin
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [BONDING_CURVE, mint.key().as_ref()],
        bump
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = bonding_curve,
    )]
    pub associated_bonding_curve: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = user
    )]
    pub associated_user: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn sell(ctx: Context<Sell>, amount: u64, min_sol_output: u64) -> Result<()> {
    let accts
    require!(
        accts.fee_recipient.key() == accts.global.fee_recipient,
        pumpCode::InValidFeeRecipient
    );
    require!(
        accts.bonding_curve.complete == false,
        pumpCode::BondingCurveComplete
    );
    require!(amount > 0, pumpCode::ZeroAmount);
    require!(accts.associated_bonding_curve.owner.key().eq(&accts.token_program.key()) , pumpCode::NotAuthorized);
    require!(accts.associated_user.owner.key().eq(&accts.token_program.key()) , pumpCode::NotAuthorized);

    let clock =
    let bonding_curve =

    // Calculate the required SOL cost for the given token amount
    let mut sol_cost = 
    if accts.bonding_curve.real_sol_reserves < sol_cost {
        sol_cost = 
    }

    // Ensure the SOL cost does not exceed max_sol_cost
    require!(
        sol_cost >= min_sol_output,
        pumpCode::TooLittleSolReceived
    );

    // send sol from vault account to user (calculate fee)
    let binding = accts.mint.key();

    let (_, bump) =
        
    let vault_seeds 
    let signer

    // Calc sol_amount with sell_tax and tax_decay
    let tax_period

    let tax_decay =

    let tax_percentage: u64;
    if tax_decay < tax_period {
        tax_percentage
    } else {
        let temp_tax_div
        let temp_sell_tax_range
        let temp
            
        tax_percentage
    }

    let fee_amount 
    let reward_sol_amount 
        
    let token_amount 
    // Send SOL vault to fee_recipient and user
    invoke_signed(
        &system_instruction::transfer(&accts.vault.key(), &accts.fee_recipient.key(), fee_amount),
        &[
            accts.vault.to_account_info().clone(),
            accts.fee_recipient.to_account_info().clone(),
            accts.system_program.to_account_info().clone(),
        ],
        signer,
    )?;

    invoke_signed(
        &system_instruction::transfer(
            &accts.vault.key(),
            &accts.user.key(),
            sol_cost - fee_amount - reward_sol_amount,
        ),
        &[
            accts.vault.to_account_info().clone(),
            accts.user.to_account_info().clone(),
            accts.system_program.to_account_info().clone(),
        ],
        signer,
    )?;
    // send tokens user to the vault
    let cpi_ctx = CpiContext::new(
        accts.token_program.to_account_info(),
        Transfer {
            from: accts.associated_user.to_account_info().clone(),
            to: accts.associated_bonding_curve.to_account_info().clone(),
            authority: accts.user.to_account_info().clone(),
        },
    );
    transfer(cpi_ctx, token_amount)?;

    // Send tokens user to reward_recipient
    let cpi_ctx_reward = CpiContext::new(
        accts.token_program.to_account_info(),
        Transfer {
            from: accts.associated_user.to_account_info().clone(),
            to: accts.associated_reward_recipient.to_account_info(),
            authority: accts.user.to_account_info().clone(),
        },
    );
    transfer(cpi_ctx_reward, amount - token_amount)?;

    //  update the bonding curve
    accts.bonding_curve.real_token_reserves 
    accts.bonding_curve.virtual_token_reserves 
    accts.bonding_curve.virtual_sol_reserves 
     
    accts.bonding_curve.real_sol_reserves = 
    // Log the TradeEvent details

    msg!(
        "Trade // Type: Sell, User: {}, Mint: {}, BondingCurve: {}, Timestamp: {}, SolCost: {}, Amount: {}, IsBuy: {}, VirtualSolReserves: {}, VirtualTokenReserves: {}",
        accts.user.key(),
        accts.mint.key(),
        accts.bonding_curve.key(),
        accts.clock.unix_timestamp,
        sol_cost,
        amount,
        false,
        accts.bonding_curve.virtual_sol_reserves,
        accts.bonding_curve.virtual_token_reserves
    );

    emit!(TradeEvent {
        mint: accts.mint.key(),
        sol_amount: sol_cost,
        token_amount: amount,
        is_buy: false,
        user: accts.user.key(),
        timestamp: accts.clock.unix_timestamp,
        virtual_sol_reserves: accts.bonding_curve.virtual_sol_reserves,
        virtual_token_reserves: accts.bonding_curve.virtual_token_reserves,
        fee_amount: fee_amount,
    });

    Ok(())
}

fn calculate_sol_cost(bonding_curve: &Account<BondingCurve>, token_amount: u64) -> Result<u64> {
    let price_per_token = (bonding_curve.virtual_token_reserves as u128)
        .checked_add(token_amount as u128)
        .ok_or(pumpCode::MathOverflow)?;

    let total_liquidity = (bonding_curve.virtual_sol_reserves as u128)
        .checked_mul(bonding_curve.virtual_token_reserves as u128)
        .ok_or(pumpCode::MathOverflow)?;

    let new_sol_reserve = total_liquidity
        .checked_div(price_per_token)
        .ok_or(pumpCode::MathOverflow)?;

    let sol_cost = ((bonding_curve.virtual_sol_reserves as u128)
        .checked_sub(new_sol_reserve)
        .ok_or(pumpCode::MathOverflow)?) as u64;
    Ok(sol_cost as u64)
}
