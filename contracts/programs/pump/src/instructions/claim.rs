use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{
    constants::{BONDING_CURVE, REWARD_STATE_SEED, SOL_VAULT_SEED, GLOBAL_STATE_SEED},
    state::{BondingCurve, Global},
    error::*,
};

#[derive(Accounts)]
pub struct Claim<'info> {
    pub mint: Account<'info, Mint>,

    #[account(
        seeds = [REWARD_STATE_SEED],
        bump
    )]
    pub reward_recipient: AccountInfo<'info>,

    #[account(
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global: Box<Account<'info, Global>>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = reward_recipient,
    )]
    pub associated_reward_recipient: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [SOL_VAULT_SEED, mint.key().as_ref()],
        bump
    )]
    /// CHECK: this should be set by admin
    pub vault: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [BONDING_CURVE, mint.key().as_ref()],
        bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = bonding_curve,
    )]
    pub associated_bonding_curve: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = user
    )]
    pub associated_user: Account<'info, TokenAccount>,

    #[account(
        constraint = backend_wallet.key() == global.owner_wallet.key() @pumpCode:: NotAuthorized
    )]
    pub backend_wallet: Signer<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn claim(ctx: Context<Claim>, amount: u64) -> Result<()> {
    let accts 
    require!(accts.bonding_curve.current_stage_complete == true, pumpCode::NotStageComplete);
    // send token reward_recipient to user(calculate claim amount)
    let binding = accts.mint.key();

    let (_, bump) =
      
    let reward_seeds
    let signer = &[&reward_seeds[..]];

    let cpi_ctx: CpiContext<'_, '_, '_, '_, Transfer<'_>> = CpiContext::new(
        accts.token_program.to_account_info(),
        Transfer {
            from: accts.associated_reward_recipient.to_account_info().clone(),
            to: accts.associated_user.to_account_info().clone(),
            authority: accts.reward_recipient.to_account_info().clone(),
        },
    );
    transfer(cpi_ctx.with_signer(signer), amount)?;
    Ok(())
}
