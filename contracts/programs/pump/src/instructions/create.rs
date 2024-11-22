use crate::{
    constants::{BONDING_CURVE, GLOBAL_STATE_SEED, REWARD_STATE_SEED, SOL_VAULT_SEED, VAULT_SEED},
    error::*,
    events::*,
    state::{BondingCurve, Global},
};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
use chrono::Duration;
use solana_program::{program::invoke, system_instruction};
use std::mem::size_of;

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        seeds = [REWARD_STATE_SEED],
        bump
    )]
    pub reward_recipient: AccountInfo<'info>,

    #[account(
        init,
        payer = user,
        seeds = [REWARD_STATE_SEED, mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = reward_recipient,
    )]
    pub associated_reward_recipient: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Box<Account<'info, Mint>>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub fee_recipient: AccountInfo<'info>, // wallet address to receive the fee as SOL

    #[account(
        init,
        payer = user,
        seeds = [BONDING_CURVE, mint.key().as_ref()],
        bump,
        space = 8 + size_of::<BondingCurve>()
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [VAULT_SEED, mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = bonding_curve,
    )]
    pub associated_bonding_curve: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub associated_user_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = user,
        seeds = [SOL_VAULT_SEED, mint.key().as_ref()],
        bump,
        space = 8,
    )]
    pub vault: AccountInfo<'info>,

    #[account(
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global: Box<Account<'info, Global>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
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
    let global
    let bonding_curve
    let mint

    let clock 
    let one_day
    let total_stage_duration 

    require!(
        ctx.accounts.mint.freeze_authority.is_none(),
        pumpCode::FreezeAuthorityEnabled
    );
    require!(
        ctx.accounts.mint.mint_authority.is_none(),
        pumpCode::MitAuthorityEnabled
    );
    require!(global.initialized == true, pumpCode::NotInitialized);
    require!(
        ctx.accounts.fee_recipient.key() == ctx.accounts.global.fee_recipient,
        pumpCode::InValidFeeRecipient
    );
    require!(
        ctx.accounts
            .associated_user_account
            .owner
            .key()
            .eq(&ctx.accounts.user.key()),
        pumpCode::NotAuthorized
    );
    require!(
        ctx.accounts
            .associated_user_account
            .mint
            .key()
            .eq(&ctx.accounts.mint.key()),
        pumpCode::NotAuthorized
    );
    require!(
        ctx.accounts.associated_bonding_curve.owner == bonding_curve.key(),
        pumpCode::NotAuthorized
    );
    require!(mint.supply == amount, pumpCode::InvalidAmount);
    require!(ctx.accounts.associated_user_account.amount >=amount, pumpCode::InsufficientFunds);

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx
                .accounts
                .associated_user_account
                .to_account_info()
                .clone(),
            to: ctx
                .accounts
                .associated_bonding_curve
                .to_account_info()
                .clone(),
            authority: ctx.accounts.user.to_account_info().clone(),
        },
    );
    transfer(cpi_ctx, amount)?;

    invoke(
        &system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.fee_recipient.key(),
            global.create_fee,
        ),
        &[
            ctx.accounts.user.to_account_info().clone(),
            ctx.accounts.fee_recipient.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;
    // init the bonding curve
    bonding_curve.virtual_token_reserves 
    bonding_curve.virtual_sol_reserves
    bonding_curve.real_token_reserves
    bonding_curve.real_sol_reserves
    bonding_curve.token_total_supply
    bonding_curve.token_mint
    bonding_curve.complete 

    bonding_curve.current_stage
    bonding_curve.current_stage_complete 
    bonding_curve.number_stage 
    bonding_curve.stage_duration 
    bonding_curve.sell_tax_max
    bonding_curve.sell_tax_min
    bonding_curve.token_pool_destination 
    bonding_curve.at_start_stage 

    // Log the event details
    msg!(
        "CreateEvent - Mint: {}, BondingCurve: {}, User: {}",
        ctx.accounts.mint.key(),
        bonding_curve.key(),
        ctx.accounts.user.key()
    );

    emit! {
        CreateEvent {
            mint: ctx.accounts.mint.key(),
            bonding_curve: bonding_curve.key(),
            user: ctx.accounts.user.key()
        }
    }

    Ok(())
}
