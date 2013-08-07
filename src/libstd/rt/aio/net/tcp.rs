// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use option::{Option, Some, None};
use result::{Result, Ok, Err};
use kinds::Send;
use cell::Cell;
use std::unstable::future;
use std::unstable::future::{Future, Promise, pair};
use rt::io::net::ip::SocketAddr;
use rt::aio::{ReaderAsync};
//use rt::aio::{ReaderAsync, WriterAsync, ListenerAsync};
use rt::io::{io_error, IoError};
//use rt::io::{io_error, read_error, EndOfFile};
use rt::rtio::{IoFactory, IoFactoryObject};
use rt::rtaio::{RtaioTcpStreamAsync, RtaioTcpListenerAsync};
//use rt::rtaio::{RtaioTcpListenerAsync, RtaioTcpListenerAsyncObject,
//                          RtaioTcpStreamAsync, RtaioTcpStreamAsyncObject};
use rt::local::Local;

use io::println;

pub struct TcpStreamAsync(~RtaioTcpStreamAsync);

// boilerplate reduction
fn async_init_common<TRaw:Send, TOut:Send>(
    io_cb: ~fn(*mut IoFactoryObject, Promise<Result<TRaw, IoError>>),
    resolve_cb: ~fn(TRaw) -> TOut)
        -> Future<Option<TOut>> {
    let (ret_future, ret_promise) = pair();
    unsafe {
        let io = Local::unsafe_borrow::<IoFactoryObject>();
        io_cb(io, ret_promise);
    }
    let f_cell = Cell::new(ret_future);
    future::from_fn(|| {
      let ret_future = f_cell.take();
      println(fmt!("BEFORE RESOLVING ret_future: %?", ret_future));
      let r = ret_future.unwrap();
      println(fmt!("AFTER RESOLVING ret_future: %?", r));
      match r {
          Ok(s) => Some(resolve_cb(s)),
          Err(e) => {
              println(fmt!("failed to async connect: %?", e));
              rtdebug!("failed to async connect: %?", e);
              io_error::cond.raise(e);
              println("AFTER COND RAISE");
              None
          }
      }
    })
}

impl TcpStreamAsync {
    fn new(s: ~RtaioTcpStreamAsync) -> TcpStreamAsync {
        TcpStreamAsync(s)
    }
    pub fn connect(addr: SocketAddr) -> Future<Option<TcpStreamAsync>> {
        async_init_common(|io, promise| unsafe {
            let p_cell = Cell::new(promise);
            (*io).tcp_connect_async(addr,
                |connect_result| {
                    let promise = p_cell.take();
                    promise.fulfill(connect_result);
                }
            );
        }, |raw| {
            TcpStreamAsync::new(raw)
        })
        /*
        let (ret_future, ret_promise) = pair();
        unsafe {
            let io = Local::unsafe_borrow::<IoFactoryObject>();
            (*io).tcp_connect_async(addr,
                |connect_result| {
                    let ret_promise = p_cell.take();
                    ret_promise.fulfill(connect_result);
                }
            );
        }
        let f_cell = Cell::new(ret_future);
        future::from_fn(|| {
          let ret_future = f_cell.take();
          let r = ret_future.unwrap();
          match r {
              Ok(s) => Some(TcpStreamAsync::new(s)),
              Err(e) => {
                  rtdebug!("failed to async connect: %?", e);
                  io_error::cond.raise(e);
                  None
              }
          }
        })
        */
    }
}

impl ReaderAsync for TcpStreamAsync {
    pub fn read_async(&mut self, bytes_to_read: uint) -> Future<Option<~[u8]>> {
        bytes_to_read + 1;
        future::from_value(None)
    }
    pub fn eof_async(&mut self) -> Future<bool> {
        future::from_value(false)
    }
}

pub struct TcpListenerAsync(~RtaioTcpListenerAsync);

impl TcpListenerAsync {
    fn new(s: ~RtaioTcpListenerAsync) -> TcpListenerAsync {
        TcpListenerAsync(s)
    }

    /*
    pub fn bind(addr: SocketAddr) -> Future<Option<TcpListenerAsync>> {
        async_init_common(|io, promise| unsafe {
            let p_cell = Cell::new(promise);
            (*io).tcp_bind_async(addr,
                |bind_result| {
                    let promise = p_cell.take();
                    promise.fulfill(bind_result);
                }
            );
        }, |raw| {
            TcpListenerAsync::new(raw)
        })
    }
    */
}

//#[cfg(test)]
mod test{
    use super::*;
    use option::{Some, None};
    //#[cfg(test)]
    use rt::test::*;
    use rt::io::*;
    use io::println;
    use rt::io::net::ip::{SocketAddr, Ipv4Addr};
    //#[test]
    //#[ignore]
    fn smoke_test_async_ip4() {
        do run_in_newsched_task {
            let addr = next_test_ip4();
            connect_async_ip4(addr);
        }
    }
    fn connect_async_ip4(addr: SocketAddr) {
        let mut called = false;
        do io_error::cond.trap(|e| {
            println(fmt!("BLEH: %?", e));
            called = true;
        }).inside {
            let stream_f = TcpStreamAsync::connect(addr);
            let stream = stream_f.unwrap();
            match stream {
                Some(_) => {},
                None => fail!("async connect is none")
            }
        }
        //assert!(called == false);
    }
    #[test]
    fn aio_net_tcp_connect_async_error() {
        do run_in_newsched_task {
            connect_error_async_impl();
        }           
    }
    fn connect_error_async_impl() {
        let mut called = false;
        do io_error::cond.trap(|e| {
            assert!(e.kind == ConnectionRefused || e.kind == PermissionDenied);
            called = true;
        }).inside {
            let addr =
                SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 1 };
            //let stream_f = TcpStreamAsync::connect(addr);
            let stream = TcpStreamAsync::connect(addr);
            println(fmt!("STREAM FUTURE FROM CONNECT: %?",stream));
            let stream = stream.unwrap();
            println(fmt!("STREAM OPTION, POST UNWRAP: %?",stream));
            //assert!(stream.is_none());
        }
        println(fmt!("CALLED? %?", called));
    }
}