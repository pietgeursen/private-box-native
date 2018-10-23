#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

extern crate private_box;
use private_box::SecretKey;

mod errors;
mod napi;
mod napi_sys;

use errors::*;
use napi::*;
use napi_sys::*;

use std::alloc::{alloc, dealloc, Layout};
use std::os::raw::c_void;
use std::ptr;

use private_box::{decrypt as decrypt_rs, init as init_rs};

#[no_mangle]
pub extern "C" fn init() {
    init_rs();
}

#[no_mangle]
pub extern "C" fn decrypt(env: napi_env, info: napi_callback_info) -> napi_value {
    match try_decrypt(env, info) {
        Ok(result) => result,
        Err(Error(ErrorKind::NotARecipient, _)) => get_undefined_value(env),
        Err(Error(err, _)) => {
            throw_error(env, err);
            get_undefined_value(env)
        }
    }
}

fn try_decrypt(env: napi_env, info: napi_callback_info) -> Result<napi_value> {
    let cypher_value = get_arg(env, info, 0);
    let secret_value = get_arg(env, info, 1);

    if !check_is_buffer(env, cypher_value) || !check_is_buffer(env, secret_value) {
        bail!(ErrorKind::ArgumentTypeError)
    }

    let (p_cypher, cypher_len) = get_buffer_info(env, cypher_value);
    let (p_secret, secret_len) = get_buffer_info(env, secret_value);

    let cypher_slice;
    let secret_slice;

    unsafe {
        cypher_slice = std::slice::from_raw_parts(p_cypher, cypher_len);
        secret_slice = std::slice::from_raw_parts(p_secret, secret_len);
    }

    let key = SecretKey::from_slice(secret_slice).ok_or(ErrorKind::SecretKeyError)?;

    match decrypt_rs(cypher_slice, &key) {
        Ok(result) => Ok(create_buffer_copy(env, &result)),
        _ => Ok(get_undefined_value(env)),
    }
}

struct DecryptContext {
    cypher_ref: napi_ref,
    secret_ref: napi_ref,
    result_ref: napi_ref,
    cb_ref: napi_ref,

    work: napi_async_work,

    p_cypher: *mut u8,
    cypher_len: usize,
    p_secret: *mut u8,
    secret_len: usize,
    p_result: *mut u8,
    result_len: usize,
}

impl Default for DecryptContext {
    fn default() -> DecryptContext {
        DecryptContext {
            cypher_ref: ptr::null_mut(),
            secret_ref: ptr::null_mut(),
            result_ref: ptr::null_mut(),
            cb_ref: ptr::null_mut(),
            work: ptr::null_mut(),
            p_cypher: ptr::null_mut(),
            cypher_len: 0,
            p_secret: ptr::null_mut(),
            secret_len: 0,
            p_result: ptr::null_mut(),
            result_len: 0,
        }
    }
}

extern "C" fn decrypt_async_execute(_env: napi_env, data: *mut c_void) {
    let context = unsafe { &mut *(data as *mut DecryptContext) };

    let cypher_slice;
    let secret_slice;
    let result_slice;

    unsafe {
        cypher_slice = std::slice::from_raw_parts(context.p_cypher, context.cypher_len);
        secret_slice = std::slice::from_raw_parts(context.p_secret, context.secret_len);
        result_slice = std::slice::from_raw_parts_mut(context.p_result, context.result_len);
    }

    let result = SecretKey::from_slice(secret_slice)
        .ok_or(ErrorKind::SecretKeyError)
        .and_then(|key| decrypt_rs(cypher_slice, &key).or(Err(ErrorKind::NotARecipient.into())));

    match result {
        Ok(result) => {
            result_slice[..result.len()].copy_from_slice(&result);
            context.result_len = result.len();
        }
        _ => {
            context.result_len = 0;
        }
    }
}

