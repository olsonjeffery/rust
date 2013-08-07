// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*! Asynchronous I/O
...
*/
use option::{Option};
use result::{Result};
//use option::{Option, Some, None};
//use result::{Result, Ok, Err};
use std::unstable::future::{Future};
use rt::io::{IoError};

// Reexports
pub use self::net::tcp::TcpStreamAsync;

/// Async, non-blocking network I/O
pub mod net {
    pub mod tcp;
}

pub trait ReaderAsync {
    /// Async read operation
    fn read_async(&mut self, bytes_to_read: uint) -> Future<Option<~[u8]>>;

    /// Async eof check
    fn eof_async(&mut self) -> Future<bool>;
}
pub trait WriterAsync {
    /// Write the given buffer asynchronously
    ///
    /// # Failure
    ///
    /// Raises the `io_error` condition on error
    fn write(&mut self, buf: &[u8]) -> Future<Result<(), IoError>>;

    /// Flush output
    fn flush(&mut self) -> Future<()>;
}
/// A listener is a value that listens for connections
pub trait ListenerAsync<S> {
    /// Wait for and accept an incoming connection
    ///
    /// Returns `None` on timeout.
    ///
    /// # Failure
    ///
    /// Raises `io_error` condition. If the condition is handled,
    /// then `accept` returns `None`.
    fn accept_async(&mut self) -> Future<Option<S>>;
}