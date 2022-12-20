//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use wasm_bindgen_test::*;
use chrono_format;
wasm_bindgen_test_configure!();

#[wasm_bindgen_test]
fn pass() {
    let valid_formats = [
        "%FT%T%.3fZ",
        "%a %d %b %T %z %Y",
        "%+",
        "%G-%m-%dT%T",
        "%g-%m-%d",
        "%V: %A %B %G",
        "%FT%T%#z"
    ];
    let invalid_formats = [
        "%Q %m",
    ];
    for format in valid_formats {
        assert_eq!(chrono_format::is_valid_strftime_format(format), true);
    }
    for format in invalid_formats {
        assert_eq!(chrono_format::is_valid_strftime_format(format), false);
    }
}
