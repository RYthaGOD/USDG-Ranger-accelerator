use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{PositionState, VaultConfig};
use crate::errors::RangerError;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"vault-config"],
        bump,
    )]
    pub vault_config: Account<'info, VaultConfig>,

    #[account(
        mut,
        seeds = [b"position-state"],
        bump,
    )]
    pub position_state: Account<'info, PositionState>,

    /// User's USDC token account
    #[account(mut)]
    pub user_usdc: Account<'info, TokenAccount>,

    /// Vault's USDC token account
    #[account(mut)]
    pub vault_usdc: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handle_withdraw(ctx: Context<Withdraw>, shares: u64, min_usdc_out: u64) -> Result<()> {
    msg!("---- Starting Withdrawal Strategy ----");
    
    // 1. Calculate underlying amount using immutable borrow
    let amount_jitosol = {
        let position = &ctx.accounts.position_state;
        let current_total_shares = position.total_shares;
        let current_jitosol = position.kamino_jitosol_amount;

        (shares as u128)
            .checked_mul(current_jitosol as u128)
            .ok_or(RangerError::MathError)?
            .checked_div(current_total_shares as u128)
            .ok_or(RangerError::MathError)? as u64
    };

    msg!("Unwinding JitoSOL amount: {:?}", amount_jitosol);

    // 2. Unwind Sequence
    unwind_meteora(&ctx, amount_jitosol)?;
    repay_kamino(&ctx, amount_jitosol)?;
    withdraw_jitosol_from_kamino(&ctx, amount_jitosol)?;

    // 3. Return Base USDC to user (Placeholder for LP -> USDC swap)
    let usdc_to_return = amount_jitosol; // Simplified 1:1 for PoC
    
    require!(usdc_to_return >= min_usdc_out, RangerError::SlippageExceeded);

    let seeds = &[
        b"vault-config".as_ref(),
        &[ctx.bumps.vault_config],
    ];
    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_usdc.to_account_info(),
        to: ctx.accounts.user_usdc.to_account_info(),
        authority: ctx.accounts.vault_config.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::transfer(cpi_ctx, usdc_to_return)?;
    
    // 4. Update State (Borrow mutably ONLY when needed for updates)
    let position = &mut ctx.accounts.position_state;

    position.total_shares = position.total_shares.checked_sub(shares).ok_or(RangerError::MathError)?;
    position.kamino_jitosol_amount = position.kamino_jitosol_amount.checked_sub(amount_jitosol).ok_or(RangerError::MathError)?;
    position.kamino_usdg_borrowed = position.kamino_usdg_borrowed.checked_sub(amount_jitosol).ok_or(RangerError::MathError)?; // Simplified
    
    msg!("Withdrawal Strategy Complete. Shares Burned: {:?}", shares);
    Ok(())
}

fn unwind_meteora(_ctx: &Context<Withdraw>, _amount: u64) -> Result<()> {
    msg!("CPI: Unwinding Meteora DLMM");
    Ok(())
}

fn repay_kamino(_ctx: &Context<Withdraw>, _amount: u64) -> Result<()> {
    msg!("CPI: Repaying Kamino Debt");
    Ok(())
}

fn withdraw_jitosol_from_kamino(_ctx: &Context<Withdraw>, _amount: u64) -> Result<()> {
    msg!("CPI: Withdrawing Collateral from Kamino");
    Ok(())
}
