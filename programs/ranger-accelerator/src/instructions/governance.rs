use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault-config"],
        bump,
        has_one = admin // Only current admin can call
    )]
    pub vault_config: Account<'info, VaultConfig>,
}

pub fn handle_update_config(
    ctx: Context<UpdateConfig>,
    new_manager: Option<Pubkey>,
    new_max_ltv: Option<u16>,
    new_emergency_ltv: Option<u16>,
    new_rebalance_throttle_hrs: Option<u8>,
) -> Result<()> {
    let vault_config = &mut ctx.accounts.vault_config;
    let current_ts = Clock::get()?.unix_timestamp;

    // Programmatic Timelock: 24 hours between sensitive updates
    if vault_config.last_config_update_ts > 0 {
        require!(
            current_ts >= vault_config.last_config_update_ts + 86400,
            crate::errors::RangerError::RebalanceThrottled // Reusing throttle error for PoC
        );
    }

    if let Some(manager) = new_manager {
        vault_config.manager = manager;
    }
    if let Some(max_ltv) = new_max_ltv {
        vault_config.max_ltv = max_ltv;
    }
    if let Some(emergency_ltv) = new_emergency_ltv {
        vault_config.emergency_ltv = emergency_ltv;
    }
    if let Some(throttle) = new_rebalance_throttle_hrs {
        vault_config.rebalance_throttle_hrs = throttle;
    }

    vault_config.last_config_update_ts = current_ts;

    msg!("Vault Config Updated at: {:?}", current_ts);
    Ok(())
}

#[derive(Accounts)]
pub struct InitKaminoObligation<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault-config"],
        bump,
        has_one = admin,
    )]
    pub vault_config: Account<'info, VaultConfig>,

    /// CHECK: The obligation account to be initialized on Kamino
    #[account(mut)]
    pub kamino_obligation: AccountInfo<'info>,

    /// CHECK: The Kamino lending market account
    pub kamino_lending_market: AccountInfo<'info>,

    /// CHECK: Verified via constraint in Constants
    #[account(constraint = kamino_program.key() == KAMINO_PROGRAM_ID)]
    pub kamino_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_init_obligation(ctx: Context<InitKaminoObligation>) -> Result<()> {
    let vault_config = &mut ctx.accounts.vault_config;
    
    msg!("CPI: Initializing Kamino Obligation");
    
    // In production, this would be a CPI to Kamino:
    // kamino_lending::instruction::init_obligation(
    //     ...
    //     vault_config.key(), // Owner
    // )

    vault_config.kamino_obligation = ctx.accounts.kamino_obligation.key();
    
    msg!("Obligation Set: {:?}", vault_config.kamino_obligation);
    Ok(())
}
