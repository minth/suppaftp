#![crate_name = "suppaftp"]
#![crate_type = "lib"]

//! # suppaftp
//!
//! suppaftp is an FTP client library written in Rust with optional FTPS support.
//!
//! ### Usage
//!
//! Here is a basic usage example:
//!
//! ```rust
//! use suppaftp::FtpStream;
//! let mut ftp_stream = FtpStream::connect("127.0.0.1:10021").unwrap_or_else(|err|
//!     panic!("{}", err)
//! );
//! assert!(ftp_stream.login("test", "test").is_ok());
//!
//! // Disconnect from server
//! assert!(ftp_stream.quit().is_ok());
//! ```
//!
//! ### FTPS
//!
//! The client supports FTPS on demand. To enable it the client should be
//! compiled with feature `secure` enabled which requires
//! [rust-native-tls](https://github.com/sfackler/rust-native-tls).
//!
//! The client uses explicit mode for connecting FTPS what means you should
//! connect the server as usually and then switch to the secure mode (TLS is used).
//! For better security it's the good practice to switch to the secure mode
//! before authentication.
//!
//! ### FTPS Usage
//!
//! ```rust
//! use suppaftp::FtpStream;
//! use suppaftp::native_tls::{TlsConnector, TlsStream};
//!
//! let ftp_stream = FtpStream::connect("test.rebex.net:21").unwrap();
//! // Switch to the secure mode
//! let mut ftp_stream = ftp_stream.into_secure(TlsConnector::new().unwrap(), "test.rebex.net").unwrap();
//! ftp_stream.login("demo", "password").unwrap();
//! // Do other secret stuff
//! // Switch back to the insecure mode (if required)
//! let mut ftp_stream = ftp_stream.into_insecure().unwrap();
//! // Do all public stuff
//! assert!(ftp_stream.quit().is_ok());
//! ```
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/rust-suppaftp/main/assets/images/cargo/suppaftp-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/rust-suppaftp/main/assets/images/cargo/suppaftp-512.png"
)]

// -- common deps
extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate thiserror;
// -- secure deps
#[cfg(feature = "secure")]
pub extern crate native_tls;
// -- async deps
#[cfg(feature = "async-secure")]
pub extern crate async_native_tls;
#[cfg(any(feature = "async", feature = "async-secure"))]
extern crate async_std;
#[cfg(any(feature = "async", feature = "async-secure"))]
extern crate pin_project;

#[cfg(test)]
extern crate pretty_assertions;
#[cfg(test)]
extern crate rand;

// -- private
#[cfg(any(feature = "async", feature = "async-secure"))]
mod async_ftp;
#[cfg(any(test, not(any(feature = "async", feature = "async-secure"))))]
mod sync_ftp;

// -- public
pub mod list;
pub mod status;
pub mod types;

// -- export async
#[cfg(any(feature = "async", feature = "async-secure"))]
pub use async_ftp::FtpStream;
// -- export sync
#[cfg(not(any(feature = "async", feature = "async-secure")))]
pub use sync_ftp::FtpStream;
// -- export (common)
pub use types::FtpError;
