use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod constants;
pub mod errors;
pub mod cpi_layouts;

pub use state::*;
pub use instructions::*;
pub use errors::*;

declare_id!("BioV76fRvp5XR1NL72GUYiu3xqAzVXso6vwBP56ghBmc");

#[program]
pub mod ranger_accelerator {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        manager: Pubkey,
        jitosol_mint: Pubkey,
        usdg_mint: Pubkey,
        sol_mint: Pubkey,
        usdc_mint: Pubkey,
    ) -> Result<()> {
        instructions::initialize::handle(ctx, manager, jitosol_mint, usdg_mint, sol_mint, usdc_mint)
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        new_manager: Option<Pubkey>,
        new_max_ltv: Option<u16>,
        new_emergency_ltv: Option<u16>,
        new_rebalance_throttle_hrs: Option<u8>,
    ) -> Result<()> {
        instructions::governance::handle_update_config(ctx, new_manager, new_max_ltv, new_emergency_ltv, new_rebalance_throttle_hrs)
    }

    pub fn init_kamino_obligation(ctx: Context<InitKaminoObligation>) -> Result<()> {
        instructions::governance::handle_init_obligation(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, min_shares_out: u64) -> Result<u64> {
        instructions::deposit::handle_deposit(ctx, amount, min_shares_out)
    }

    pub fn rebalance(ctx: Context<Rebalance>) -> Result<()> {
        instructions::rebalance::handle_rebalance(ctx)
    }

    pub fn emergency_deleverage(ctx: Context<EmergencyDeleverage>) -> Result<()> {
        instructions::rebalance::handle_emergency_deleverage(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>, shares: u64, min_usdc_out: u64) -> Result<()> {
        instructions::withdrawal::handle_withdraw(ctx, shares, min_usdc_out)
    }
}
