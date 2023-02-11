use std::{
    collections::HashMap,
    ffi::{c_char, c_void, CStr, CString},
    ops::{Deref, DerefMut},
};

use dashmap::DashMap;
use uuid::Uuid;

use crate::{
    document::DocumentId,
    fake_stew::{Stew, StewRegisterFn, StewRequestFn},
};

struct MyPlugin {
    stew: *const Stew,
    stew_request: StewRequestFn,
    popups: DashMap<Uuid, String>,
}


#[repr(C)]
struct BufferGetLineArgs {
    document_id: u128,
    line: usize,
}

impl MyPlugin {
    fn open_popup(&self, content: String) -> Uuid {
        let uuid = Uuid::new_v4();
        self.popups.insert(uuid, content);
        uuid
    }
    fn get_line_from_document(&self, document_id: DocumentId, line: usize) -> String {
        let fn_path = CString::new("core::buffer::get_line").unwrap().into_raw();
        let args = Box::into_raw(Box::new(BufferGetLineArgs {
            document_id: document_id.0.as_u128(),
            line,
        }));

        // Safety: We hope that we read the documentation and function type definition correctly, and it hasn't changed since...
        unsafe {
            let result = (self.stew_request)(self.stew, fn_path, args as *const c_void);
            CStr::from_ptr(result as _).to_str().unwrap().to_string()
        }
    }
}

/// # Safety
/// even more lmao
unsafe extern "C" fn open_popup(args: *const c_void, userdata: *mut *mut c_void) -> *const c_void {
    let content = CStr::from_ptr(args as _).to_str().unwrap().to_string();
    let plugin = (*userdata.cast::<*mut MyPlugin>()).as_ref().unwrap();
    let popup_id = plugin.open_popup(content);
    Box::into_raw(Box::new(popup_id)) as *const c_void
}

/// # Safety
/// even more lmao
pub unsafe extern "C" fn init(stew: *const Stew, register: StewRegisterFn, request: StewRequestFn) {
    let plugin = MyPlugin {
        stew,
        stew_request: request,
        popups: Default::default(),
    };
    let plugin = Box::into_raw(Box::new(plugin));
    register(
        stew,
        CString::new("fake_plugin::open_popup").unwrap().into_raw(),
        open_popup,
        plugin as *mut c_void,
    )
}
