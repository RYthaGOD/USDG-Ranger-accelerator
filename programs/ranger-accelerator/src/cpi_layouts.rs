use anchor_lang::prelude::*;

pub fn parse_pyth_price(account: &AccountInfo) -> Result<f64> {
    let data = account.try_borrow_data()?;
    if data.len() < 200 { 
        return err!(crate::errors::RangerError::InvalidOracle); 
    }
    
    // Check Pyth Magic Number (0xa1b2c3d4) At Offset 0
    let magic = u32::from_le_bytes(data[0..4].try_into().unwrap());
    if magic != 0xa1b2c3d4 {
        return err!(crate::errors::RangerError::InvalidOracle);
    }
    
    // Read Exponent at Offset 20 (i32)
    let expo = i32::from_le_bytes(data[20..24].try_into().unwrap());
    
    // Read Publish Time at Offset 40 (i64)
    let publish_time = i64::from_le_bytes(data[40..48].try_into().unwrap());
    let current_ts = Clock::get()?.unix_timestamp;
    
    // Staleness Check: 60 seconds max
    if current_ts - publish_time > 60 {
        msg!("Oracle Price Stale: {}s lag", current_ts - publish_time);
        return err!(crate::errors::RangerError::InvalidOracle);
    }
    
    // Read Confidence at Offset 152 (u64)
    let conf = u64::from_le_bytes(data[152..160].try_into().unwrap());
    
    // Read Agg Price at Offset 168 (i64)
    let price = i64::from_le_bytes(data[168..176].try_into().unwrap());
    
    // Confidence Check: conf / price < 2%
    if (conf as f64) / (price as f64).abs() > 0.02 {
        msg!("Oracle Confidence interval too wide: {:.2}%", (conf as f64) / (price as f64).abs() * 100.0);
        return err!(crate::errors::RangerError::InvalidOracle);
    }
    
    let value = (price as f64) * 10f64.powi(expo);
    Ok(value)
}