extern "C" fn decrypt_async_complete(env: napi_env, _status: napi_status, data: *mut c_void) {
    let context = unsafe { &mut *(data as *mut DecryptContext) };

    delete_reference(env, context.cypher_ref);
    delete_reference(env, context.secret_ref);

    let result = match context.result_len {
        0 => get_undefined_value(env),
        len => {
            let unsliced_result = get_reference_value(env, context.result_ref);
            slice_buffer(env, unsliced_result, 0, len)
        }
    };

    let cb = get_reference_value(env, context.cb_ref);

    let undefined = get_undefined_value(env);
    let args = [undefined, result];
    let mut global: napi_value = ptr::null_mut();
    let mut return_value: napi_value = ptr::null_mut();

    unsafe {
        napi_get_global(env, &mut global);
        napi_call_function(
            env,
            global,
            cb,
            2,
            &args[0] as *const napi_value,
            &mut return_value,
        );
    }

    delete_reference(env, context.cb_ref);
    unsafe {
        napi_delete_async_work(env, context.work);
    }
}

//maybe there's a trait that let's you "new" an unmanaged thing?
fn alloc_decrypt_context() -> *mut DecryptContext {
    let context = DecryptContext::default();
    let p_context: *mut DecryptContext;
    let layout = Layout::for_value(&context);

    unsafe {
        p_context = alloc(layout) as *mut DecryptContext;
    }
    p_context
}

//maybe there's a trait that let's you "new" an unmanaged thing?
extern "C" fn cleanup_decrypt_context(arg: *mut c_void) {
    let context = unsafe { &mut *(arg as *mut DecryptContext) };
    let layout = Layout::for_value(&context);

    unsafe {
        dealloc(arg as *mut u8, layout);
    }
}

#[no_mangle]

pub extern "C" fn decrypt_async(env: napi_env, info: napi_callback_info) -> napi_value {
    let context = alloc_decrypt_context();
    let mut status;

    unsafe {
        status =
            napi_add_env_cleanup_hook(env, Some(cleanup_decrypt_context), context as *mut c_void);
    }
    debug_assert!(status == napi_status_napi_ok);

    let cypher_value = get_arg(env, info, 0);
    let secret_value = get_arg(env, info, 1);
    let cb_value = get_arg(env, info, 2);

    if !check_is_buffer(env, cypher_value) || !check_is_buffer(env, secret_value) {
        let err = create_error(env, ErrorKind::ArgumentTypeError);
        let mut global: napi_value = ptr::null_mut();
        let mut return_value: napi_value = ptr::null_mut();

        unsafe {
            napi_get_global(env, &mut global);
            napi_call_function(
                env,
                global,
                cb_value,
                1,
                &err as *const napi_value,
                &mut return_value,
            );
        }
        return get_undefined_value(env);
    }

    let (p_cypher, cypher_len) = get_buffer_info(env, cypher_value);
    let (p_secret, secret_len) = get_buffer_info(env, secret_value);

    let result_buffer = create_buffer(env, cypher_len);
    let (p_result, result_len) = get_buffer_info(env, result_buffer);

    let cypher_ref = create_reference(env, cypher_value);
    let secret_ref = create_reference(env, secret_value);
    let result_ref = create_reference(env, result_buffer);
    let cb_ref = create_reference(env, cb_value);

    let mut work: napi_async_work = ptr::null_mut();
    let work_name = create_string_utf8(env, "private_box_decrypt_async");

    unsafe {
        (*context).p_cypher = p_cypher;
        (*context).p_secret = p_secret;
        (*context).p_result = p_result;
        (*context).cypher_len = cypher_len;
        (*context).secret_len = secret_len;
        (*context).result_len = result_len;

        (*context).cypher_ref = cypher_ref;
        (*context).secret_ref = secret_ref;
        (*context).result_ref = result_ref;
        (*context).cb_ref = cb_ref;

        //TODO check status
        status = napi_create_async_work(
            env,
            ptr::null_mut(),
            work_name,
            Some(decrypt_async_execute),
            Some(decrypt_async_complete),
            context as *mut DecryptContext as *mut c_void,
            &mut work,
        );
    }
    debug_assert!(status == napi_status_napi_ok);

    unsafe {
        (*context).work = work;
        status = napi_queue_async_work(env, work);
    }

    debug_assert!(status == napi_status_napi_ok);

    get_undefined_value(env)
}
