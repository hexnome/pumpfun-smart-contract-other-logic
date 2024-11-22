use anchor_lang::prelude::*;

#[error_code]
pub enum pumpCode {
    #[msg("The given account is not authorized to execute this instruction.")]
    NotAuthorized,

    #[msg("The given account is not valid fee recipient.")]
    InValidFeeRecipient,

    #[msg("The program is already initialized.")]
    AlreadyInitialized,

    #[msg("slippage: Too much SOL required to buy the given amount of tokens.")]
    TooMuchSolRequired,

    #[msg("slippage: Too little SOL received to sell the given amount of tokens.")]
    TooLittleSolReceived,

    #[msg("The mint does not match the bonding curve.")]
    MintDoesNotMatchBondingCurve,

    #[msg("The bonding curve has completed and liquidity migrated to raydium.")]
    BondingCurveComplete,

    #[msg("The bonding curve has not completed.")]
    BondingCurveNotComplete,

    #[msg("The program is not initialized.")]
    NotInitialized,

    #[msg("Math operation overflow.")]
    MathOverflow,

    #[msg("Amount should be bigger than 0.")]
    ZeroAmount,

    #[msg("Amount is invalid to create the pool.")]
    InvalidAmount,

    #[msg("Supply is invalid to create the pool.")]
    InvalidSupply,

    #[msg("Freeze authority enabled.")]
    FreezeAuthorityEnabled,

    #[msg("Mint authority enabled.")]
    MitAuthorityEnabled,

    #[msg("TaxPercentage invalid.")]
    InvalidTaxPercentage,

    #[msg("FeeBasisPoints invalid.")]
    InvalidFeeBasisPoints,

    #[msg("The Current stage has completed.")]
    CurrentStaeComplete,

    #[msg("The Token is insufficient.")]
    InsufficientFunds,

    #[msg("Now, Next stage doesn't start.")]
    NotMoveNextStage,

    #[msg("Now, current stage has not completeted.")]
    NotStageComplete,

    #[msg("Now, Token pool destination is incorrect.")]
    IncorrectPoolDestination,
}
