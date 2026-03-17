use anchor_lang::prelude::*;

#[account]
pub struct VaultConfig {
    /// Admin address (governance, parameters)
    pub admin: Pubkey,
    
    /// Manager/Cranker address (only authorized to trigger rebalances)
    pub manager: Pubkey,
    
    /// JitoSOL Mint
    pub jitosol_mint: Pubkey,
    
    /// USDG Mint
    pub usdg_mint: Pubkey,
    
    /// SOL Mint (or wrapping)
    pub sol_mint: Pubkey,
    
    /// Kamino Obligation PDA linked to this vault
    pub kamino_obligation: Pubkey,
    
    /// USDC Mint (Base Asset)
    pub usdc_mint: Pubkey,
    
    /// Max LTV before triggers. 4500 = 45%
    pub max_ltv: u16,
    
    /// Emergency de-lever LTV threshold. 5000 = 50%
    pub emergency_ltv: u16,
    
    /// Rebalance throttle in hours
    pub rebalance_throttle_hrs: u8,
    
    /// Timestamp of last rebalance
    pub last_rebalance_ts: i64,

    /// Timestamp of last config update (for timelock)
    pub last_config_update_ts: i64,
    
    /// Max allowable total JitoSOL position size (Volume Cap)
    pub max_deposit_cap: u64,
    
    /// Reserved space for future upgrades
    pub padding: [u64; 9],
}

impl VaultConfig {
    pub const SIZE: usize = 8 + // Discriminator
        32 + // admin
        32 + // manager
        32 + // jitosol_mint
        32 + // usdg_mint
        32 + // sol_mint
        32 + // kamino_obligation
        2 +  // max_ltv
        2 +  // emergency_ltv
        1 +  // rebalance_throttle_hrs
        8 +  // last_rebalance_ts
        (16 * 8); // padding (u128 * 8)
}

#[account]
pub struct PositionState {
    /// Total shares issued to depositors
    pub total_shares: u64,

    /// Amount of JitoSOL deposited in Kamino
    pub kamino_jitosol_amount: u64,
    
    /// Amount of USDG borrowed from Kamino
    pub kamino_usdg_borrowed: u64,
    
    /// Meteora LP position key/shares details
    pub meteora_lp_amount: u64,
    
    /// Drift short position size (base)
    pub drift_short_size: u64,
    
    /// Drift collateral deposited
    pub drift_collateral_amount: u64,
    
    /// Timestamp of last deposit (for 3-month lockup tracking)
    pub deposit_ts: i64,
    
    /// Reserved space
    pub padding: [u64; 7],
}

impl PositionState {
    pub const SIZE: usize = 8 + // Discriminator
        8 + // total_shares
        8 + // kamino_jitosol_amount
        8 + // kamino_usdg_borrowed
        8 + // meteora_lp_amount
        8 + // drift_short_size
        8 + // drift_collateral_amount
        (16 * 4); // padding
}
