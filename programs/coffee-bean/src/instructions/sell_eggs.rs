use crate::{constants::*, error::*,states::*, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, Transfer};
#[derive(Accounts)]
pub struct SellEggs<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      mut,
      seeds = [GLOBAL_STATE_SEED],
      bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    /// CHECK: this should be checked with address in global_state
    pub vault: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: this should be checked with address in global_state
    pub treasury: AccountInfo<'info>,
    /// CHECK: we read this key only
    pub token_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [USER_STATE_SEED, user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub user_state: Account<'info, UserState>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> SellEggs<'info> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handle(ctx: Context<SellEggs>) -> Result<()> {
    let cur_timestamp = Clock::get()?.unix_timestamp as u64;
    let accts = ctx.accounts;

    msg!("SellEggs claimed eggs {}", accts.user_state.claimed_eggs);
    let has_eggs = accts
        .user_state
        .claimed_eggs
        .checked_add(get_eggs_since_last_hatch(
            &accts.user_state,
            cur_timestamp,
            accts.global_state.eggs_per_miner,
        )?)
        .unwrap();

    msg!("SellEggs has_eggs {}", has_eggs);
    let egg_value = calculate_eggs_sell(&accts.global_state, has_eggs, accts.vault.lamports())?;

    let fee = dev_fee(&accts.global_state, egg_value)?;
    accts.user_state.claimed_eggs = 0;
    accts.user_state.last_hatch_time = cur_timestamp;
    accts.global_state.market_eggs = accts
        .global_state
        .market_eggs
        .checked_add(has_eggs)
        .unwrap();

    msg!("SellEggs selling egg_value {}", egg_value);
    msg!("SellEggs selling fee {}", fee);
    let real_val = egg_value.checked_sub(fee).unwrap();

    // send fee to treasury
    let bump = ctx.bumps.vault;
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            accts.token_program.to_account_info(),
            Transfer {
                from: accts.vault.to_account_info(),
                to: accts.treasury.to_account_info(),
                authority: accts.user.to_account_info(),
            },  
            &[&[VAULT_SEED, &[bump]]],
        ),
        fee,
    )?;


    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            accts.token_program.to_account_info(),
            Transfer {
                from: accts.vault.to_account_info(),
                to: accts.user.to_account_info(),
                authority: accts.user.to_account_info(),
            },
            &[&[VAULT_SEED, &[bump]]],
        ),
        real_val,
    )?;



    // lamports should be bigger than zero to prevent rent exemption
    let rent = Rent::default();
    let required_lamports = rent
        .minimum_balance(0)
        .max(1)
        .saturating_sub(accts.vault.to_account_info().lamports());
    require!(
        **accts.vault.lamports.borrow() > required_lamports,
        BeanError::InsufficientAmount
    );
    Ok(())
}
