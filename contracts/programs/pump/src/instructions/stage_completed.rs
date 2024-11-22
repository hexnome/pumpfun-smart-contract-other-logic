use crate::{constants::BONDING_CURVE, error::*, events::*, state::BondingCurve};
use anchor_lang::prelude::*;
use chrono::Duration;

use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct StageCompleted<'info> {
    #[account(mut)]
    pub mint: Box<Account<'info, Mint>>, // the mint address of token

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

    #[account(mut)]
    pub user: Signer<'info>,
}

pub fn stage_completed(ctx: Context<StageCompleted>) -> Result<()> {
    let clock
    let accts
    let one_day 
    let current_time 
    if accts.bonding_curve.current_stage_complete == true {
        // Log the event details
        msg!(
            "MoveStageEvent - Mint: {}, BondingCurve: {}, User: {}, CurrentStage: {}",
            accts.mint.key(),
            accts.bonding_curve.key(),
            accts.user.key(),
            accts.bonding_curve.current_stage,
        );

        emit! {
            MoveStageEvent {
                mint: accts.mint.key(),
                bonding_curve: accts.bonding_curve.key(),
                user: accts.user.key(),
                current_stage: accts.bonding_curve.current_stage,
            }
        }
    } else {
        if accts.bonding_curve.current_stage > accts.bonding_curve.number_stage {
            accts.bonding_curve.complete 
            msg!(
                "Bonding Curve Complete : User: {}, Mint: {}, BondingCurve: {}, Timestamp: {}",
                accts.user.key(),
                accts.mint.key(),
                accts.bonding_curve.key(),
                clock.unix_timestamp
            );
            emit!(CompleteEvent {
                user: accts.user.key(),
                mint: accts.mint.key(),
                bonding_curve: accts.bonding_curve.key(),
                timestamp: clock.unix_timestamp,
            });
        } else {
            msg!(
                "Stage Complete : User: {}, Mint: {}, BondingCurve: {},CurrentStage:{}, Timestamp: {}",
                accts.user.key(),
                accts.mint.key(),
                accts.bonding_curve.key(),
                accts.bonding_curve.current_stage,
                clock.unix_timestamp
            );
            emit!(CompleteStageEvent {
                user: accts.user.key(),
                mint: accts.mint.key(),
                bonding_curve: accts.bonding_curve.key(),
                current_stage: accts.bonding_curve.current_stage,
                timestamp: clock.unix_timestamp,
            });
        }
    }

    Ok(())
}
