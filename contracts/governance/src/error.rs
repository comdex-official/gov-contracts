use cosmwasm_std::StdError;
use cw_utils::ThresholdError;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Proposal is not eligible")]
    ProposalNotEligible {},

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Threshold(#[from] ThresholdError),

    #[error("Required weight cannot be zero")]
    ZeroWeight {},

    #[error("Additional denom deposit detected")]
    AdditionalDenomDeposit {},


    #[error("Not possible to reach required (passing) weight")]
    UnreachableWeight {},

    #[error("No voters")]
    NoVoters {},

    #[error("More than 1 messages provided")]
    ExtraMessages {},

    #[error("Deposit refund already completed for the proposal")]
    RefundedAlready {},

    #[error("No Messages provided")]
    NoMessage {},

    #[error("Wrong Deposit Provided")]
    IncorrectDeposit {},

    #[error("No refund for failed proposal")]
    NonPassedProposalRefund {},

    #[error("No deposit record found for proposal")]
    NoDeposit{},

    #[error("Insufficient funds sent")]
    InsufficientFundsSend {},
    
    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Proposal is not open")]
    NotOpen {},

    #[error("Proposal voting period has expired")]
    Expired {},

    #[error("Proposal voting period has not expired , cannot execute the proposal message")]
    NotExpiredYet {},

    #[error("Proposal must expire before you can claim refund it")]
    NotExpired {},

    #[error("Governance token does not exist for app")]
    NoGovToken {},

    #[error("Gov Token not found in info.funds")]
    DenomNotFound {},

    #[error("Absolute Count Not Accepted")]
    AbsoluteCountNotAccepted {},

    #[error("Absolute percentage Not Accepted")]
    AbsolutePercentageNotAccepted {},

    #[error("Invalid threshold")]
    InvalidThreshold {},
    
    #[error("Required quorum threshold cannot be zero")]
    ZeroQuorumThreshold {},

    #[error("Not possible to reach required quorum threshold")]
    UnreachableQuorumThreshold {},
    
    #[error("Wrong expiration option")]
    WrongExpiration {},

    #[error("Already voted on this proposal")]
    AlreadyVoted {},

    #[error("Proposal must have passed and not yet been executed")]
    WrongExecuteStatus {},

    #[error("Cannot initiate the refund")]
    WrongRefundStatus {},

    #[error("IncorrectDenomDeposit")]
    IncorrectDenomDeposit {},


    #[error("Proposal must be open or pending to deposit")]
    CannotDeposit {},


    #[error("Total Gov Token Supply is 0")]
    ZeroSupply {},

    #[error("Proposal Msg Error ( {err:?}")]
    ProposalError{err:String},

    #[error("Incorrect App ID provided in msg")]
    DifferentAppID {},

    #[error("Proposal is slashed")]
    SlashedProposal {},

    #[error("Proposal is not vetoed")]
    ProposalNotVetoed {},

    #[error("Proposal is not rejected")]
    NotRejected {},

    #[error("Proposal is already slashed")]
    AlreadySlashed {},
}
