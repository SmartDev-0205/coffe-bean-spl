use crate::{constants::*, error::*, states::*, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, Transfer};

use std::mem::size_of;
#[derive(Accounts)]
pub struct BuyEggs<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      mut,
      seeds = [GLOBAL_STATE_SEED],
      bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut)]
    /// CHECK: this should be set by admin
    pub treasury: AccountInfo<'info>,
    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    /// CHECK: this should be set by admin
    pub vault: AccountInfo<'info>,

    #[account(
        init,
        seeds = [USER_STATE_SEED, user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + size_of::<UserState>()
    )]
    pub user_state: Account<'info, UserState>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handle(ctx: Context<BuyEggs>, sol_amount: u64) -> Result<()> {
    let accts = ctx.accounts;

    let cur_timestamp = Clock::get()?.unix_timestamp;
    if accts.user_state.is_initialized == 0 {
        accts.user_state.is_initialized = 1;
        accts.user_state.last_hatch_time = cur_timestamp as u64;
        accts.user_state.user = accts.user.key();
    } else {
        require!(
            accts.user_state.user.eq(&accts.user.key()),
            BeanError::IncorrectUserState
        );
    }

    let mut eggs_bought =
        calculate_eggs_buy(&accts.global_state, sol_amount, accts.vault.lamports())?;

    let eggs_bought_fee = dev_fee(&accts.global_state, eggs_bought)?;
    eggs_bought = eggs_bought.checked_sub(eggs_bought_fee).unwrap();

    let sol_fee = dev_fee(&accts.global_state, sol_amount)?;
    let real_sol = sol_amount.checked_sub(sol_fee).unwrap();

    anchor_spl::token::transfer(
        CpiContext::new(
            accts.token_program.to_account_info(),
            Transfer {
                from: accts.user.to_account_info(),
                to: accts.treasury.to_account_info(),
                authority: accts.user.to_account_info(),
            },
        ),
        sol_fee,
    )?;

    anchor_spl::token::transfer(
        CpiContext::new(
            accts.token_program.to_account_info(),
            Transfer {
                from: accts.user.to_account_info(),
                to: accts.vault.to_account_info(),
                authority: accts.user.to_account_info(),
            },
        ),
        real_sol,
    )?;



    accts.user_state.claimed_eggs = accts
        .user_state
        .claimed_eggs
        .checked_add(eggs_bought)
        .unwrap();
    Ok(())
}
