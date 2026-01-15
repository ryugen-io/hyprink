use std::cell::RefCell;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::path::{Path, PathBuf};
use std::ptr;

use hi_core::config::Config;
use hi_core::db::Store;
use hi_core::{logger, packager, processor, template};

// --- Contexts ---

/// Opaque context pointer (wraps Config with error storage)
pub struct HyprinkContext {
    pub config: Config,
    pub last_error: RefCell<Option<String>>,
    pub app_name: RefCell<Option<String>>,
}

/// Opaque store wrapper
pub struct HyprinkStore {
    pub inner: RefCell<Store>,
    pub last_error: RefCell<Option<String>>,
}

macro_rules! impl_error_handling {
    ($struct_name:ident) => {
        impl $struct_name {
            fn set_error(&self, err: String) {
                *self.last_error.borrow_mut() = Some(err);
            }

            fn clear_error(&self) {
                *self.last_error.borrow_mut() = None;
            }
        }
    };
}

impl_error_handling!(HyprinkContext);
impl_error_handling!(HyprinkStore);

// --- HyprinkContext API ---

#[no_mangle]
pub extern "C" fn hyprink_context_new() -> *mut HyprinkContext {
    match Config::load() {
        Ok(cfg) => {
            let ctx = Box::new(HyprinkContext {
                config: cfg,
                last_error: RefCell::new(None),
                app_name: RefCell::new(None),
            });
            Box::into_raw(ctx)
        }
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_context_free(ctx: *mut HyprinkContext) {
    if !ctx.is_null() {
        unsafe {
            drop(Box::from_raw(ctx));
        }
    }
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_get_last_error(
    ctx: *mut HyprinkContext,
    buffer: *mut c_char,
    len: usize,
) -> c_int {
    if ctx.is_null() || buffer.is_null() {
        return -1;
    }

    let context = unsafe { &*ctx };
    let borrow = context.last_error.borrow();

    if let Some(msg) = &*borrow {
        let bytes = msg.as_bytes();
        if bytes.len() >= len {
            return -1;
        }

        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), buffer as *mut u8, bytes.len());
            *buffer.add(bytes.len()) = 0;
        }

        return bytes.len() as c_int;
    }

    0
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_context_set_app_name(
    ctx: *mut HyprinkContext,
    name: *const c_char,
) {
    if !ctx.is_null() && !name.is_null() {
        let context = &*ctx;
        let s = CStr::from_ptr(name).to_string_lossy();
        *context.app_name.borrow_mut() = Some(s.into_owned());
    }
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_log(
    ctx: *mut HyprinkContext,
    level: *const c_char,
    scope: *const c_char,
    msg: *const c_char,
) {
    if ctx.is_null() {
        return;
    }

    unsafe {
        let context = &*ctx;
        let level_str = CStr::from_ptr(level).to_string_lossy();
        let scope_str = CStr::from_ptr(scope).to_string_lossy();
        let msg_str = CStr::from_ptr(msg).to_string_lossy();
        let app = context.app_name.borrow();

        logger::log_to_terminal(&context.config, &level_str, &scope_str, &msg_str);

        if context.config.layout.logging.write_by_default {
            let _ = logger::log_to_file(
                &context.config,
                &level_str,
                &scope_str,
                &msg_str,
                app.as_deref(),
            );
        }
    }
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_log_preset(
    ctx: *mut HyprinkContext,
    preset_key: *const c_char,
    msg_override: *const c_char,
) -> c_int {
    if ctx.is_null() || preset_key.is_null() {
        return 1;
    }

    let context = unsafe { &*ctx };
    context.clear_error();

    let key = unsafe { CStr::from_ptr(preset_key).to_string_lossy() };

    let preset = match context.config.presets.get(key.as_ref()) {
        Some(p) => p,
        None => {
            context.set_error(format!("Preset '{}' not found", key));
            return 1;
        }
    };

    let level = &preset.level;
    let scope = preset.scope.as_deref().unwrap_or("");
    let msg_default = &preset.msg;

    let msg_final = if !msg_override.is_null() {
        unsafe { CStr::from_ptr(msg_override).to_string_lossy() }
    } else {
        std::borrow::Cow::Borrowed(msg_default.as_str())
    };

    let app = context.app_name.borrow();
    logger::log_to_terminal(&context.config, level, scope, &msg_final);

    if context.config.layout.logging.write_by_default {
        let _ = logger::log_to_file(&context.config, level, scope, &msg_final, app.as_deref());
    }

    0
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_pack(
    ctx: *mut HyprinkContext,
    src_dir: *const c_char,
    out_file: *const c_char,
) -> c_int {
    if ctx.is_null() {
        return 1;
    }
    let context = unsafe { &*ctx };
    context.clear_error();

    unsafe {
        let src = CStr::from_ptr(src_dir).to_string_lossy();
        let out = CStr::from_ptr(out_file).to_string_lossy();

        match packager::pack(Path::new(&*src), Path::new(&*out)) {
            Ok(_) => 0,
            Err(e) => {
                context.set_error(format!("{:#}", e));
                1
            }
        }
    }
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_unpack(
    ctx: *mut HyprinkContext,
    pkg_file: *const c_char,
    target_dir: *const c_char,
) -> c_int {
    if ctx.is_null() {
        return 1;
    }
    let context = unsafe { &*ctx };
    context.clear_error();

    unsafe {
        let pkg = CStr::from_ptr(pkg_file).to_string_lossy();
        let target = CStr::from_ptr(target_dir).to_string_lossy();

        match packager::unpack(Path::new(&*pkg), Path::new(&*target)) {
            Ok(_) => 0,
            Err(e) => {
                context.set_error(format!("{:#}", e));
                1
            }
        }
    }
}

#[no_mangle]
/// Applies a template file immediately to the current config context.
/// # Safety
pub unsafe extern "C" fn hyprink_apply_file(
    ctx: *mut HyprinkContext,
    path: *const c_char,
) -> c_int {
    if ctx.is_null() {
        return 1;
    }
    let context = unsafe { &*ctx };
    context.clear_error();

    let p = unsafe { CStr::from_ptr(path).to_string_lossy() };
    match std::fs::read_to_string(Path::new(&*p)) {
        Ok(content) => match toml::from_str::<template::Template>(&content) {
            Ok(tpl) => match processor::apply(&tpl, &context.config, false) {
                Ok(_) => 0,
                Err(e) => {
                    context.set_error(format!("Apply error: {:#}", e));
                    1
                }
            },
            Err(e) => {
                context.set_error(format!("Parse error: {:#}", e));
                1
            }
        },
        Err(e) => {
            context.set_error(format!("File read error: {:#}", e));
            1
        }
    }
}

// --- HyprinkStore API ---

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_store_load(path: *const c_char) -> *mut HyprinkStore {
    let p_str = if !path.is_null() {
        unsafe { CStr::from_ptr(path).to_string_lossy() }
    } else {
        return ptr::null_mut();
    };

    let p_path = PathBuf::from(p_str.as_ref());

    match Store::load(&p_path) {
        Ok(s) => {
            let ctx = Box::new(HyprinkStore {
                inner: RefCell::new(s),
                last_error: RefCell::new(None),
            });
            Box::into_raw(ctx)
        }
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_store_free(store: *mut HyprinkStore) {
    if !store.is_null() {
        unsafe {
            drop(Box::from_raw(store));
        }
    }
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_store_get_last_error(
    store: *mut HyprinkStore,
    buffer: *mut c_char,
    len: usize,
) -> c_int {
    if store.is_null() || buffer.is_null() {
        return -1;
    }

    let s = unsafe { &*store };
    let borrow = s.last_error.borrow();

    if let Some(msg) = &*borrow {
        let bytes = msg.as_bytes();
        if bytes.len() >= len {
            return -1;
        }

        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), buffer as *mut u8, bytes.len());
            *buffer.add(bytes.len()) = 0;
        }

        return bytes.len() as c_int;
    }

    0
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_store_save(store: *mut HyprinkStore) -> c_int {
    if store.is_null() {
        return 1;
    }
    let s = unsafe { &*store };
    s.clear_error();

    match s.inner.borrow().save() {
        Ok(_) => 0,
        Err(e) => {
            s.set_error(format!("{:#}", e));
            1
        }
    }
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_store_add_toml(
    store: *mut HyprinkStore,
    toml_content: *const c_char,
) -> c_int {
    if store.is_null() || toml_content.is_null() {
        return 1;
    }
    let s = unsafe { &*store };
    s.clear_error();

    let content = unsafe { CStr::from_ptr(toml_content).to_string_lossy() };
    match toml::from_str::<template::Template>(&content) {
        Ok(tpl) => match s.inner.borrow_mut().add(tpl) {
            Ok(_) => 0,
            Err(e) => {
                s.set_error(format!("Store error: {:#}", e));
                1
            }
        },
        Err(e) => {
            s.set_error(format!("Parse error: {:#}", e));
            1
        }
    }
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_store_remove(
    store: *mut HyprinkStore,
    name: *const c_char,
) -> c_int {
    if store.is_null() || name.is_null() {
        return 1;
    }
    let s = unsafe { &*store };
    s.clear_error();

    let n = unsafe { CStr::from_ptr(name).to_string_lossy() };
    s.inner.borrow_mut().remove(&n);
    0
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn hyprink_store_count(store: *mut HyprinkStore) -> c_int {
    if store.is_null() {
        return -1;
    }
    let s = unsafe { &*store };
    s.inner.borrow().list().len() as c_int
}
