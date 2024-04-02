use std::vec;

use chrono::{Local, TimeZone};
use comfy_table::modifiers::UTF8_SOLID_INNER_BORDERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use starknet::core::types::{
    DataAvailabilityMode, DeclareTransaction, DeployAccountTransaction, FieldElement,
};
use starknet::core::types::{
    Event, InvokeTransaction, MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, MsgToL1,
    Transaction, TransactionReceipt,
};

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

pub trait Pretty {
    fn prettify(&self) -> String;
}

pretty_for_lower_hex!(FieldElement, u64);

impl<T> Pretty for Vec<T>
where
    T: Pretty,
{
    fn prettify(&self) -> String {
        self.iter()
            .map(|i| i.prettify())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Pretty for Event {
    fn prettify(&self) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_SOLID_INNER_BORDERS)
            .add_row(vec!["FROM", &self.from_address.prettify()])
            .add_row(vec!["KEYS", &self.keys.prettify()])
            .add_row(vec!["DATA", &self.data.prettify()]);

        format!("{table}")
    }
}

impl Pretty for MsgToL1 {
    fn prettify(&self) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_SOLID_INNER_BORDERS)
            .add_row(vec!["TO", &self.to_address.prettify()])
            .add_row(vec!["PAYLOAD", &self.payload.prettify()]);

        format!("{table}")
    }
}

impl Pretty for MaybePendingTransactionReceipt {
    fn prettify(&self) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_SOLID_INNER_BORDERS);

        match self {
            Self::Receipt(receipt) => match receipt {
                TransactionReceipt::Invoke(_) => {
                    table.add_row(vec!["TYPE", "INVOKE"]);
                }
                TransactionReceipt::Declare(_) => {
                    table.add_row(vec!["TYPE", "DECLARE"]);
                }
                TransactionReceipt::Deploy(_) => {
                    table.add_row(vec!["TYPE", "DEPLOY"]);
                }
                TransactionReceipt::DeployAccount(_) => {
                    table.add_row(vec!["TYPE", "DEPLOY ACCOUNT"]);
                }
                TransactionReceipt::L1Handler(_) => {
                    table.add_row(vec!["TYPE", "L1 HANDLER"]);
                }
            },

            Self::PendingReceipt(pending) => return serde_json::to_string_pretty(pending).unwrap(),
        }

        let mut value = serde_json::to_value(self).unwrap();

        table
            .add_row(vec![
                "TRANSACTION HASH",
                &serde_json::from_value::<FieldElement>(value["transaction_hash"].take())
                    .unwrap()
                    .prettify(),
            ])
            .add_row(vec![
                "BLOCK HASH",
                &serde_json::from_value::<FieldElement>(value["block_hash"].take())
                    .unwrap()
                    .prettify(),
            ])
            .add_row(vec![
                "BLOCK NUMBER",
                &serde_json::from_value::<u64>(value["block_number"].take())
                    .unwrap()
                    .prettify(),
            ])
            .add_row(vec![
                "ACTUAL FEE",
                &serde_json::from_value::<FieldElement>(value["actual_fee"].take())
                    .unwrap()
                    .prettify(),
            ]);

        if let Some(value) = value.get("contract_address") {
            table.add_row(vec![
                "CONTRACT\nADDRESS",
                &serde_json::from_value::<FieldElement>(value.to_owned())
                    .unwrap()
                    .prettify(),
            ]);
        }

        table
            .add_row(vec![
                "FINALITY_STATUS",
                value["finality_status"].take().as_str().unwrap(),
            ])
            .add_row(vec![
                "EXECUTION_STATUS",
                value["execution_status"].take().as_str().unwrap(),
            ])
            .add_row(vec![
                "EVENTS",
                &serde_json::from_value::<Vec<Event>>(value["events"].take())
                    .unwrap()
                    .prettify(),
            ])
            .add_row(vec![
                "MESSAGES SENT",
                &serde_json::from_value::<Vec<MsgToL1>>(value["messages_sent"].take())
                    .unwrap()
                    .prettify(),
            ]);

        format!("{table}")
    }
}

