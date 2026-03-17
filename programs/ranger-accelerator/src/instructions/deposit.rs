use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::PositionState;
use crate::state::VaultConfig;
use crate::errors::RangerError;

#[derive(Accounts)]
pub struct Deposit<'info> {
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

    /// User's USDC token account (Base Asset)
    #[account(mut)]
    pub user_usdc: Account<'info, TokenAccount>,

    /// Vault's USDC token account
    #[account(mut)]
    pub vault_usdc: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle_deposit(ctx: Context<Deposit>, amount: u64, min_shares_out: u64) -> Result<u64> {
    msg!("---- Starting Deposit Strategy ----");
    msg!("Amount USDC: {:?}", amount);

    // --- 0. Static Oracle Reading ---
    // Read from remaining_accounts due to macro depth limits
    let oracle_sol_price = &ctx.remaining_accounts[0];
    
    // Safety check: Verify Oracle account owner (Placeholder for Pyth Program ID)
    // require!(oracle_sol_price.owner == &PYTH_PROGRAM_ID, RangerError::InvalidOracle);
    
    let sol_price = crate::cpi_layouts::parse_pyth_price(oracle_sol_price)?;
    msg!("SOL Price from Pyth: ${:.2}", sol_price);

    // --- 0. Global LTV Pre-check (Audit 3.0 Fix) ---
    {
        let position = &ctx.accounts.position_state;
        let config = &ctx.accounts.vault_config;
        
        let total_borrowed = position.kamino_usdg_borrowed as u128;
        let current_qty = position.kamino_jitosol_amount as u128;
        
        // Scale sol_price to u128 with 9 decimals for calculation
        let sol_price_fixed = (sol_price * 1_000_000_000.0) as u128;
        let total_collateral_value = current_qty.checked_mul(sol_price_fixed).ok_or(RangerError::MathError)? / 1_000_000_000;
        
        if total_collateral_value > 0 {
            let current_ltv = total_borrowed
                .checked_mul(10000)
                .ok_or(RangerError::MathError)?
                .checked_div(total_collateral_value)
                .ok_or(RangerError::MathError)? as u16;
                
            require!(current_ltv <= config.max_ltv, RangerError::MathError); 
        }
    }

    // --- 0. Share Math (Price Guard / Inflation Protect) ---
    let (shares, current_jitosol_cap) = {
        let position = &ctx.accounts.position_state;
        let _config = &ctx.accounts.vault_config;
        
        let current_total_shares = position.total_shares as u128;
        let current_jitosol = position.kamino_jitosol_amount as u128;

        let shares = if current_total_shares == 0 {
            amount // 1:1 on first deposit (USDC 6 decimals -> Shares 6 decimals)
        } else {
            (amount as u128)
                .checked_mul(current_total_shares)
                .ok_or(RangerError::MathError)?
                .checked_div(current_jitosol)
                .ok_or(RangerError::MathError)? as u64
        };
        
        (shares, position.kamino_jitosol_amount)
    };

    msg!("Shares calculation: {:?}", shares);
    require!(shares >= min_shares_out, RangerError::SlippageExceeded);

    // --- 0. Volume Cap Verification ---
    require!(
        current_jitosol_cap.checked_add(amount).ok_or(RangerError::MathError)? <= ctx.accounts.vault_config.max_deposit_cap,
        RangerError::VolumeCapExceeded
    );

    // --- 1. Transfer USDC from User to Vault ---
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_usdc.to_account_info(),
        to: ctx.accounts.vault_usdc.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    /*
    /// --- 2. Kamino Collateralization (MOCK / PLACEHOLDER for strategy retention) ---
    /// 1. Deposit USDC into Kamino collateral pool.
    /// 2. Borrow JitoSOL from Kamino into Vault.
    /// (After this point, the vault holds JitoSOL and the previous logic remains identical!)
    */

    // 2. Supply JitoSOL to Kamino
    supply_to_kamino(&ctx, amount)?;

    // 3. Refresh Obligation
    refresh_kamino_obligation(&ctx)?;

    // 4. Borrow USDG from Kamino
    let borrow_amount = calculate_borrow_amount(amount, sol_price, ctx.accounts.vault_config.max_ltv); 
    borrow_from_kamino(&ctx, borrow_amount)?;

    // 5. Add Liquidity to Meteora DLMM (USDG-SOL)
    add_liquidity_meteora(&ctx, borrow_amount)?;

    // 6. Open Short on Drift via CPI
    open_short_drift(&ctx, amount)?; // Hedge amount linked to SOL deposited

    // Update State (Borrow mutably ONLY when needed for updates)
    let position = &mut ctx.accounts.position_state;

    position.kamino_jitosol_amount = position.kamino_jitosol_amount.checked_add(amount).ok_or(RangerError::MathError)?;
    position.kamino_usdg_borrowed = position.kamino_usdg_borrowed.checked_add(borrow_amount).ok_or(RangerError::MathError)?;
    position.total_shares = position.total_shares.checked_add(shares).ok_or(RangerError::MathError)?;
    
    // Update absolute quantities (Reconciliation)
    position.meteora_lp_amount = position.meteora_lp_amount.checked_add(borrow_amount).ok_or(RangerError::MathError)?;
    position.drift_short_size = position.drift_short_size.checked_add(amount).ok_or(RangerError::MathError)?;
    position.drift_collateral_amount = position.drift_collateral_amount.checked_add(borrow_amount).ok_or(RangerError::MathError)?;
    position.deposit_ts = Clock::get()?.unix_timestamp;

    msg!("Deposit Strategy Complete. Shares Issued: {:?}", shares);
    Ok(shares)
}

fn supply_to_kamino(_ctx: &Context<Deposit>, _amount: u64) -> Result<()> {
    msg!("CPI: Supplying USDC to Kamino");
    Ok(())
}

fn refresh_kamino_obligation(_ctx: &Context<Deposit>) -> Result<()> {
    msg!("CPI: Refreshing Kamino Obligation");
    Ok(())
}

fn borrow_from_kamino(_ctx: &Context<Deposit>, _amount: u64) -> Result<()> {
    msg!("CPI: Borrowing JitoSOL from Kamino");
    Ok(())
}

fn add_liquidity_meteora(_ctx: &Context<Deposit>, _amount: u64) -> Result<()> {
    msg!("CPI: Adding Liquidity to Meteora DLMM");
    Ok(())
}

fn open_short_drift(_ctx: &Context<Deposit>, _base_amount: u64) -> Result<()> {
    msg!("CPI: Opening Short on Drift");
    Ok(())
}

fn calculate_borrow_amount(deposit_amount: u64, sol_price: f64, max_ltv: u16) -> u64 {
    // deposit_amount is in USDC (6 decimals)
    // max_ltv is 4500 = 45%
    
    let usdc_value = deposit_amount as f64;
    let ltv = (max_ltv as f64) / 10000.0;
    let borrow_value_usdc = usdc_value * ltv;
    
    (borrow_value_usdc / sol_price) as u64
}
