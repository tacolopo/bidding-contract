use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("No Funds")]
    NoFunds {},

    #[error("Bid is too low")]
    BidTooLow {},

    #[error("Contract is closed")]
    ContractClosed {},

    #[error("Cannot withdraw when contract still open")]
    CannotWithdraw{},
}
