use anchor_lang::prelude::*;

#[error_code]
pub enum RangerError {
    #[msg("Rebalance throttled: Time limit not reached")]
    RebalanceThrottled,

    #[msg("Slippage exceeded: shares issued lower than min_shares_out")]
    SlippageExceeded,

    #[msg("Invalid Program ID handed to instruction")]
    InvalidProgramId,

    #[msg("Invalid Oracle Account data or buffer length")]
    InvalidOracle,

    #[msg("Volume Cap Exceeded: Maximum Position Size reached")]
    VolumeCapExceeded,
}
