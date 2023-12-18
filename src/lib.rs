#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use selection::get_text as _get_text;

#[napi]
pub fn get_text() -> String {
	_get_text()
}
