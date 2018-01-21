#[macro_use]
extern crate neon;
extern crate neon_runtime;
extern crate private_box;
extern crate sodiumoxide;
extern crate cslice;
#[macro_use]
extern crate arrayref;

extern crate futures;
extern crate futures_cpupool;

use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{PublicKey, PUBLICKEYBYTES, SecretKey, SECRETKEYBYTES};

use neon::vm::{Lock, Call, JsResult};
use neon::js::{JsValue, JsUndefined, JsFunction};
use neon::js::binary::JsBuffer;
use neon::scope::Scope;
use neon::task::Task;

use private_box::{encrypt as encrypt_rs, decrypt as decrypt_rs, init as init_rs};

struct DecryptTask {
    cyphertext: Vec<u8>,
    secretkey:  [u8;32]
}

impl Task for DecryptTask {
    type Output = Vec<u8>;  
    type Error = ();
    type JsEvent = JsValue; 

    fn perform(&self) -> Result<Self::Output, Self::Error>{
        let key = SecretKey(self.secretkey);
        decrypt_rs(&self.cyphertext, &key)
    }

    fn complete<'a, T: Scope<'a>>(self, scope: &'a mut T, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent>
    {
        match result {
            Ok(data) => {
                let mut result_buffer = JsBuffer::new(scope, data.len() as u32).unwrap();

                result_buffer.grab(|mut contents|{
                    let slice = &contents.as_slice();

                    for i in 0..slice.len(){
                        contents[i] = data[i]; 
                    }
                });

                Ok(result_buffer.upcast())
            }
            Err(()) => {
                Ok(JsUndefined::new().upcast())
            }
        }
    }
}

fn init(_: Call) -> JsResult<JsUndefined>{
    init_rs();
    Ok(JsUndefined::new())
}

fn encrypt(call: Call){
    let scope = call.scope;
}

fn decrypt_async(call: Call) -> JsResult<JsUndefined> {

    let mut cypher_text_checked = call
        .arguments
        .require(call.scope, 0)?
        .check::<JsBuffer>()?;

    let mut secret_key_checked = call
        .arguments
        .require(call.scope, 1)?
        .check::<JsBuffer>()?;

    let callback = call
        .arguments
        .require(call.scope, 2)?
        .check::<JsFunction>()?;


    let mut cypher_text_vec = Vec::<u8>::new();
    let mut secret_key_array: [u8; SECRETKEYBYTES] = [0; SECRETKEYBYTES];

    cypher_text_checked.grab(|contents|{
        let slice = &contents.as_slice();
        cypher_text_vec.extend_from_slice(slice);
    });

    secret_key_checked.grab(|contents|{
        let slice = &contents.as_slice();

        for i in 0..slice.len(){
            secret_key_array[i] = slice[i]; 
        }
    });

    let decrypt_task = DecryptTask{cyphertext: cypher_text_vec, secretkey: secret_key_array};

    decrypt_task.schedule(callback);

    Ok(JsUndefined::new())
}

fn decrypt(call: Call)-> JsResult<JsValue>{

    let mut cypher_text_checked = call
        .arguments
        .require(call.scope, 0)?
        .check::<JsBuffer>()?;

    let mut secret_key_checked = call
        .arguments
        .require(call.scope, 1)?
        .check::<JsBuffer>()?;

    let mut cypher_text_vec = Vec::<u8>::new();
    let mut result = Err(());

    cypher_text_checked.grab(|contents|{
        let slice = &contents.as_slice();
        cypher_text_vec.extend_from_slice(slice);
    });

    secret_key_checked.grab(|contents|{
        let slice = contents.as_slice();
        let secret_key = SecretKey(*array_ref![slice,0,32]);
        result = decrypt_rs(&cypher_text_vec, &secret_key);
    });
    
    match result {
        Ok(data) => {
            let mut result_buffer = JsBuffer::new(call.scope, data.len() as u32).unwrap();

            result_buffer.grab(|mut contents|{
                let slice = &contents.as_slice();

                for i in 0..slice.len(){
                    contents[i] = data[i]; 
                }
            });

            Ok(result_buffer.upcast())
        }
        Err(()) => {
            Ok(JsUndefined::new().upcast())
        }
    }
    
}

register_module!(m, {
    m.export("decrypt", decrypt);
    m.export("decrypt_async", decrypt_async);
    m.export("init", init)
});
