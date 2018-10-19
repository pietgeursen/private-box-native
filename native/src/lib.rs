#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

extern crate private_box;
use private_box::SecretKey;

mod napi_sys;
mod napi;
mod errors;

use napi_sys::*;
use napi::*;
use errors::*;

use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;
use std::alloc::{Layout, alloc, dealloc};

use private_box::{decrypt as decrypt_rs, init as init_rs};

use errors::*;

#[no_mangle]
pub extern "C" fn init() {
    init_rs();
}

#[no_mangle]
pub extern "C" fn decrypt(env: napi_env, info: napi_callback_info) -> napi_value {
    match try_decrypt(env, info) {
        Ok(result) => result,
        Err(Error(ErrorKind::NotARecipient, _)) => get_undefined_value(env).unwrap(),
        Err(Error(err, _)) => {
            throw_error(env, err).unwrap();
            get_undefined_value(env).unwrap()
        }
    }
}

fn try_decrypt(env: napi_env, info: napi_callback_info) -> Result<napi_value> {
    let cypher_value = get_arg(env, info, 0)?;
    let secret_value = get_arg(env, info, 1)?;

    let (p_cypher, cypher_len) = get_buffer_info(env, cypher_value)?;
    let (p_secret, secret_len) = get_buffer_info(env, secret_value)?;

    let cypher_slice;
    let secret_slice;

    unsafe {
        cypher_slice = std::slice::from_raw_parts(p_cypher, cypher_len);
        secret_slice = std::slice::from_raw_parts(p_secret, secret_len);
    }

    let key = SecretKey::from_slice(secret_slice).ok_or(ErrorKind::SecretKeyError)?;

    decrypt_rs(cypher_slice, &key)
        .or(Err(ErrorKind::NotARecipient.into()))
        .and_then(|result| create_buffer(env, &result))
        .or_else(|_| get_undefined_value(env))
}

struct DecryptContext {
    cypher_ref: napi_ref,
    secret_ref: napi_ref,
    result_ref: napi_ref,

    work: napi_async_work,

    p_cypher: *mut u8,
    cypher_len: usize,
    p_secret: *mut u8,
    secret_len: usize,
    p_result: *mut u8,
    result_len: usize,
}

impl Default for DecryptContext{
    fn default() -> DecryptContext {
        DecryptContext{
            cypher_ref: ptr::null_mut(),
            secret_ref: ptr::null_mut(),
            result_ref: ptr::null_mut(),
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

extern "C" fn decrypt_async_execute(env: napi_env, data: *mut c_void){
    let status: napi_status;
    let mut context = unsafe { &mut *(data as *mut DecryptContext) };
}

extern "C" fn decrypt_async_complete(env: napi_env, data: *mut c_void){
    let status: napi_status;
    let mut context = unsafe { &mut *(data as *mut DecryptContext) };


    let layout = Layout::for_value(&context);
    unsafe {
        status = napi_delete_async_work(env, context.work);
        dealloc(data as *mut u8, layout);
    }
}

//maybe there's a trait that let's you "new" an unmanaged thing?
fn alloc_decrypt_context() -> *mut DecryptContext{
    let mut context = DecryptContext::default();
    let layout = Layout::for_value(&context);

    unsafe {
        alloc(layout);
    }
    &mut context
}

#[no_mangle]
pub extern "C" fn decrypt_async(env: napi_env, info: napi_callback_info) -> Result<napi_value> {
    let cypher_value = get_arg(env, info, 0)?;
    let secret_value = get_arg(env, info, 1)?;

    let (p_cypher, cypher_len) = get_buffer_info(env, cypher_value)?;
    let (p_secret, secret_len) = get_buffer_info(env, secret_value)?;

    let cypher_slice;
    let secret_slice;

    unsafe {
        cypher_slice = std::slice::from_raw_parts(p_cypher, cypher_len);
        secret_slice = std::slice::from_raw_parts(p_secret, secret_len);
    }

    let key = SecretKey::from_slice(secret_slice).ok_or(ErrorKind::SecretKeyError)?;

    let mut context = alloc_decrypt_context();

    unimplemented!()
}


