#![allow(unused)]

use std::{
    collections::HashMap,
    ffi::{c_char, c_void, CStr},
    sync::Arc,
};

use dashmap::DashMap;
use parking_lot::RwLock;

unsafe fn convert_str(ptr: *const c_char) -> String {
    CStr::from_ptr(ptr).to_str().unwrap().to_string()
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, derive_more::Display)]
#[display(fmt = transparent)]
struct FnPath(String);

pub type StewCallback =
    unsafe extern "C" fn(args: *const c_void, userdata: *mut *mut c_void) -> *const c_void;

pub type StewPluginInitFn =
    extern "C" fn(stew: *const Stew, register: StewRegisterFn, request: StewRequestFn);

#[derive(Clone)]
pub struct Stew {
    functions: Arc<DashMap<FnPath, (StewCallback, *mut c_void)>>,
}

impl Stew {
    fn load_plugin(&self, plugin_init_fn: StewPluginInitFn) {
        // As we never free this again, this is a memory leak.
        // But: who cares, making this global would result in the same lifetime ¯\_(ツ)_/¯
        let stew_ref = Box::into_raw(Box::new(self.clone()));
        plugin_init_fn(stew_ref, stew_register, stew_request);
    }
}

pub type StewRegisterFn = unsafe extern "C" fn(
    stew: *const Stew,
    fn_path: *const c_char,
    cb: StewCallback,
    userdata: *mut c_void,
);

/// # Safety
/// lmao
#[no_mangle]
pub unsafe extern "C" fn stew_register(
    stew: *const Stew,
    fn_path: *const c_char,
    cb: StewCallback,
    userdata: *mut c_void,
) {
    let fn_path = FnPath(convert_str(fn_path));
    (*stew).functions.insert(fn_path, (cb, userdata));
}

pub type StewRequestFn = unsafe extern "C" fn(
    stew: *const Stew,
    fn_path: *const c_char,
    arg: *const c_void,
) -> *const c_void;

/// # Safety
/// lmao
#[no_mangle]
pub unsafe extern "C" fn stew_request(
    stew: *const Stew,
    fn_path: *const c_char,
    arg: *const c_void,
) -> *const c_void {
    let fn_path = &FnPath(convert_str(fn_path));
    let mut entry = (*stew)
        .functions
        .get_mut(fn_path)
        .unwrap_or_else(|| panic!("no function registered under {fn_path}"));
    let (function, userdata) = entry.value_mut();
    function(arg, &mut *userdata)
}
