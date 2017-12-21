#[cfg(target_os="linux")]

#[macro_use]
extern crate neon;

extern crate libc;

use libc::{
    c_void,
    c_int,
    c_char,
    prctl,
    syscall
};

use std::ptr;
use std::ffi::{CStr, CString};

#[allow(non_camel_case_types)]
type kafel_ctxt_t = *mut c_void;

#[allow(non_camel_case_types)]
#[repr(C)]
struct sock_filter {
	code: u16,
	jt: u8,
	jf: u8,
    k: u32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct sock_fprog {
    len: u16,
    filter: *mut sock_filter,
}

const PR_SET_NO_NEW_PRIVS: i32 = 38;
const SECCOMP_FILTER_FLAG_TSYNC: i64 = 1;
const SECCOMP_SET_MODE_FILTER: i64 = 1;
const SYSCALL_SECCOMP: i64 = 317;
const DEFAULT_POLICY: &str  = "POLICY a { ERRNO(1) { execve, clone, fork } } USE a DEFAULT ALLOW";

#[link(name="kafel")]
extern "C" {
    fn kafel_ctxt_create() -> kafel_ctxt_t;
    fn kafel_ctxt_destroy(ctxt: kafel_ctxt_t) -> c_void;
    fn kafel_set_input_string(ctxt: kafel_ctxt_t, source: *const c_char) -> c_int;
    fn kafel_compile(ctxt: kafel_ctxt_t, prog: *mut sock_fprog) -> c_int;
    fn kafel_error_msg(ctx: kafel_ctxt_t) -> *const c_char;
}

unsafe fn kafel_set_policy(source: &CString, sync_all_threads: bool) -> i32 {
    let prog = Box::new(sock_fprog{len: 0, filter: ptr::null_mut()});
    let prog_ptr = Box::into_raw(prog);
    let ctxt = kafel_ctxt_create();
    kafel_set_input_string(ctxt, source.as_ptr());
    if kafel_compile(ctxt, prog_ptr) != 0{
        let err = CStr::from_ptr(kafel_error_msg(ctxt));
        eprintln!("error: {:?}", err);
        kafel_ctxt_destroy(ctxt);
        return -1;
    }
    let mut rv = prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
    if rv == 0{
        let mut flags: i64 = 0;
        if sync_all_threads {
            flags = SECCOMP_FILTER_FLAG_TSYNC;
        }
        let res =  syscall(SYSCALL_SECCOMP, SECCOMP_SET_MODE_FILTER, flags, prog_ptr);
        rv = res as i32;
    }
    return rv;
}


use neon::vm::{Call, JsResult};
use neon::js::{JsNumber};

fn forkoff(call: Call) -> JsResult<JsNumber> {
    let scope = call.scope;

    let policy = CString::new(DEFAULT_POLICY).unwrap();
    let rv = unsafe {kafel_set_policy(&policy, true)};
    Ok(JsNumber::new(scope, rv as f64))
}

register_module!(m, {
    m.export("forkoff", forkoff)
});
