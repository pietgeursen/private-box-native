extern crate private_box;
use private_box::{SecretKey};

mod napi_sys;
use napi_sys::*;
use std::{ptr, fmt, error};
use std::os::raw::c_void;

use private_box::{decrypt as decrypt_rs, init as init_rs};

#[no_mangle]
pub extern "C" fn init(){
    init_rs();
}

fn try_decrypt(env: napi_env, info: napi_callback_info) -> Result<napi_value, DecryptError> {

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
        .ok_or(DecryptError::SecretKeyError)?;

    decrypt_rs(cypher_slice, &key)
        .or(Err(DecryptError::NotARecipient))
        .and_then(|result|{
            create_buffer(env, &result)
        })
        .or_else(|_|{
            get_undefined_value(env)
        })
}

#[no_mangle]
pub extern "C" fn decrypt(env: napi_env, info: napi_callback_info) -> napi_value {
    match try_decrypt(env, info) {
        Ok(result) => result,
        Err(_) => get_undefined_value(env).unwrap() 
    }
}

#[no_mangle]
pub extern "C" fn decrypt_async() -> isize{
    unimplemented!()
}

fn get_undefined_value(env: napi_env)-> Result<napi_value, DecryptError>{
    let mut undefined_value: napi_value = ptr::null_mut();
    let status: napi_status;
    unsafe {
        status = napi_get_undefined(env, &mut undefined_value);
    }

    match status {
        napi_status_napi_ok => Ok(undefined_value),
        _ => Err(DecryptError::NapiError)
    }
}

fn get_arg(env: napi_env, info: napi_callback_info, arg_index: usize) -> Result<napi_value, DecryptError> {
    let status: napi_status;
    let mut num_args = arg_index + 1;
    let mut args: Vec<napi_value> = Vec::with_capacity(num_args);

    unsafe {
        status = napi_get_cb_info(env, info, &mut num_args, args.as_mut_ptr(), ptr::null_mut(), ptr::null_mut());
        args.set_len(num_args);
    }

    match status {
        napi_status_napi_ok => Ok(args[arg_index].clone()),
        _ => Err(DecryptError::ArgumentError)
    }
}

fn get_buffer_info(env: napi_env, buffer: napi_value) -> Result<(*const u8, usize), DecryptError>{
    let status: napi_status;
    let mut buff_size = 0; 
    let mut p_buff: * mut c_void = ptr::null_mut(); 

    unsafe {
        status = napi_get_buffer_info(env, buffer, &mut p_buff, &mut buff_size);
    }

    match status {
        napi_status_napi_ok => Ok((p_buff as *const u8, buff_size)),
        _ => Err(DecryptError::ArgumentToBufferError)
    }
}

fn create_buffer(env: napi_env, slice: &[u8] ) -> Result<napi_value, DecryptError>{
    let status: napi_status;
    let mut _p_buff: * mut c_void = ptr::null_mut(); 
    let mut buffer: napi_value = ptr::null_mut();

    unsafe {
        status = napi_create_buffer_copy(env, slice.len(), slice.as_ptr() as * const c_void, &mut _p_buff, &mut buffer);
    }

    match status {
        napi_status_napi_ok => Ok(buffer),
        _ => Err(DecryptError::CreateBufferError)
    }
}

#[derive(Debug)]
enum DecryptError {
    ArgumentError,
    ArgumentToBufferError,
    CreateBufferError,
    SecretKeyError,
    NotARecipient,
    NapiError
}

impl fmt::Display for DecryptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecryptError::ArgumentError => write!(f,"Error getting argument"),
            DecryptError::ArgumentToBufferError => write!(f,"Error converting to buffer"),
            DecryptError::CreateBufferError => write!(f,"Error creating buffer"),
            DecryptError::SecretKeyError => write!(f,"Error converting slice to Secret key, was the key length right?"),
            DecryptError::NotARecipient => write!(f,"Couldn't find a message for us"),
            DecryptError::NapiError => write!(f,"Error calling n-api internally."),
        }
    }
}

impl error::Error for DecryptError {
    fn description(&self) -> &str {
        match *self {
            DecryptError::ArgumentError => "Error getting argument",
            DecryptError::ArgumentToBufferError => "Error converting to buffer",
            DecryptError::CreateBufferError => "Error creating buffer",
            DecryptError::SecretKeyError => "Error converting slice to Secret key, was the key length right?",
            DecryptError::NotARecipient => "Couldn't find a message for us",
            DecryptError::NapiError => "Error calling n-api internally.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

