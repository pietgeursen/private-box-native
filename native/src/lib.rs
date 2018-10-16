extern crate private_box;
use private_box::{SecretKey, SECRETKEYBYTES};

use private_box::{decrypt as decrypt_rs, init as init_rs};

#[no_mangle]
pub extern "C" fn init(){
    init_rs();
}

#[no_mangle]
pub extern "C" fn decrypt(p_cypher_text: &u8, cypher_text_len: usize, p_key: &u8, p_result_buf: &mut u8, result_buf_len: &mut usize) -> isize{
    let key_slice: &[u8];
    let cypher_text_slice: &[u8];
    let result_slice: &mut[u8];
    unsafe {
        key_slice = std::slice::from_raw_parts(p_key, SECRETKEYBYTES);
        cypher_text_slice = std::slice::from_raw_parts(p_cypher_text, cypher_text_len);
        result_slice = std::slice::from_raw_parts_mut(p_result_buf, cypher_text_len); //We intentionally set the result lenght to cypher text len. It will never be bigger than that.
    }

    SecretKey::from_slice(key_slice)
        .ok_or_else(||())
        .and_then(|key|{
            decrypt_rs(cypher_text_slice, &key)
        })
        .and_then(|result|{
            let result_length = result.len();
            result_slice[..result_length].copy_from_slice(&result); 
            *result_buf_len = result_length;

            Ok(0)
        })
        .unwrap_or(-1)
}
