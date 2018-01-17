#[macro_use]
extern crate neon;
extern crate neon_runtime;
extern crate private_box;
extern crate sodiumoxide;
extern crate cslice;

use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{PublicKey, PUBLICKEYBYTES, SecretKey, SECRETKEYBYTES};

use std::ops::Deref;
use neon::vm::{Call, JsResult};
use neon::js::{JsNumber, Object};
use neon::js::binary::JsBuffer;
use neon::mem::Managed;
use neon_runtime::buffer;
use cslice::{CMutSlice};

use private_box::{encrypt as encrypt_rs, decrypt as decrypt_rs};


fn encrypt(call: Call){
    let scope = call.scope;

}


fn decrypt(call: Call)-> JsResult<JsBuffer>{

    let cypher_text_checked = call
        .arguments
        .require(call.scope, 0)?
        .check::<JsBuffer>()?;

    let cypher_text_buffer = cypher_text_checked
        .deref();

    let cypher_text_length = cypher_text_buffer
        .get(call.scope, "length")?
        .check::<JsNumber>()?
        .value();

    let mut cypher_text_vec = Vec::<u8>::with_capacity(cypher_text_length as usize);

    let mut cypher_text_cslice : CMutSlice<u8>;

    unsafe {
        cypher_text_vec.set_len(cypher_text_length as usize);
        cypher_text_cslice = CMutSlice::new(
            cypher_text_vec.as_mut_ptr(),
            cypher_text_length as usize
        );
        buffer::data(& mut cypher_text_cslice, cypher_text_buffer.to_raw());
    }

    let mut key_buffer : [u8; SECRETKEYBYTES] = [0; SECRETKEYBYTES];
    
    let secret_key_checked = call
        .arguments
        .require(call.scope, 1)?
        .check::<JsBuffer>()?;

    let secret_key_buffer = secret_key_checked
        .deref();

    let mut secret_key_cslice : CMutSlice<u8>;

    unsafe {
        secret_key_cslice = CMutSlice::new(
            key_buffer.as_mut_ptr(),
            SECRETKEYBYTES
        );

        buffer::data(& mut secret_key_cslice, secret_key_buffer.to_raw());
    }

    println!("{:?}", key_buffer);
    let secret_key = SecretKey(key_buffer);

    println!("13");
    println!("len of ctxt {}",cypher_text_vec.len());
    let mut plain_text = decrypt_rs(&cypher_text_vec, &secret_key).unwrap();
    println!("14");
    let mut plain_text_cslice : CMutSlice<u8>;
    println!("15");

    let result_buffer = JsBuffer::new(call.scope, plain_text.len() as u32).unwrap();

    println!("16");
    unsafe {
        plain_text_cslice = CMutSlice::new(
            plain_text.as_mut_ptr(),
            plain_text.len()
        );

        buffer::data(& mut plain_text_cslice, result_buffer.to_raw());
    }

    println!("17");

    Ok(result_buffer)

}

register_module!(m, {
    m.export("decrypt", decrypt)
});
