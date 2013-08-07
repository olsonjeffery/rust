// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*! rtaio.rs
*/

use std::unstable::future::{Future};
use result::{Result};
use rt::io::{IoError};
use rt::io::net::ip::{SocketAddr};
//use rt::io::net::ip::{SocketAddr, Ipv4, Ipv6};
//use rt::uv::net::{UvIpv4, UvIpv6};

pub trait RtaioSocketAsync {
    fn socket_name_async(&mut self) -> Future<Result<SocketAddr, IoError>>;
}
pub trait RtaioTcpStreamAsync : RtaioSocketAsync {
    /*
    fn read_async(&mut self, buf: &mut [u8]) -> Future<Result<uint, IoError>>;
    fn write_async(&mut self, buf: &[u8]) -> Future<Result<(), IoError>>;
    fn peer_name_async(&mut self) -> Future<Result<SocketAddr, IoError>>;
    fn control_congestion_async(&mut self) -> Future<Result<(), IoError>>;
    fn nodelay_async(&mut self) -> Future<Result<(), IoError>>;
    fn keepalive_async(&mut self, delay_in_seconds: uint) ->
        Future<Result<(), IoError>>;
    fn letdie_async(&mut self) -> Future<Result<(), IoError>>;
    */
}
pub trait RtaioTcpListenerAsync : RtaioSocketAsync {
    /*
    fn accept_async(&mut self) -> Future<Result<~RtaioTcpStreamAsync, IoError>>;
    fn accept_simultaneously_async(&mut self) -> Future<Result<(), IoError>>;
    fn dont_accept_simultaneously_async(&mut self) -> Future<Result<(), IoError>>;
    */
}