use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = VaultConfig::SIZE,
        seeds = [b"vault-config"],
        bump
    )]
    pub vault_config: Account<'info, VaultConfig>,

    pub system_program: Program<'info, System>,
}

pub fn handle(
    ctx: Context<Initialize>,
    manager: Pubkey,
    jitosol_mint: Pubkey,
    usdg_mint: Pubkey,
    sol_mint: Pubkey,
    usdc_mint: Pubkey,
) -> Result<()> {
    let vault_config = &mut ctx.accounts.vault_config;
    
    vault_config.admin = ctx.accounts.admin.key();
    vault_config.manager = manager;
    vault_config.jitosol_mint = jitosol_mint;
    vault_config.usdg_mint = usdg_mint;
    vault_config.sol_mint = sol_mint;
    vault_config.usdc_mint = usdc_mint;
    vault_config.kamino_obligation = Pubkey::default(); // Will be set during first deposit or init_obligation
    vault_config.max_ltv = 4500; // 45%
    vault_config.emergency_ltv = 5000; // 50%
    vault_config.rebalance_throttle_hrs = 4;
    vault_config.last_rebalance_ts = 0;
    vault_config.max_deposit_cap = 10_000 * 1_000_000_000; // 10k JitoSOL default cap

    msg!("Vault Config Initialized. Admin: {:?}", vault_config.admin);
    Ok(())
}
