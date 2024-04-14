mod rpc;
pub mod utils;

use prettytable::format::TableFormat;
use prettytable::Table;
use starknet::core::types::FieldElement;

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
    /// Build the table using the given table.
    fn build_table(&self, table: &mut Table);

    /// Convert the type to a prettytable::Table
    fn tablify(&self) -> Table {
        let mut table = Table::new();
        table.set_format(Self::format());
        self.build_table(&mut table);
        table
    }

    /// Get the default table format
    fn format() -> TableFormat {
        use prettytable::format::consts::FORMAT_BOX_CHARS;
        *FORMAT_BOX_CHARS
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

impl<T: Pretty> Pretty for Vec<T> {
    fn prettify(&self) -> String {
        self.iter()
            .map(|i| i.prettify())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
