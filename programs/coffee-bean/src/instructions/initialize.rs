use crate::{constants::*,error::*,states::*, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        space = 8 + std::mem::size_of::<GlobalState>(),
        payer = authority,
    )]
    pub global_state: Account<'info, GlobalState>,
    pub token_mint: Account<'info, Mint>,

    /// CHECK: this should be set by admin
    pub treasury: AccountInfo<'info>,

    #[account(
        init,
        seeds = [VAULT_SEED],
        bump,
        payer = authority,
        space = std::mem::size_of::<TokenAccount>() + 8 
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> Initialize<'info> {
    pub fn validate(&self) -> Result<()> {
        if self.global_state.is_initialized == 1 {
            require!(
                self.global_state.authority.eq(&self.authority.key()),
                BeanError::NotAllowedAuthority
            )
        }
        Ok(())
    }
}

/// Initialize Staking Program for the first time
/// to init global state with some data for validation
///
#[access_control(ctx.accounts.validate())]
pub fn handle(ctx: Context<Initialize>, new_authority: Pubkey) -> Result<()> {
    let accts = ctx.accounts;
    accts.global_state.is_initialized = 1;
    accts.global_state.authority = new_authority;
    accts.global_state.vault = accts.vault.key();
    accts.global_state.treasury = accts.treasury.key();
    accts.global_state.token_mint = accts.token_mint.key();

    accts.global_state.market_eggs = 108000000000;
    accts.global_state.dev_fee = 300; // means 3%
    accts.global_state.psn = 10000;
    accts.global_state.psnh = 5000;
    accts.global_state.eggs_per_miner = 1080000;

    let rent = Rent::default();
    let required_lamports = rent
        .minimum_balance(0)
        .max(1)
        .saturating_sub(accts.vault.to_account_info().lamports());
    msg!("required lamports = {:?}", required_lamports);
    anchor_spl::token::transfer(
        CpiContext::new(
            accts.token_program.to_account_info(),
            Transfer {
                from: accts.authority.to_account_info(),
                to: accts.vault.to_account_info(),
                authority: accts.authority.to_account_info(),
            },
        ),
        required_lamports,
    )?;

    Ok(())
}
