mod utils;

use wasm_bindgen::prelude::*;
// use chrono::prelude::*;
use chrono::{format::{Item, strftime::StrftimeItems}};

#[wasm_bindgen]
pub fn is_valid_strftime_format(format: &str) -> bool {
    utils::set_panic_hook();

    StrftimeItems::new(format).all(|item| !matches!(item, Item::Error))
} 
