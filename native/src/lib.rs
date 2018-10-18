extern crate private_box;
use private_box::{SecretKey, SECRETKEYBYTES};

mod napi_sys;
use napi_sys::*;
use std::ptr;
use std::os::raw::c_void;

use private_box::{decrypt as decrypt_rs, init as init_rs};

#[no_mangle]
pub extern "C" fn init(){
    init_rs();
}

fn get_undefined_value(env: napi_env)-> Result<napi_value, napi_status>{
    let mut undefined_value: napi_value = ptr::null_mut();
    let status: napi_status;
    unsafe {
        status = napi_get_undefined(env, &mut undefined_value);
    }

    match status {
        napi_status_napi_ok => Ok(undefined_value),
        _ => Err(status)
    }
}

fn get_arg(env: napi_env, info: napi_callback_info, arg_index: usize) -> Result<napi_value, napi_status> {
    let status: napi_status;
    let mut args: Vec<napi_value> = Vec::with_capacity(arg_index);
    let mut argc = arg_index;

    unsafe {
        status = napi_get_cb_info(env, info, &mut argc, args.as_mut_ptr(), ptr::null_mut(), ptr::null_mut());
    }

    match status {
        napi_status_napi_ok => Ok(args[arg_index].clone()),
        _ => Err(status)
    }
}

fn get_buffer_info(env: napi_env, buffer: napi_value) -> Result<(*const u8, usize), napi_status>{
    let status: napi_status;
    let mut buff_size = 0; 
    let mut p_buff: * mut c_void = ptr::null_mut(); 

    unsafe {
        status = napi_get_buffer_info(env, buffer, &mut p_buff, &mut buff_size);
    }

    match status {
        napi_status_napi_ok => Ok((p_buff as *const u8, buff_size)),
        _ => Err(status)
    }
}

fn create_buffer(env: napi_env, slice: &[u8] ) -> Result<napi_value, napi_status>{
    let status: napi_status;
    let mut _p_buff: * mut c_void = ptr::null_mut(); 
    let mut buffer: napi_value = ptr::null_mut();

    unsafe {
        status = napi_create_buffer_copy(env, slice.len(), slice.as_ptr() as * const c_void, &mut _p_buff, &mut buffer);
    }

    match status {
        napi_status_napi_ok => Ok(buffer),
        _ => Err(status)
    }
}

#[no_mangle]
pub extern "C" fn decrypt(env: napi_env, info: napi_callback_info) -> napi_value {
    let status: napi_status;

    //TODO: convert all the unwraps to useful js errors
    let cypher_value = get_arg(env, info, 0).unwrap();
    let secret_value = get_arg(env, info, 1).unwrap();

    let (p_cypher, cypher_len) = get_buffer_info(env, cypher_value).unwrap();
    let (p_secret, secret_len) = get_buffer_info(env, secret_value).unwrap();

    let cypher_slice;
    let secret_slice;

    unsafe {
        cypher_slice = std::slice::from_raw_parts(p_cypher, cypher_len);
        secret_slice = std::slice::from_raw_parts(p_secret, secret_len);
    }

    let key = SecretKey::from_slice(secret_slice)
        .unwrap();

    match decrypt_rs(cypher_slice, &key) {
        Ok(result) => {
            create_buffer(env, &result).unwrap()
        }
        _ => get_undefined_value(env).unwrap()
    }
}

#[no_mangle]
pub extern "C" fn decrypt_async() -> isize{
    unimplemented!()
}
