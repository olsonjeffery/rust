// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::{c_uint, c_int, c_void};
use ptr::null;

#[cfg(unix)]
pub type Key = pthread_key_t;

#[cfg(unix)]
pub unsafe fn create(key: &mut Key) {
    unsafe { fail_unless!(0 == pthread_key_create(key, null())); }
}

#[cfg(unix)]
pub unsafe fn set(key: Key, value: *mut c_void) {
    unsafe { fail_unless!(0 == pthread_setspecific(key, value)); }
}

#[cfg(unix)]
pub unsafe fn get(key: Key) -> *mut c_void {
    unsafe { pthread_getspecific(key) }
}

#[cfg(unix)]
#[allow(non_camel_case_types)] // foreign type
type pthread_key_t = c_uint;

#[cfg(unix)]
extern {
    fn pthread_key_create(key: *mut pthread_key_t, dtor: *u8) -> c_int;
    fn pthread_setspecific(key: pthread_key_t, value: *mut c_void) -> c_int;
    fn pthread_getspecific(key: pthread_key_t) -> *mut c_void;
}

#[test]
fn tls_smoke_test() {
    use cast::transmute;
    unsafe {
        let mut key = 0;
        let value = ~20;
        create(&mut key);
        set(key, transmute(value));
        let value: ~int = transmute(get(key));
        fail_unless!(value == ~20);
        let value = ~30;
        set(key, transmute(value));
        let value: ~int = transmute(get(key));
        fail_unless!(value == ~30);
    }
}
