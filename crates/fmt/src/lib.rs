use std::vec;

use chrono::{Local, TimeZone};
use prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE;
use prettytable::Table;
use starknet::core::types::{
    DataAvailabilityMode, DeclareTransaction, DeployAccountTransaction, FieldElement,
    MaybePendingBlockWithTxHashes, TransactionExecutionStatus, TransactionStatus,
};
use starknet::core::types::{
    Event, InvokeTransaction, MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, MsgToL1,
    Transaction, TransactionReceipt,
};

/// Display trait for pretty printing
pub trait Pretty {
    fn prettify(&self) -> String;
}

impl<T: Tabular> Pretty for T {
    fn prettify(&self) -> String {
        self.tablify().to_string()
    }
}

/// Display trait for types that can be tabulated
pub trait Tabular {
    /// Convert the type to a prettytable::Table
    fn tablify(&self) -> Table;

    /// Get the default table format
    fn with_default_table() -> Table {
        let mut table = Table::new();
        table.set_format(*FORMAT_NO_LINESEP_WITH_TITLE);
        table
    }
}

/// Macro for implementing the [Pretty] trait for types implement [LowerHex](std::fmt::LowerHex) trait.
macro_rules! pretty_for_lower_hex {
	($($name:ty),*) => {
		$(
			impl Pretty for $name {
	            fn prettify(&self) -> String {
	                format!("{self:#x}")
	            }
	        }
		)*
	};
}

pretty_for_lower_hex!(FieldElement, u64);
