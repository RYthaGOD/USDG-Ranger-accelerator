use anchor_lang::prelude::*;
use crate::state::*;

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

    msg!("Vault Config Updated");
    Ok(())
}
