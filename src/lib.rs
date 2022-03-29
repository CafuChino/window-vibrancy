//! Make your windows vibrant.
//!
//! ## Platform-specific
//!
//! - **Linux**: Unsupported, Blur and any vibrancy effects are controlled by the compositor installed on the end-user system.
//!
//! # Example
//!
//! ```no_run
//! use window_vibrancy::{apply_vibrancy, apply_blur, NSVisualEffectMaterial};
//!
//! # let window: &dyn raw_window_handle::HasRawWindowHandle = unsafe { std::mem::zeroed() };
//! #[cfg(target_os = "macos")]
//! apply_vibrancy(&window, NSVisualEffectMaterial::AppearanceBased).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
//!
//! #[cfg(target_os = "windows")]
//! apply_blur(&window, Some((18, 18, 18, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");
//! ```

mod macos;
mod windows;

pub use macos::NSVisualEffectMaterial;

/// a tuple of RGBA colors. Each value has minimum of 0 and maximum of 255.
pub type Color = (u8, u8, u8, u8);

/// Applies blur effect to window. Works only on Windows 7, Windows 10 v1809 or newer and Windows 11.
///
/// ## Argumesnts:
///
/// - *`color`* is ignored on Windows 7 and has no effect.
///
/// ## Platform-specific
///
/// - **Linux / macOS**: Unsupported.
pub fn apply_blur(
  window: impl raw_window_handle::HasRawWindowHandle,
  #[allow(unused)] color: Option<Color>,
) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => {
      windows::apply_blur(handle.hwnd as _, color)
    }
    _ => Err(Error::UnsupportedPlatform(
      "\"apply_blur()\" is only supported on Windows.",
    )),
  }
}

/// Clears blur effect applied to window. Works only on Windows 7, Windows 10 v1809 or newer and Windows 11.
///
/// ## Platform-specific
///
/// - **Linux / macOS**: Unsupported.
pub fn clear_blur(window: impl raw_window_handle::HasRawWindowHandle) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => windows::clear_blur(handle.hwnd as _),
    _ => Err(Error::UnsupportedPlatform(
      "\"clear_blur()\" is only supported on Windows.",
    )),
  }
}

/// Applies Acrylic effect to you window. Works only on Windows 10 v1809 or newer and Windows 11
///
/// - *`color`* is ignored on Windows 11 build 22523 and newer and has no effect.
///
/// ## WARNING:
///
/// This method has poor performance on Windows 10 v1903+ and Windows 11 build 22000,
/// the window will lag when resizing or dragging.
/// It is an issue in the undocumented api used for this method
/// and microsoft needs to fix it (they probably won't).
///
/// ## Platform-specific
///
/// - **Linux / macOS**: Unsupported.
pub fn apply_acrylic(
  window: impl raw_window_handle::HasRawWindowHandle,
  #[allow(unused)] color: Option<Color>,
) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => {
      windows::apply_acrylic(handle.hwnd as _, color)
    }
    _ => Err(Error::UnsupportedPlatform(
      "\"apply_acrylic()\" is only supported on Windows.",
    )),
  }
}

/// Clears acrylic effect applied to window. Works only on Windows 10 v1809 or newer and Windows 11.
///
/// ## Platform-specific
///
/// - **Linux / macOS**: Unsupported.
pub fn clear_acrylic(window: impl raw_window_handle::HasRawWindowHandle) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => windows::clear_acrylic(handle.hwnd as _),
    _ => Err(Error::UnsupportedPlatform(
      "\"clear_acrylic()\" is only supported on Windows.",
    )),
  }
}

/// Applies mica effect to window. Works only on Windows 11.
///
/// ## Platform-specific
///
/// - **Linux / macOS**: Unsupported.
pub fn apply_mica(window: impl raw_window_handle::HasRawWindowHandle) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => windows::apply_mica(handle.hwnd as _),
    _ => Err(Error::UnsupportedPlatform(
      "\"apply_mica()\" is only supported on Windows.",
    )),
  }
}

/// Clears mica effect applied to window. Works only on Windows 11.
///
/// ## Platform-specific
///
/// - **Linux / macOS**: Unsupported.
pub fn clear_mica(window: impl raw_window_handle::HasRawWindowHandle) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => windows::clear_mica(handle.hwnd as _),
    _ => Err(Error::UnsupportedPlatform(
      "\"clear_mica()\" is only supported on Windows.",
    )),
  }
}

/// Applies macos vibrancy effect to window. Works only on macOS 10.10 or newer.
///
/// ## Platform-specific
///
/// - **Linux / Windows**: Unsupported.
pub fn apply_vibrancy(
  window: impl raw_window_handle::HasRawWindowHandle,
  #[allow(unused)] effect: NSVisualEffectMaterial,
) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "macos")]
    raw_window_handle::RawWindowHandle::AppKit(handle) => {
      macos::apply_vibrancy(handle.ns_window as _, effect)
    }
    _ => Err(Error::UnsupportedPlatform(
      "\"apply_vibrancy()\" is only supported on macOS.",
    )),
  }
}

#[derive(Debug)]
pub enum Error {
  UnsupportedPlatform(&'static str),
  UnsupportedPlatformVersion(&'static str),
  NotMainThread(&'static str),
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::UnsupportedPlatform(e)
      | Error::UnsupportedPlatformVersion(e)
      | Error::NotMainThread(e) => {
        write!(f, "{}", e)
      }
    }
  }
}
