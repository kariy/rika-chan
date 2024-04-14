use starknet::core::types::{
    ContractErrorData, NoTraceAvailableErrorData, StarknetError, TransactionExecutionErrorData,
};

#[derive(Debug, thiserror::Error)]
pub enum StarknetRpcError {
    #[error("Failed to write transaction")]
    FailedToReceiveTransaction,
    #[error("Contract not found")]
    ContractNotFound,
    #[error("Block not found")]
    BlockNotFound,
    #[error("Invalid transaction index in a block")]
    InvalidTransactionIndex,
    #[error("Class hash not found")]
    ClassHashNotFound,
    #[error("Transaction hash not found")]
    TransactionHashNotFound,
    #[error("Requested page size is too big")]
    PageSizeTooBig,
    #[error("There are no blocks")]
    NoBlocks,
    #[error("The supplied continuation token is invalid or unknown")]
    InvalidContinuationToken,
    #[error("Too many keys provided in a filter")]
    TooManyKeysInFilter,
    #[error("Contract error: {}", _0.revert_error)]
    ContractError(ContractErrorData),
    #[error("Transaction execution error for transaction at index {}: {}", _0.transaction_index, _0.execution_error)]
    TransactionExecutionError(TransactionExecutionErrorData),
    #[error("Class already declared")]
    ClassAlreadyDeclared,
    #[error("Invalid transaction nonce")]
    InvalidTransactionNonce,
    #[error("Max fee is smaller than the minimal transaction cost (validation plus fee transfer)")]
    InsufficientMaxFee,
    #[error("Account balance is smaller than the transaction's max_fee")]
    InsufficientAccountBalance,
    #[error("Account validation failed")]
    ValidationFailure(String),
    #[error("Compilation failed")]
    CompilationFailed,
    #[error("Contract class size it too large")]
    ContractClassSizeIsTooLarge,
    #[error("Sender address in not an account contract")]
    NonAccount,
    #[error("A transaction with the same hash already exists in the mempool")]
    DuplicateTx,
    #[error("The compiled class hash did not match the one supplied in the transaction")]
    CompiledClassHashMismatch,
    #[error("The transaction version is not supported")]
    UnsupportedTxVersion,
    #[error("The contract class version is not supported")]
    UnsupportedContractClassVersion,
    #[error("An unexpected error occurred")]
    UnexpectedError(String),
    #[error("No trace available for transaction")]
    NoTraceAvailable(NoTraceAvailableErrorData),
}

impl From<StarknetError> for StarknetRpcError {
    fn from(error: StarknetError) -> Self {
        match error {
            StarknetError::FailedToReceiveTransaction => {
                StarknetRpcError::FailedToReceiveTransaction
            }
            StarknetError::ContractNotFound => StarknetRpcError::ContractNotFound,
            StarknetError::BlockNotFound => StarknetRpcError::BlockNotFound,
            StarknetError::InvalidTransactionIndex => StarknetRpcError::InvalidTransactionIndex,
            StarknetError::ClassHashNotFound => StarknetRpcError::ClassHashNotFound,
            StarknetError::TransactionHashNotFound => StarknetRpcError::TransactionHashNotFound,
            StarknetError::PageSizeTooBig => StarknetRpcError::PageSizeTooBig,
            StarknetError::NoBlocks => StarknetRpcError::NoBlocks,
            StarknetError::InvalidContinuationToken => StarknetRpcError::InvalidContinuationToken,
            StarknetError::TooManyKeysInFilter => StarknetRpcError::TooManyKeysInFilter,
            StarknetError::ContractError(data) => StarknetRpcError::ContractError(data),
            StarknetError::TransactionExecutionError(data) => {
                StarknetRpcError::TransactionExecutionError(data)
            }
            StarknetError::ClassAlreadyDeclared => StarknetRpcError::ClassAlreadyDeclared,
            StarknetError::InvalidTransactionNonce => StarknetRpcError::InvalidTransactionNonce,
            StarknetError::InsufficientMaxFee => StarknetRpcError::InsufficientMaxFee,
            StarknetError::InsufficientAccountBalance => {
                StarknetRpcError::InsufficientAccountBalance
            }
            StarknetError::ValidationFailure(msg) => StarknetRpcError::ValidationFailure(msg),
            StarknetError::CompilationFailed => StarknetRpcError::CompilationFailed,
            StarknetError::ContractClassSizeIsTooLarge => {
                StarknetRpcError::ContractClassSizeIsTooLarge
            }
            StarknetError::NonAccount => StarknetRpcError::NonAccount,
            StarknetError::DuplicateTx => StarknetRpcError::DuplicateTx,
            StarknetError::CompiledClassHashMismatch => StarknetRpcError::CompiledClassHashMismatch,
            StarknetError::UnsupportedTxVersion => StarknetRpcError::UnsupportedTxVersion,
            StarknetError::UnsupportedContractClassVersion => {
                StarknetRpcError::UnsupportedContractClassVersion
            }
            StarknetError::UnexpectedError(msg) => StarknetRpcError::UnexpectedError(msg),
            StarknetError::NoTraceAvailable(data) => StarknetRpcError::NoTraceAvailable(data),
        }
    }
}
