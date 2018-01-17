#[macro_use]
extern crate neon;
extern crate neon_runtime;
extern crate private_box;
extern crate sodiumoxide;
extern crate cslice;

use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{PublicKey, PUBLICKEYBYTES, SecretKey, SECRETKEYBYTES};

use std::ops::Deref;
use neon::vm::{Lock, Call, JsResult};
use neon::js::{JsNumber, Object, JsUndefined, JsArray};
use neon::js::binary::JsBuffer;
use neon::mem::Managed;
use neon_runtime::buffer;
use cslice::{CMutSlice};

use private_box::{encrypt as encrypt_rs, decrypt as decrypt_rs, init as init_rs};

fn init(call: Call) -> JsResult<JsUndefined>{
    init_rs();

    Ok(JsUndefined::new())
}

fn encrypt(call: Call){
    let scope = call.scope;

}


fn decrypt(call: Call)-> JsResult<JsBuffer>{
    let mut cypher_text_checked = call
        .arguments
        .require(call.scope, 0)?
        .check::<JsBuffer>()?;

    let mut cypher_text_vec = Vec::<u8>::new();

    cypher_text_checked.grab(|contents|{
        let slice = &contents.as_slice();
        cypher_text_vec.extend_from_slice(slice);
    });


    let mut secret_key_vec = Vec::<u8>::with_capacity(SECRETKEYBYTES);
    
    let mut secret_key_checked = call
        .arguments
        .require(call.scope, 1)?
        .check::<JsBuffer>()?;

    let mut secret_key_array: [u8; SECRETKEYBYTES] = [0; SECRETKEYBYTES];

    secret_key_checked.grab(|contents|{
        let slice = &contents.as_slice();

        for i in 0..slice.len(){
            secret_key_array[i] = slice[i]; 
        }
    });

    let secret_key = SecretKey(secret_key_array);
    
    //TODO: remove unwrap and make whole function return undefined if no result.
    let mut plain_text = decrypt_rs(&cypher_text_vec, &secret_key).unwrap();
    let mut result_buffer = JsBuffer::new(call.scope, plain_text.len() as u32).unwrap();

    result_buffer.grab(|mut contents|{
        let slice = &contents.as_slice();

        for i in 0..slice.len(){
            contents[i] = plain_text[i]; 
        }
    });

    Ok(result_buffer)

}

register_module!(m, {
    m.export("decrypt", decrypt);
    m.export("init", init)
});
