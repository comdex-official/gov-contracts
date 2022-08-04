use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient funds were sent: {funds}")]
    InsufficientFunds { funds: u128 },

    #[error("The locking period is still active.")]
    TimeNotOvered {},

    #[error("The VToken is Allready Unlocked.")]
    AllreadyUnLocked {},

    #[error("The VToken is in Locked State.")]
    NotUnlocked {},

    #[error("The token is not in locked state")]
    NotLocked {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
