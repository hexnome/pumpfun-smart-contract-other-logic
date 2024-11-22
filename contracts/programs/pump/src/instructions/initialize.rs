use anchor_lang::prelude::*;

use crate::{
    constants::{GLOBAL_STATE_SEED, REWARD_STATE_SEED},
    state::Global,
    error::*,
};
use std::mem::size_of;
use std::str::FromStr;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = owner, 
        seeds = [GLOBAL_STATE_SEED],
        bump,
        space = 8 + size_of::<Global>()
    )]
    pub global: Box<Account<'info, Global>>,

    #[account(
        init, 
        payer = owner, 
        seeds = [REWARD_STATE_SEED],
        bump,
        space = 0
    )]
    pub reward_recipient:AccountInfo<'info>,

   
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

    let global = &mut ctx.accounts.global;

    // this pubkey has to be set by admin
    let EXPECTED_OWNER = Pubkey::from_str("DmK2k1vXApWgkP1yhcKhnjqkVD79x8VHFVRMqNZzK4WH").unwrap();

    require_keys_eq!(ctx.accounts.owner.key(), EXPECTED_OWNER.key(), pumpCode::NotAuthorized);

    require!(global.initialized == false, pumpCode::AlreadyInitialized);
    
    global.authority = ctx.accounts.owner.key();
    global.initialized = true;
    
    Ok(())
}
