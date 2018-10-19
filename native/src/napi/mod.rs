use napi_sys::*;
use errors::*;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;
use std::alloc::{Layout, alloc, dealloc};

pub fn throw_error(env: napi_env, err: ErrorKind) -> Result<()> {
    let status: napi_status;
    let msg = CString::new(err.description()).unwrap();
    unsafe {
        status = napi_throw_error(env, ptr::null(), msg.as_ptr() as *const i8);
    }

    match status {
        napi_status_napi_ok => Ok(()),
        _ => Err(ErrorKind::NapiError.into()),
    }
}

pub fn get_undefined_value(env: napi_env) -> Result<napi_value> {
    let mut undefined_value: napi_value = ptr::null_mut();
    let status: napi_status;
    unsafe {
        status = napi_get_undefined(env, &mut undefined_value);
    }

    match status {
        napi_status_napi_ok => Ok(undefined_value),
        _ => Err(ErrorKind::NapiError.into()),
    }
}

pub fn get_arg(env: napi_env, info: napi_callback_info, arg_index: usize) -> Result<napi_value> {
    let status: napi_status;
    let mut num_args = arg_index + 1;
    let mut args: Vec<napi_value> = Vec::with_capacity(num_args);

    unsafe {
        status = napi_get_cb_info(
            env,
            info,
            &mut num_args,
            args.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        args.set_len(num_args);
    }

    match status {
        napi_status_napi_ok => Ok(args[arg_index].clone()),
        _ => Err(ErrorKind::ArgumentError.into()),
    }
}

pub fn check_is_buffer(env: napi_env, value: napi_value) -> Result<bool> {
    let status: napi_status;
    let mut result = false;
    unsafe { status = napi_is_buffer(env, value, &mut result) }
    match status {
        napi_status_napi_ok => Ok(result),
        _ => Err(ErrorKind::NapiError.into()),
    }
}

pub fn get_buffer_info(env: napi_env, buffer: napi_value) -> Result<(*const u8, usize)> {
    let status: napi_status;
    let mut buff_size = 0;
    let mut p_buff: *mut c_void = ptr::null_mut();

    let is_buffer = check_is_buffer(env, buffer)?;
    if !is_buffer {
        bail!(ErrorKind::ArgumentTypeError)
    }

    unsafe {
        status = napi_get_buffer_info(env, buffer, &mut p_buff, &mut buff_size);
    }

    match status {
        napi_status_napi_ok => Ok((p_buff as *const u8, buff_size)),
        _ => Err(ErrorKind::ArgumentToBufferError.into()),
    }
}

pub fn create_buffer(env: napi_env, slice: &[u8]) -> Result<napi_value> {
    let status: napi_status;
    let mut _p_buff: *mut c_void = ptr::null_mut();
    let mut buffer: napi_value = ptr::null_mut();

    unsafe {
        status = napi_create_buffer_copy(
            env,
            slice.len(),
            slice.as_ptr() as *const c_void,
            &mut _p_buff,
            &mut buffer,
        );
    }

    match status {
        napi_status_napi_ok => Ok(buffer),
        _ => Err(ErrorKind::CreateBufferError.into()),
    }
}

