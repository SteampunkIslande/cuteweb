pub mod fields;
pub mod main_table;
pub mod queries;

pub use fields::fields_get;
pub use main_table::main_table_get;
use rocket::{Route, routes};

pub fn modules_routes() -> Vec<Route> {
    routes![main_table_get, fields_get]
}
