use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{PositionState, VaultConfig};
use crate::errors::RangerError;
use crate::constants::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"vault-config"],
        bump,
    )]
    pub vault_config: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"position-state"],
        bump,
    )]
    pub position_state: UncheckedAccount<'info>,

    /// User's USDC token account (Base Asset)
    #[account(mut)]
    pub user_usdc: Account<'info, TokenAccount>,

    /// Vault's USDC token account
    #[account(mut)]
    pub vault_usdc: Account<'info, TokenAccount>,

    // --- ALL External Accounts moved to Remaining Accounts due to macro depth limits ---

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle_deposit(ctx: Context<Deposit>, amount: u64, min_shares_out: u64) -> Result<u64> {
    msg!("---- Starting Deposit Strategy ----");
    msg!("Amount JitoSOL: {:?}", amount);

    // --- 0. Static Oracle Reading ---
    // Read from remaining_accounts due to macro depth limits
    let oracle_sol_price = &ctx.remaining_accounts[0];
    let sol_price = crate::cpi_layouts::parse_pyth_price(oracle_sol_price)?;
    msg!("SOL Price from Pyth: ${:.2}", sol_price);

    // --- 0. Manual Deserialization backends ---
    let position_info = ctx.accounts.position_state.to_account_info();
    let mut position_data = position_info.try_borrow_mut_data()?;
    let mut position = PositionState::try_deserialize(&mut &position_data[..])?;

    let config_info = ctx.accounts.vault_config.to_account_info();
    let config_data = config_info.try_borrow_data()?;
    let config = VaultConfig::try_deserialize(&mut &config_data[..])?;

    // --- 0. Share Math (Price Guard / Inflation Protect) ---
    let current_total_shares = position.total_shares;
    let current_jitosol = position.kamino_jitosol_amount;

    let shares = if current_total_shares == 0 {
        amount // 1:1 on first deposit
    } else {
        (amount as u128)
            .checked_mul(current_total_shares as u128)
            .unwrap()
            .checked_div(current_jitosol as u128)
            .unwrap() as u64
    };

    msg!("Shares calculation: {:?}", shares);
    require!(shares >= min_shares_out, RangerError::SlippageExceeded);

    // --- 0. Volume Cap Verification ---
    let current_jitosol_cap = position.kamino_jitosol_amount;
    require!(
        current_jitosol_cap.checked_add(amount).unwrap() <= config.max_deposit_cap,
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
    let borrow_amount = calculate_borrow_amount(amount); // Placeholder math
    borrow_from_kamino(&ctx, borrow_amount)?;

    // 5. Add Liquidity to Meteora DLMM (USDG-SOL)
    add_liquidity_meteora(&ctx, borrow_amount)?;

    // 6. Open Short on Drift via CPI
    open_short_drift(&ctx, amount)?; // Hedge amount linked to SOL deposited

    // Update State (Using top-level deserialized reference)

    position.kamino_jitosol_amount = position.kamino_jitosol_amount.checked_add(amount).unwrap();
    position.kamino_usdg_borrowed = position.kamino_usdg_borrowed.checked_add(borrow_amount).unwrap();
    position.total_shares = position.total_shares.checked_add(shares).unwrap();
    
    // Update absolute quantities (Reconciliation)
    position.meteora_lp_amount = position.meteora_lp_amount.checked_add(borrow_amount).unwrap();
    position.drift_short_size = position.drift_short_size.checked_add(amount).unwrap();
    position.drift_collateral_amount = position.drift_collateral_amount.checked_add(borrow_amount).unwrap();
    position.deposit_ts = Clock::get()?.unix_timestamp;

    // Serialize back
    let mut writer = &mut position_data[..];
    position.serialize(&mut writer)?;

    msg!("Deposit Strategy Complete. Shares Issued: {:?}", shares);
    Ok(shares)
}

fn supply_to_kamino(ctx: &Context<Deposit>, amount: u64) -> Result<()> {
    msg!("CPI: Supplying JitoSOL to Kamino");
    // TODO: Build Kamino Deposit Instruction and invoke_signed
    Ok(())
}

fn refresh_kamino_obligation(ctx: &Context<Deposit>) -> Result<()> {
    msg!("CPI: Refreshing Kamino Obligation");
    Ok(())
}

fn borrow_from_kamino(ctx: &Context<Deposit>, amount: u64) -> Result<()> {
    msg!("CPI: Borrowing USDG from Kamino: {:?}", amount);
    Ok(())
}

fn add_liquidity_meteora(ctx: &Context<Deposit>, amount: u64) -> Result<()> {
    msg!("CPI: Adding Liquidity to Meteora DLMM: {:?}", amount);
    Ok(())
}

fn open_short_drift(ctx: &Context<Deposit>, base_amount: u64) -> Result<()> {
    msg!("CPI: Opening Short on Drift: {:?}", base_amount);
    Ok(())
}

fn calculate_borrow_amount(deposit_amount: u64) -> u64 {
    // Target 45% LTV. Placeholder math.
    // Assuming 1 JitoSOL = 1 USDG for simple multiplication
    (deposit_amount * 45) / 100
}
