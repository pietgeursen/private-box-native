
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

extern crate private_box;
use private_box::{SecretKey};

mod napi_sys;
use napi_sys::*;
use std::ptr;
use std::os::raw::c_void;
use std::ffi::CString;

use private_box::{decrypt as decrypt_rs, init as init_rs};

use errors::*;

#[no_mangle]
pub extern "C" fn init(){
    init_rs();
}

#[no_mangle]
pub extern "C" fn decrypt(env: napi_env, info: napi_callback_info) -> napi_value {
    match try_decrypt(env, info) {
        Ok(result) => result,
        Err(Error(ErrorKind::NotARecipient, _)) => {
            get_undefined_value(env).unwrap()
        },
        Err(Error(err, _)) => {
            throw_type_error(env, err).unwrap();
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

    let key = SecretKey::from_slice(secret_slice)
        .ok_or_else(|| ErrorKind::SecretKeyError)?;

    decrypt_rs(cypher_slice, &key)
        .or(Err(ErrorKind::NotARecipient.into()))
        .and_then(|result|{
            create_buffer(env, &result)
        })
        .or_else(|_|{
            get_undefined_value(env)
        })
}


#[no_mangle]
pub extern "C" fn decrypt_async() -> isize{
    unimplemented!()
}

fn throw_type_error(env: napi_env, err: ErrorKind) -> Result<()>{
    let status: napi_status;
    let msg = CString::new(err.description()).unwrap();
    unsafe {
        status = napi_throw_type_error(env, ptr::null(), msg.as_ptr() as * const i8);
    }

    match status {
        napi_status_napi_ok => Ok(()),
        _ => Err(ErrorKind::NapiError.into())
    }
}

fn get_undefined_value(env: napi_env)-> Result<napi_value>{
    let mut undefined_value: napi_value = ptr::null_mut();
    let status: napi_status;
    unsafe {
        status = napi_get_undefined(env, &mut undefined_value);
    }

    match status {
        napi_status_napi_ok => Ok(undefined_value),
        _ => Err(ErrorKind::NapiError.into())
    }
}

fn get_arg(env: napi_env, info: napi_callback_info, arg_index: usize) -> Result<napi_value> {
    let status: napi_status;
    let mut num_args = arg_index + 1;
    let mut args: Vec<napi_value> = Vec::with_capacity(num_args);

    unsafe {
        status = napi_get_cb_info(env, info, &mut num_args, args.as_mut_ptr(), ptr::null_mut(), ptr::null_mut());
        args.set_len(num_args);
    }

    match status {
        napi_status_napi_ok => Ok(args[arg_index].clone()),
        _ => Err(ErrorKind::ArgumentError.into())
    }
}

fn check_is_buffer(env: napi_env, value: napi_value) -> Result<bool> {
    let status: napi_status;
    let mut result = false;
    unsafe {
        status = napi_is_buffer(env, value, &mut result)
    }
    match status {
        napi_status_napi_ok => Ok(result),
        _ => Err(ErrorKind::NapiError.into())
    }
}


fn get_buffer_info(env: napi_env, buffer: napi_value) -> Result<(*const u8, usize)>{
    let status: napi_status;
    let mut buff_size = 0; 
    let mut p_buff: * mut c_void = ptr::null_mut(); 

    let is_buffer = check_is_buffer(env, buffer)?;
    if !is_buffer {
        bail!(ErrorKind::ArgumentTypeError)
    }

    unsafe {
        status = napi_get_buffer_info(env, buffer, &mut p_buff, &mut buff_size);
    }

    match status {
        napi_status_napi_ok => Ok((p_buff as *const u8, buff_size)),
        _ => Err(ErrorKind::ArgumentToBufferError.into())
    }
}

fn create_buffer(env: napi_env, slice: &[u8] ) -> Result<napi_value>{
    let status: napi_status;
    let mut _p_buff: * mut c_void = ptr::null_mut(); 
    let mut buffer: napi_value = ptr::null_mut();

    unsafe {
        status = napi_create_buffer_copy(env, slice.len(), slice.as_ptr() as * const c_void, &mut _p_buff, &mut buffer);
    }

    match status {
        napi_status_napi_ok => Ok(buffer),
        _ => Err(ErrorKind::CreateBufferError.into())
    }
}

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        errors {
            ArgumentError {}
            ArgumentTypeError{}
            ArgumentToBufferError{}
            CreateBufferError{}
            SecretKeyError{}
            NotARecipient{}
            NapiError{}
        }
    }
}
