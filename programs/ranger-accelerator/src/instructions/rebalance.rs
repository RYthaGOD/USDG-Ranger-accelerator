use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::constants::*;

#[derive(Accounts)]
pub struct Rebalance<'info> {
    #[account(mut)]
    pub manager: Signer<'info>,

    #[account(
        seeds = [b"vault-config"],
        bump,
        has_one = manager // Must be the authorized manager/cranker
    )]
    pub vault_config: Account<'info, VaultConfig>,

    #[account(
        mut,
        seeds = [b"position-state"],
        bump,
    )]
    pub position_state: Account<'info, PositionState>,

    /// CHECK: Verified via constraint
    #[account(constraint = meteora_program.key() == METEORA_DLMM_PROGRAM_ID @ RangerError::InvalidProgramId)]
    pub meteora_program: AccountInfo<'info>,

    /// --- Meteora DLMM External Accounts ---
    /// CHECK: Unchecked for placeholder
    pub meteora_lb_pair: AccountInfo<'info>,
    /// CHECK: Unchecked
    pub meteora_bin_array_0: AccountInfo<'info>,
    /// CHECK: Unchecked
    pub meteora_bin_array_1: AccountInfo<'info>,
    /// CHECK: Unchecked
    pub meteora_reserve_x: AccountInfo<'info>,
    /// CHECK: Unchecked
    pub meteora_reserve_y: AccountInfo<'info>,
}

pub fn handle_rebalance(ctx: Context<Rebalance>) -> Result<()> {
    let vault_config = &ctx.accounts.vault_config;

    let current_ts = Clock::get()?.unix_timestamp;
    let throttle_seconds = (vault_config.rebalance_throttle_hrs as i64) * 3600;

    require!(
        current_ts >= vault_config.last_rebalance_ts + throttle_seconds,
        RangerError::RebalanceThrottled
    );

    msg!("Triggering Rebalance CPI to Meteora DLMM");
    
    // 1. Withdraw Liquidity from current bins
    // let withdraw_ix = meteora_dlmm::instruction::remove_liquidity(...);
    // invoke_signed(&withdraw_ix, ...)?;

    // 2. Add Liquidity to new bins (re-centered)
    // let deposit_ix = meteora_dlmm::instruction::add_liquidity(...);
    // invoke_signed(&deposit_ix, ...)?;

    // Update timestamp
    ctx.accounts.vault_config.last_rebalance_ts = current_ts;

    msg!("Rebalance complete.");
    Ok(())
}

#[derive(Accounts)]
pub struct EmergencyDeleverage<'info> {
    #[account(mut)]
    pub manager: Signer<'info>,

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

    /// CHECK: Verified via constraint
    #[account(constraint = kamino_program.key() == KAMINO_PROGRAM_ID @ RangerError::InvalidProgramId)]
    pub kamino_program: AccountInfo<'info>,
    
    /// CHECK: Verified via constraint
    #[account(constraint = meteora_program.key() == METEORA_DLMM_PROGRAM_ID @ RangerError::InvalidProgramId)]
    pub meteora_program: AccountInfo<'info>,

    /// --- Meteora DLMM External Accounts ---
    /// CHECK: Unchecked for placeholder
    pub meteora_lb_pair: AccountInfo<'info>,
    /// CHECK: Unchecked
    pub meteora_bin_array_0: AccountInfo<'info>,
    /// CHECK: Unchecked
    pub meteora_bin_array_1: AccountInfo<'info>,
    /// CHECK: Unchecked
    pub meteora_reserve_x: AccountInfo<'info>,
    /// CHECK: Unchecked
    pub meteora_reserve_y: AccountInfo<'info>,

    /// --- Kamino External Accounts ---
    /// CHECK: Unchecked
    pub kamino_reserve: AccountInfo<'info>,
    /// CHECK: Unchecked
    pub kamino_obligation: AccountInfo<'info>,
    /// CHECK: Unchecked
    pub kamino_lending_market: AccountInfo<'info>,
}

pub fn handle_emergency_deleverage(ctx: Context<EmergencyDeleverage>) -> Result<()> {
    msg!("---- EMERGENCY DE-LEVER TRIGGERED ----");
    
    let position = &mut ctx.accounts.position_state;

    // 1. Withdraw portion of Liquidity from Meteora
    msg!("CPI: Emergency Withdrawal from Meteora DLMM");
    // let withdraw_ix = meteora_dlmm::instruction::remove_liquidity(...);
    // invoke_signed(&withdraw_ix, ...)?;

    // 2. Repay portion of USDG debt in Kamino
    msg!("CPI: Emergency Repayment to Kamino");
    // let repay_ix = kamino_lending::instruction::repay_reserve_liquidity(...);
    // invoke_signed(&repay_ix, ...)?;

    // 3. Update Position State (Simplified de-lever by 20% for PoC)
    position.kamino_usdg_borrowed = (position.kamino_usdg_borrowed as u128)
        .checked_mul(80)
        .ok_or(RangerError::MathError)?
        .checked_div(100)
        .ok_or(RangerError::MathError)? as u64;

    msg!("Emergency De-lever Complete. New Debt: {:?}", position.kamino_usdg_borrowed);
    Ok(())
}
