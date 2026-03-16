use anchor_client::{Client, Cluster, anchor_lang::solana_program::pubkey::Pubkey};
use solana_sdk::signature::{Keypair, read_keypair_file};
use reqwest::Client as HttpClient;
use std::rc::Rc;
use std::time::Duration;
use tokio::time::sleep;

use ranger_accelerator::state::{PositionState, VaultConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Ranger Accelerator Cranker Bot...");

    // 1. Initialize Clients
    // Load manager keypair from the fixtures directory we generated during testing
    let manager_keypair_path = "../tests/fixtures/manager.json"; 
    let manager = read_keypair_file(manager_keypair_path)
        .expect("Failed to read manager keypair file at ../tests/fixtures/manager.json");
    
    let payer = Rc::new(manager);
    let rpc_url = "http://localhost:8899"; // Localnet Default for testing
    
    let client = Client::new(
        Cluster::Custom(rpc_url.to_string(), "ws://127.0.0.1:8900".to_string()), 
        payer.clone()
    );
    
    let program_id = ranger_accelerator::ID; // Load program ID dynamically from the compiled crate!
    let program = client.program(program_id)?;

    let http_client = HttpClient::new();

    // Derive PDAs
    let (position_state_pda, _bump) = Pubkey::find_program_address(
        &[b"position-state"],
        &program_id,
    );
    
    let (vault_config_pda, _bump) = Pubkey::find_program_address(
        &[b"vault-config"],
        &program_id,
    );

    loop {
        println!("\n*** CRANK CYCLE ***");
        
        // A. Fetch AetherIndex Live Data (SOL Price)
        let sol_price = fetch_aether_price(&http_client).await.unwrap_or(205.50);
        println!("SOL Price from AetherIndex: ${:.2}", sol_price);

        // B. Fetch On-chain Vault State
        let vault_state: PositionState = match program.account::<PositionState>(position_state_pda) {
            Ok(acc) => acc,
            Err(e) => {
                println!("Failed to load PositionState Account: {:?}", e);
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        };

        let config_state: VaultConfig = match program.account::<VaultConfig>(vault_config_pda) {
            Ok(acc) => acc,
            Err(e) => {
                println!("Failed to load VaultConfig Account: {:?}", e);
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        };

        // Calculate LTV
        let total_borrowed = vault_state.kamino_usdg_borrowed as f64;
        let jitosol_qty = vault_state.kamino_jitosol_amount as f64;
        let total_collateral_value = jitosol_qty * sol_price; 
        
        let ltv = if total_collateral_value > 0.0 {
            (total_borrowed / total_collateral_value) * 100.0
        } else {
            0.0
        };

        println!("PositionState Stats:");
        println!(" - JitoSOL Collateral: {:.4}", jitosol_qty);
        println!(" - USDG Borrowed: {:.2}", total_borrowed);
        println!(" - Calculated LTV: {:.2}%", ltv);

        // C. Decision Matrix
        if ltv >= 50.0 {
            println!("🚨 LTV too high ({:.2}%)! Triggering Emergency De-lever...", ltv);
            match trigger_emergency_deleverage(&program, position_state_pda, vault_config_pda).await {
                Ok(sig) => println!("✅ Emergency De-lever Signature: {:?}", sig),
                Err(e) => println!("❌ Emergency De-lever Failed: {:?}", e),
            }
        } else if ltv >= 45.0 {
            println!("⚠️ LTV approaching risk limit ({:.2}%)!", ltv);
        }

        // Rebalance Check (Throttle)
        let current_ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64;
        let throttle_seconds = (config_state.rebalance_throttle_hrs as i64) * 3600;

        if current_ts >= config_state.last_rebalance_ts + throttle_seconds {
             println!("🔧 Triggering Rebalance CPI to Meteora DLMM...");
             match trigger_rebalance(&program, position_state_pda, vault_config_pda).await {
                 Ok(sig) => println!("✅ Rebalance Signature: {:?}", sig),
                 Err(e) => println!("❌ Rebalance Failed: {:?}", e),
             }
        } else {
             println!("⏳ Rebalance throttled. Next allowed in {} hrs", config_state.rebalance_throttle_hrs);
        }

        sleep(Duration::from_secs(30)).await;
    }
}

async fn fetch_aether_price(client: &HttpClient) -> Result<f64, reqwest::Error> {
    // Placeholder URI for AetherIndex Price Endpoint
    let _url = "https://api.aetherindex.com/v1/price?symbol=SOL";
    // In production, execute call:
    // let res = client.get(url).send().await?.json::<PriceResponse>().await?;
    // Ok(res.price)
    Ok(205.50) // Mocked return for structural compliance
}

async fn trigger_rebalance(
    program: &anchor_client::Program<std::rc::Rc<Keypair>>,
    position_state: Pubkey,
    vault_config: Pubkey,
) -> Result<solana_sdk::signature::Signature, anchor_client::ClientError> {
    // Static / Mock Placeholders for testing
    let dummy_account = Pubkey::new_from_array([1; 32]);
    let meteora_program = Pubkey::new_from_array([10; 32]); // Placeholders trigger no crash on localnet if not loaded

    program.request()
        .accounts(ranger_accelerator::accounts::Rebalance {
            manager: program.payer(),
            vault_config: vault_config,
            position_state: position_state,
            meteora_program: meteora_program, // Needs constant matching
            meteora_lb_pair: dummy_account,
            meteora_bin_array_0: dummy_account,
            meteora_bin_array_1: dummy_account,
            meteora_reserve_x: dummy_account,
            meteora_reserve_y: dummy_account,
        })
        .args(ranger_accelerator::instruction::Rebalance {})
        .send()
}

async fn trigger_emergency_deleverage(
    program: &anchor_client::Program<std::rc::Rc<Keypair>>,
    position_state: Pubkey,
    vault_config: Pubkey,
) -> Result<solana_sdk::signature::Signature, anchor_client::ClientError> {
    let dummy_account = Pubkey::new_from_array([1; 32]);
    let kamino_program = Pubkey::new_from_array([11; 32]);
    let meteora_program = Pubkey::new_from_array([10; 32]);

    program.request()
        .accounts(ranger_accelerator::accounts::EmergencyDeleverage {
            manager: program.payer(),
            vault_config: vault_config,
            position_state: position_state,
            kamino_program: kamino_program,
            meteora_program: meteora_program,
            meteora_lb_pair: dummy_account,
            meteora_bin_array_0: dummy_account,
            meteora_bin_array_1: dummy_account,
            meteora_reserve_x: dummy_account,
            meteora_reserve_y: dummy_account,
            kamino_reserve: dummy_account,
            kamino_obligation: dummy_account,
            kamino_lending_market: dummy_account,
        })
        .args(ranger_accelerator::instruction::EmergencyDeleverage {})
        .send()
}
