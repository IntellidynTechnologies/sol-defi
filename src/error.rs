
use borsh::{ BorshSerialize, BorshDeserialize };

#[derive(BorshDeserialize, BorshSerialize)]
pub enum CustomError {
    /// Zero Liquidity
    ZeroLiquidity,
    /// Zero Amount
    ZeroAmount,
    /// Insufficient Amount
    InsufficientAmount,
    /// Non Equivalent Value
    NonEquivalientValue,
    /// Threshold Not Reached
    ThresholdNotReached,
    /// Invalid Share
    InvalidShare,
    /// Insufficient Liquidity
    InsufficientLiquidity,
    /// Slippage Exceeded
    SlippageExceeded,
}