impl Pretty for Transaction {
    fn prettify(&self) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_SOLID_INNER_BORDERS);

        match self {
            Self::Invoke(invoke) => {
                table.add_row(vec!["TYPE", "INVOKE"]);

                match invoke {
                    InvokeTransaction::V3(tx) => {
                        table
                            .add_row(vec!["TRANSACTION HASH", &tx.transaction_hash.prettify()])
                            .add_row(vec!["SENDER ADDRESS", &tx.sender_address.prettify()])
                            .add_row(vec!["SIGNATURE", &tx.signature.prettify()])
                            .add_row(vec!["NONCE", &tx.nonce.prettify()])
                            .add_row(vec!["CALLDATA", &tx.calldata.prettify()])
                            .add_row(vec!["TIP", &tx.tip.prettify()])
                            .add_row(vec!["PAYMASTER DATA", &tx.paymaster_data.prettify()])
                            .add_row(vec![
                                "ACCOUNT DEPLOYMENT DATA",
                                &tx.account_deployment_data.prettify(),
                            ])
                            .add_row(vec![
                                "NONCE DA MODE",
                                &tx.nonce_data_availability_mode.prettify(),
                            ])
                            .add_row(vec![
                                "FEE DA MODE",
                                &tx.fee_data_availability_mode.prettify(),
                            ])
                            .add_row(vec!["VERSION", "3"]);
                    }
                    InvokeTransaction::V1(tx) => {
                        table
                            .add_row(vec!["TRANSACTION HASH", &tx.transaction_hash.prettify()])
                            .add_row(vec!["SENDER ADDRESS", &tx.sender_address.prettify()])
                            .add_row(vec!["SIGNATURE", &tx.signature.prettify()])
                            .add_row(vec!["NONCE", &tx.nonce.prettify()])
                            .add_row(vec!["MAX FEE", &tx.max_fee.prettify()])
                            .add_row(vec!["CALLDATA", &tx.calldata.prettify()])
                            .add_row(vec!["VERSION", "1"]);
                    }
                    InvokeTransaction::V0(tx) => {
                        table
                            .add_row(vec!["TRANSACTION HASH", &tx.transaction_hash.prettify()])
                            .add_row(vec!["CONTRACT ADDRESS", &tx.contract_address.prettify()])
                            .add_row(vec![
                                "ENTRY POINT\nSELECTOR",
                                &tx.entry_point_selector.prettify(),
                            ])
                            .add_row(vec!["SIGNATURE", &tx.signature.prettify()])
                            .add_row(vec!["MAX FEE", &tx.max_fee.prettify()])
                            .add_row(vec!["CALLDATA", &tx.calldata.prettify()])
                            .add_row(vec!["VERSION", "0"]);
                    }
                }
            }

            Self::Declare(tx) => {
                table.add_row(vec!["TYPE", "DECLARE"]);
                match tx {
                    DeclareTransaction::V0(tx) => {
                        table
                            .add_row(vec!["TRANSACTION HASH", &tx.transaction_hash.prettify()])
                            .add_row(vec!["SENDER ADDRESS", &tx.sender_address.prettify()])
                            .add_row(vec!["CLASS HASH", &tx.class_hash.prettify()])
                            .add_row(vec!["SIGNATURE", &tx.signature.prettify()])
                            .add_row(vec!["MAX FEE", &tx.max_fee.prettify()])
                            .add_row(vec!["VERSION", &FieldElement::ZERO.prettify()]);
                    }

                    DeclareTransaction::V1(tx) => {
                        table
                            .add_row(vec!["TRANSACTION HASH", &tx.transaction_hash.prettify()])
                            .add_row(vec!["SENDER ADDRESS", &tx.sender_address.prettify()])
                            .add_row(vec!["CLASS HASH", &tx.class_hash.prettify()])
                            .add_row(vec!["SIGNATURE", &tx.signature.prettify()])
                            .add_row(vec!["NONCE", &tx.nonce.prettify()])
                            .add_row(vec!["MAX FEE", &tx.max_fee.prettify()])
                            .add_row(vec!["VERSION", &FieldElement::ONE.prettify()]);
                    }

                    DeclareTransaction::V2(tx) => {
                        table
                            .add_row(vec!["TRANSACTION HASH", &tx.transaction_hash.prettify()])
                            .add_row(vec!["SENDER ADDRESS", &tx.sender_address.prettify()])
                            .add_row(vec!["CLASS HASH", &tx.class_hash.prettify()])
                            .add_row(vec!["SIGNATURE", &tx.signature.prettify()])
                            .add_row(vec!["NONCE", &tx.nonce.prettify()])
                            .add_row(vec!["MAX FEE", &tx.max_fee.prettify()])
                            .add_row(vec!["VERSION", &FieldElement::TWO.prettify()]);
                    }

                    DeclareTransaction::V3(tx) => {
                        table
                            .add_row(vec!["TRANSACTION HASH", &tx.transaction_hash.prettify()])
                            .add_row(vec!["SENDER ADDRESS", &tx.sender_address.prettify()])
                            .add_row(vec!["CLASS HASH", &tx.class_hash.prettify()])
                            .add_row(vec!["SIGNATURE", &tx.signature.prettify()])
                            .add_row(vec!["NONCE", &tx.nonce.prettify()])
                            .add_row(vec!["VERSION", &FieldElement::THREE.prettify()]);
                    }
                }
            }

            Self::Deploy(tx) => {
                table
                    .add_row(vec!["TYPE", "DEPLOY"])
                    .add_row(vec!["TRANSACTION HASH", &tx.transaction_hash.prettify()])
                    .add_row(vec!["CLASS HASH", &tx.class_hash.prettify()])
                    .add_row(vec![
                        "CONTRACT ADDRESS\nSALT",
                        &tx.contract_address_salt.prettify(),
                    ])
                    .add_row(vec![
                        "CONSTRUCTOR\nCALLDATA",
                        &tx.constructor_calldata.prettify(),
                    ])
                    .add_row(vec!["VERSION", &tx.version.prettify()]);
            }

            Self::L1Handler(tx) => {
                table
                    .add_row(vec!["TYPE", "L1_HANDLER"])
                    .add_row(vec!["TRANSACTION HASH", &tx.transaction_hash.prettify()])
                    .add_row(vec!["CONTRACT ADDRESS", &tx.contract_address.prettify()])
                    .add_row(vec![
                        "ENTRY POINT\nSELECTOR",
                        &tx.entry_point_selector.prettify(),
                    ])
                    .add_row(vec!["CALLDATA", &tx.calldata.prettify()])
                    .add_row(vec!["NONCE", &tx.nonce.prettify()])
                    .add_row(vec!["VERSION", &tx.version.prettify()]);
            }

            Self::DeployAccount(tx) => {
                table.add_row(vec!["TYPE", "DEPLOY_ACCOUNT"]);

                match tx {
                    DeployAccountTransaction::V1(tx) => {
                        table
                            .add_row(vec!["TYPE", "DEPLOY_ACCOUNT"])
                            .add_row(vec!["TRANSACTION HASH", &tx.transaction_hash.prettify()])
                            .add_row(vec!["CLASS HASH", &tx.class_hash.prettify()])
                            .add_row(vec![
                                "CONTRACT ADDRESS\nSALT",
                                &tx.contract_address_salt.prettify(),
                            ])
                            .add_row(vec![
                                "CONSTRUCTOR\nCALLDATA",
                                &tx.constructor_calldata.prettify(),
                            ])
                            .add_row(vec!["SIGNATURE", &tx.signature.prettify()])
                            .add_row(vec!["MAX FEE", &tx.max_fee.prettify()])
                            .add_row(vec!["NONCE", &tx.nonce.prettify()])
                            .add_row(vec!["VERSION", &FieldElement::ONE.prettify()]);
                    }

                    DeployAccountTransaction::V3(tx) => {
                        table
                            .add_row(vec!["TYPE", "DEPLOY_ACCOUNT"])
                            .add_row(vec!["TRANSACTION HASH", &tx.transaction_hash.prettify()])
                            .add_row(vec!["CLASS HASH", &tx.class_hash.prettify()])
                            .add_row(vec![
                                "CONTRACT ADDRESS\nSALT",
                                &tx.contract_address_salt.prettify(),
                            ])
                            .add_row(vec![
                                "CONSTRUCTOR\nCALLDATA",
                                &tx.constructor_calldata.prettify(),
                            ])
                            .add_row(vec!["SIGNATURE", &tx.signature.prettify()])
                            .add_row(vec!["NONCE", &tx.nonce.prettify()])
                            .add_row(vec!["VERSION", &FieldElement::THREE.prettify()]);
                    }
                }
            }
        }

        format!("{table}")
    }
}

