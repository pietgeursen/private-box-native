extern crate private_box;
extern crate sodiumoxide;

use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{SecretKey, SECRETKEYBYTES};

use private_box::{decrypt as decrypt_rs, init as init_rs};

#[no_mangle]
pub extern "C" fn init(){
    init_rs();
}

#[no_mangle]
pub extern "C" fn decrypt(p_cypher_text: &u8, cypher_text_len: usize, p_key: &u8, p_result_buf: &mut u8, result_buf_len: usize) -> isize{
    let key_slice: &[u8];
    let cypher_text_slice: &[u8];
    let result_slice: &mut[u8];
    unsafe {
        key_slice = std::slice::from_raw_parts(p_key, SECRETKEYBYTES);
        cypher_text_slice = std::slice::from_raw_parts(p_cypher_text, cypher_text_len);
        result_slice = std::slice::from_raw_parts_mut(p_result_buf, result_buf_len);
    }

    SecretKey::from_slice(key_slice)
        .ok_or_else(||())
        .and_then(|key|{
            decrypt_rs(cypher_text_slice, &key)
        })
        .and_then(|result|{
            result_slice.copy_from_slice(&result); 

            Ok(0)
        })
        .unwrap_or(-1)
}
