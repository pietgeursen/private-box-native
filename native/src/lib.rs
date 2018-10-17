extern crate private_box;
use private_box::{SecretKey, SECRETKEYBYTES};

mod napi_sys;
use napi_sys::*;

use private_box::{decrypt as decrypt_rs, init as init_rs};

#[no_mangle]
pub extern "C" fn init(){
    init_rs();
}

#[no_mangle]
pub extern "C" fn decrypt() -> isize{
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn decrypt_async(p_cypher_text: &u8, cypher_text_len: usize, p_key: &u8, p_result_buf: &mut u8, result_buf_len: &mut usize) -> isize{
    unimplemented!()
}
