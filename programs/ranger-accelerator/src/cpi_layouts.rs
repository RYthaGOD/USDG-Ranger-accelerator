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
    
    // Read Agg Price at Offset 168 (i64)
    let price = i64::from_le_bytes(data[168..176].try_into().unwrap());
    
    let value = (price as f64) * 10f64.powi(expo);
    Ok(value)
}
