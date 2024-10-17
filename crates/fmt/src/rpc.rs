use std::vec;

use alloy_primitives::U256;
use prettytable::format::Alignment;
use prettytable::{Cell, Row, Table};
use starknet::core::types::{
    Event, ExecutionResources, ExecutionResult, FeePayment, MaybePendingTransactionReceipt,
    MsgToL1, PendingTransactionReceipt, PriceUnit, TransactionFinalityStatus, TransactionReceipt,
};

use crate::{utils, Pretty, Tabular};

impl Tabular for TransactionReceipt {
    fn build_table(&self, table: &mut Table) {
        match self {
            TransactionReceipt::Invoke(receipt) => {
                table.add_row(Row::from([
                    "TX HASH".to_string(),
                    receipt.transaction_hash.prettify(),
                ]));
                table.add_row(Row::from(["BLOCK HASH".to_string(), receipt.block_hash.prettify()]));
                table.add_row(Row::from([
                    "BLOCK NUMBER".to_string(),
                    receipt.block_number.to_string(),
                ]));
                table.add_row(Row::from(["ACTUAL FEE".to_string(), receipt.actual_fee.prettify()]));
                table.add_row(Row::from([
                    "FINALITY STATUS".to_string(),
                    receipt.finality_status.prettify(),
                ]));
                table.add_row(Row::from([
                    "EXECUTION RESULT".to_string(),
                    receipt.execution_result.prettify(),
                ]));
                table.add_row(Row::from([
                    "EXECUTION RESOURCES".to_string(),
                    receipt.execution_resources.tablify().to_string(),
                ]));
                table.add_row(Row::from([
                    "EVENTS".to_string(),
                    receipt.events.iter().map(|e| e.tablify().to_string()).collect(),
                ]));
                table.add_row(Row::from([
                    "MESSAGES SENT".to_string(),
                    receipt.messages_sent.iter().map(|e| e.tablify().to_string()).collect(),
                ]));
            }

            _ => {
                todo!()
            }
        }
    }
}

impl Tabular for MaybePendingTransactionReceipt {
    fn build_table(&self, table: &mut Table) {
        let type_row = match self {
            Self::Receipt(receipt) => {
                receipt.build_table(table);
                match receipt {
                    TransactionReceipt::Invoke(_) => {
                        Row::new(vec![Cell::new("TYPE"), Cell::new("INVOKE")])
                    }
                    TransactionReceipt::Declare(_) => {
                        Row::new(vec![Cell::new("TYPE"), Cell::new("DECLARE")])
                    }
                    TransactionReceipt::Deploy(_) => {
                        Row::new(vec![Cell::new("TYPE"), Cell::new("DEPLOY")])
                    }
                    TransactionReceipt::DeployAccount(_) => {
                        Row::new(vec![Cell::new("TYPE"), Cell::new("DEPLOY ACCOUNT")])
                    }
                    TransactionReceipt::L1Handler(_) => {
                        Row::new(vec![Cell::new("TYPE"), Cell::new("L1 HANDLER")])
                    }
                }
            }

            Self::PendingReceipt(pending) => match pending {
                PendingTransactionReceipt::Invoke(_) => {
                    Row::new(vec![Cell::new("TYPE"), Cell::new("INVOKE")])
                }
                PendingTransactionReceipt::Declare(_) => {
                    Row::new(vec![Cell::new("TYPE"), Cell::new("DECLARE")])
                }
                PendingTransactionReceipt::DeployAccount(_) => {
                    Row::new(vec![Cell::new("TYPE"), Cell::new("DEPLOY ACCOUNT")])
                }
                PendingTransactionReceipt::L1Handler(_) => {
                    Row::new(vec![Cell::new("TYPE"), Cell::new("L1 HANDLER")])
                }
            },
        };

        table.insert_row(0, type_row);
    }
}

impl Tabular for Event {
    fn build_table(&self, table: &mut Table) {
        table.add_row(Row::from(["From".to_string(), self.from_address.prettify()]));
        table.add_row(Row::from(["Keys".to_string(), self.keys.prettify()]));
        table.add_row(Row::from(["Data".to_string(), self.data.prettify()]));
    }
}

impl Tabular for MsgToL1 {
    fn build_table(&self, table: &mut Table) {
        table.add_row(Row::from(["From".to_string(), self.from_address.prettify()]));
        table.add_row(Row::from(["To".to_string(), self.to_address.prettify()]));
        table.add_row(Row::from(["Payload".to_string(), self.payload.prettify()]));
    }
}

impl Pretty for FeePayment {
    fn prettify(&self) -> String {
        let amount = U256::from_be_bytes(self.amount.to_bytes_be());
        let unit = match self.unit {
            PriceUnit::Wei => "ETH",
            PriceUnit::Fri => "STRK",
        };
        utils::format_erc20_balance(amount, unit, 18)
    }
}

impl Pretty for TransactionFinalityStatus {
    fn prettify(&self) -> String {
        match self {
            Self::AcceptedOnL1 => "Accepted on L1".into(),
            Self::AcceptedOnL2 => "Accepted on L2".into(),
        }
    }
}

impl Pretty for ExecutionResult {
    fn prettify(&self) -> String {
        match self {
            ExecutionResult::Succeeded => "Succeeded".into(),
            ExecutionResult::Reverted { reason } => format!("Reverted ({reason})"),
        }
    }
}

impl Tabular for ExecutionResources {
    fn format() -> prettytable::format::TableFormat {
        use prettytable::format::consts::FORMAT_CLEAN;
        *FORMAT_CLEAN
    }

    fn build_table(&self, table: &mut Table) {
        // in case we want to change the value alignment
        macro_rules! add_row {
            ($name:literal, $val:expr) => {
                table.add_row(Row::new(vec![
                    Cell::new($name).with_hspan(5),
                    Cell::new_align($val, Alignment::LEFT),
                ]))
            };
        }

        add_row!("Steps", &self.steps.to_string());

        if let Some(val) = self.memory_holes {
            add_row!("Memory holes", &val.to_string());
        }

        if let Some(val) = self.range_check_builtin_applications {
            add_row!("Range check builtin", &val.to_string());
        }

        if let Some(val) = self.pedersen_builtin_applications {
            add_row!("Pedersen builtin", &val.to_string());
        }

        if let Some(val) = self.poseidon_builtin_applications {
            add_row!("Poseidon builtin", &val.to_string());
        }

        if let Some(val) = self.ec_op_builtin_applications {
            add_row!("Ec op builtin", &val.to_string());
        }

        if let Some(val) = self.ecdsa_builtin_applications {
            add_row!("ECDSA builtin", &val.to_string());
        }

        if let Some(val) = self.bitwise_builtin_applications {
            add_row!("Bitwise builtin", &val.to_string());
        }

        if let Some(val) = self.keccak_builtin_applications {
            add_row!("Keccak builtin", &val.to_string());
        }

        if let Some(val) = self.segment_arena_builtin {
            add_row!("Segment arena builtin", &val.to_string());
        }
    }
}
