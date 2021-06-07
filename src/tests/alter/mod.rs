mod alter_table;
mod create_table;
mod drop_table;

#[cfg(feature = "alter-table")]
pub use alter_table::{alter_table_add_drop, alter_table_rename};
pub use create_table::create_table;
pub use drop_table::drop_table;