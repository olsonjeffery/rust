// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*! uvaio.rs
*/

use result::*;
use std::unstable::future;
use std::unstable::future::{Future};
use ops::Drop;
use cell::Cell;
use rt::uv::{TcpWatcher};
use rt::uv::uvio::{Tcp};
use uvio_socket_name = rt::uv::uvio::socket_name;
//use rt::rtio::{IoFactory, IoFactoryObject};
use rt::io::net::ip::{SocketAddr};
use rt::io::{IoError};
use rt::local::Local;
use rt::sched::Scheduler;
//use rt::rtaio::{RtaioSocketAsync};
use rt::tube::{Tube};
use rt::rtaio::{RtaioTcpStreamAsync, RtaioSocketAsync, RtaioTcpListenerAsync};
//use rt::rtaio::{RtaioTcpStreamAsync, RtaioTcpStreamAsyncObject};
//use rt::io::net::ip::{SocketAddr, Ipv4, Ipv6};
//use rt::uv::net::{UvIpv4, UvIpv6};

//#[cfg(test)] use container::Container;
//#[cfg(test)] use uint;
//#[cfg(test)] use unstable::run_in_bare_thread;
//#[cfg(test)] use rt::test::{spawntask_immediately,
//                            next_test_ip4,
//                            run_in_newsched_task};

pub struct UvTcpStreamAsync(TcpWatcher);

impl Drop for UvTcpStreamAsync {
    fn drop(&self) {
        rtdebug!("closing tcp stream");
        let scheduler = Local::take::<Scheduler>();
        do scheduler.deschedule_running_task_and_then |_, task| {
            let task_cell = Cell::new(task);
            do self.as_stream().close {
                let scheduler = Local::take::<Scheduler>();
                scheduler.resume_blocked_task_immediately(task_cell.take());
            }
        }
    }
}

impl RtaioSocketAsync for UvTcpStreamAsync {
    fn socket_name_async(&mut self) -> Future<Result<SocketAddr, IoError>> {
        let watcher = **self;
        do future::spawn {
            uvio_socket_name(Tcp, watcher)
        }
    }
}

impl RtaioTcpStreamAsync for UvTcpStreamAsync {
}

pub struct UvTcpListenerAsync {
    watcher: TcpWatcher,
    listening: bool,
    incoming_streams: Tube<Result<~RtaioTcpStreamAsync, IoError>>
}

impl UvTcpListenerAsync {
    pub fn new(watcher: TcpWatcher) -> UvTcpListenerAsync {
        UvTcpListenerAsync {
            watcher: watcher,
            listening: false,
            incoming_streams: Tube::new()
        }
    }
    fn watcher(&self) -> TcpWatcher { self.watcher }
}

impl Drop for UvTcpListenerAsync {
    fn drop(&self) {
        let watcher = self.watcher();
        let scheduler = Local::take::<Scheduler>();
        do scheduler.deschedule_running_task_and_then |_, task| {
            let task_cell = Cell::new(task);
            do watcher.as_stream().close {
                let scheduler = Local::take::<Scheduler>();
                scheduler.resume_blocked_task_immediately(task_cell.take());
            }
        }
    }
}

impl RtaioSocketAsync for UvTcpListenerAsync {
    fn socket_name_async(&mut self) -> Future<Result<SocketAddr, IoError>> {
        let watcher = (*self).watcher;
        do future::spawn {
            uvio_socket_name(Tcp, watcher)
        }
    }
}

impl RtaioTcpListenerAsync for UvTcpListenerAsync {
}