impl Pretty for MaybePendingBlockWithTxs {
    fn prettify(&self) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_SOLID_INNER_BORDERS);

        match self {
            Self::Block(block) => {
                table
                    .add_row(vec!["BLOCK HASH", &block.block_hash.prettify()])
                    .add_row(vec!["PARENT HASH", &block.parent_hash.prettify()])
                    .add_row(vec!["BLOCK NUMBER", &block.block_number.prettify()])
                    .add_row(vec!["NEW ROOT", &block.new_root.prettify()])
                    .add_row(vec![
                        "TIMESTAMP",
                        &Local
                            .timestamp_opt(block.timestamp as i64, 0)
                            .unwrap()
                            .to_string(),
                    ])
                    .add_row(vec![
                        "SEQUENCER ADDRESS",
                        &block.sequencer_address.prettify(),
                    ])
                    .add_row(vec![
                        "STATUS",
                        serde_json::to_value(block.status)
                            .unwrap_or_default()
                            .as_str()
                            .unwrap_or_default(),
                    ])
                    .add_row(vec!["TRANSACTIONS", &block.transactions.prettify()]);
            }

            Self::PendingBlock(block) => {
                table
                    .add_row(vec!["PARENT HASH", &block.parent_hash.prettify()])
                    .add_row(vec![
                        "TIMESTAMP",
                        &Local
                            .timestamp_opt(block.timestamp as i64, 0)
                            .unwrap()
                            .to_string(),
                    ])
                    .add_row(vec![
                        "SEQUENCER ADDRESS",
                        &block.sequencer_address.prettify(),
                    ])
                    .add_row(vec!["TRANSACTIONS", &block.transactions.prettify()]);
            }
        }

        format!("{table}")
    }
}

