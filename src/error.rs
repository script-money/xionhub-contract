use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("CreatorAlreadyHasHub")]
    CreatorAlreadyHasHub,

    #[error("HubNotFound")]
    HubNotFound,

    #[error("AlreadySubscribed")]
    AlreadySubscribed,

    #[error("InsufficientFunds")]
    InsufficientFunds,

    #[error("PostNotFound")]
    PostNotFound,

    #[error("PostAlreadyExists")]
    PostAlreadyExists { id: String },

    #[error("PostAlreadyLiked")]
    PostAlreadyLiked { id: String },
}
