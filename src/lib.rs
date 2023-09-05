#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use crate::linux::get_text as _get_text;
#[cfg(target_os = "macos")]
pub use crate::macos::get_text as _get_text;
#[cfg(target_os = "windows")]
pub use crate::windows::get_text as _get_text;

#[napi]
pub fn get_text() -> String {
	_get_text()
}