pub fn pretty_block_without_txs(block: &MaybePendingBlockWithTxs) -> String {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS);

    match block {
        MaybePendingBlockWithTxs::Block(block) => {
            table
                .add_row(vec!["BLOCK HASH", &block.block_hash.prettify()])
                .add_row(vec!["PARENT HASH", &block.parent_hash.prettify()])
                .add_row(vec!["BLOCK NUMBER", &block.block_number.prettify()])
                .add_row(vec!["NEW ROOT", &block.new_root.prettify()])
                .add_row(vec![
                    "TIMESTAMP",
                    &Local
                        .timestamp_opt(block.timestamp as i64, 0)
                        .unwrap()
                        .to_string(),
                ])
                .add_row(vec![
                    "SEQUENCER ADDRESS",
                    &block.sequencer_address.prettify(),
                ])
                .add_row(vec![
                    "STATUS",
                    serde_json::to_value(block.status)
                        .unwrap_or_default()
                        .as_str()
                        .unwrap_or_default(),
                ]);
        }
        MaybePendingBlockWithTxs::PendingBlock(block) => {
            table
                .add_row(vec!["PARENT HASH", &block.parent_hash.prettify()])
                .add_row(vec![
                    "TIMESTAMP",
                    &Local
                        .timestamp_opt(block.timestamp as i64, 0)
                        .unwrap()
                        .to_string(),
                ])
                .add_row(vec![
                    "SEQUENCER ADDRESS",
                    &block.sequencer_address.prettify(),
                ]);
        }
    }

    format!("{table}")
}

impl Pretty for DataAvailabilityMode {
    fn prettify(&self) -> String {
        match self {
            DataAvailabilityMode::L1 => format!("L1"),
            DataAvailabilityMode::L2 => format!("L2"),
        }
    }
}
