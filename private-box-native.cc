#include <node_api.h>
#include "private_box.h"

#include <assert.h>
#include <stdio.h>

//  intptr_t decrytErrorCode = decrypt((const uint8_t *)cypher, cypherLen, (const uint8_t *)sk, (uint8_t *)result, &resultLen);


struct DecryptContext {
  const uint8_t *p_cypher_text;
  uintptr_t cypher_text_len;
  const uint8_t *p_key;
  uint8_t *p_result_buf;
  uintptr_t result_buf_len;
  intptr_t result_code;
  napi_ref refArg0;
  napi_ref refArg1;
  napi_ref refResultBuffer;
  napi_ref refCompletionCallback;
};

void execute_decrypt(napi_env env, void* data){
  DecryptContext* ctx = (DecryptContext*) data;

  //do the decryption. If decrypt returns 0 then it decrypted something.
  ctx->result_code = decrypt(
     ctx->p_cypher_text, 
     ctx->cypher_text_len,
     ctx->p_key,
     ctx->p_result_buf,
     &ctx->result_buf_len
     );
}

void complete_decrypt(napi_env env, napi_status status, void* data){
  napi_value null;
  napi_get_undefined(env, &null);

  DecryptContext* ctx = (DecryptContext*) data;

  //All this just to slice the buffer...
  napi_value resultBuffer;
  status = napi_get_reference_value(env, ctx->refResultBuffer, &resultBuffer);
  assert(status == napi_ok);

  napi_value sliceFn;
  status = napi_get_named_property(env, resultBuffer, "slice", &sliceFn );
  assert(status == napi_ok);

  napi_value arg0, arg1;
  napi_create_int32(env, 0, &arg0);
  napi_create_int32(env, ctx->result_buf_len, &arg1);

  napi_value sliceArgs[2] = {arg0, arg1};

  napi_value resultSlice;

  status = napi_call_function(env, resultBuffer, sliceFn, 2, sliceArgs, &resultSlice);
  assert(status == napi_ok);


  napi_value completionCallback;
  status = napi_get_reference_value(env, ctx->refCompletionCallback, &completionCallback);
  assert(status == napi_ok);

  napi_value completionArgs[] = {null, ctx->result_code == 0 ? resultSlice : null};
  napi_value global, cbResult;
  status = napi_get_global(env, &global);
  assert(status == napi_ok);

  status = napi_call_function(env, global, completionCallback, 2, completionArgs, &cbResult);
  assert(status == napi_ok);
  //clean up context object
  
  //Delete refs to input ctx.
  status = napi_delete_reference(env, ctx->refArg0);
  assert(status == napi_ok);
  status = napi_delete_reference(env, ctx->refArg1);
  assert(status == napi_ok);
  status = napi_delete_reference(env, ctx->refResultBuffer);
  assert(status == napi_ok);
  delete ctx;
}

napi_value decrypt_async(napi_env env, napi_callback_info info) {
  DecryptContext* ctx = new DecryptContext;

  napi_status status;
  napi_value undefined;
  napi_get_undefined(env, &undefined);

  size_t argc = 3;
  napi_value args[3];
  status = napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
  assert(status == napi_ok);

  if (argc != 3) {
    napi_throw_type_error(env, nullptr, "Wrong number of arguments, expected 3 args.");
    return nullptr;
  }

  //check arg0 is a buffer
  bool isArg0Buffer;  
  bool isArg1Buffer;  
  status = napi_is_buffer(env, args[0], &isArg0Buffer);
  assert(status == napi_ok);

  status = napi_is_buffer(env, args[1], &isArg1Buffer);
  assert(status == napi_ok);

  if (!isArg0Buffer || !isArg1Buffer) {
    napi_throw_type_error(env, nullptr, "Expected 2 buffers and a callback.");
    return nullptr;
  }

  //use napi_create_reference to stop gc fucking with the underlying buffer.
  status = napi_create_reference(env, args[0], 1, &ctx->refArg0);
  assert(status == napi_ok);
  status = napi_create_reference(env, args[1], 1, &ctx->refArg1);
  assert(status == napi_ok);
  status = napi_create_reference(env, args[2], 1, &ctx->refCompletionCallback);
  assert(status == napi_ok);

  //get pointers to start of buffers. 
  void * cypher;
  void * sk;
  size_t cypherLen;
  size_t skLen;
  status = napi_get_buffer_info(env, args[0], &cypher, &cypherLen);
  assert(status == napi_ok);

  status = napi_get_buffer_info(env, args[1], &sk, &skLen);
  assert(status == napi_ok);

  //make a new buffer the same size as the cypher text.
  napi_value resultBuffer;
  void * result;
  napi_create_buffer(env, cypherLen, &result, &resultBuffer);

  status = napi_create_reference(env, resultBuffer, 1, &ctx->refResultBuffer);
  assert(status == napi_ok);

  ctx->p_result_buf = (uint8_t *) result;
  ctx->p_cypher_text = (const uint8_t *) cypher;
  ctx->cypher_text_len = cypherLen;
  ctx->p_key = (const uint8_t *) sk;

  napi_value workName;
  const char * workNameString = "private_box_decrypt_async_task";

  napi_create_string_utf8(env, workNameString, sizeof(workNameString), &workName);
  napi_async_work work;
  status = napi_create_async_work(env, nullptr, workName, execute_decrypt, complete_decrypt, (void *) ctx, &work);
  assert(status == napi_ok);

  status = napi_queue_async_work(env, work);
  assert(status == napi_ok);

  return undefined;
}

napi_value decrypt(napi_env env, napi_callback_info info) {
  napi_status status;
  napi_value undefined;
  napi_get_undefined(env, &undefined);

  size_t argc = 2;
  napi_value args[2];
  status = napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
  assert(status == napi_ok);

  if (argc != 2) {
    napi_throw_type_error(env, nullptr, "Wrong number of arguments, expected 2 args.");
    return nullptr;
  }

  //check arg0 is a buffer
  bool isArg0Buffer;  
  bool isArg1Buffer;  
  status = napi_is_buffer(env, args[0], &isArg0Buffer);
  assert(status == napi_ok);

  status = napi_is_buffer(env, args[1], &isArg1Buffer);
  assert(status == napi_ok);

  if (!isArg0Buffer || !isArg1Buffer) {
    napi_throw_type_error(env, nullptr, "Expected args to be buffers");
    return nullptr;
  }
  
  //use napi_create_reference to stop gc fucking with the underlying buffer.
  //not sure if this is needed. But better safe than sorry.
  napi_ref refArg0;
  napi_ref refArg1;
  status = napi_create_reference(env, args[0], 1, &refArg0);
  assert(status == napi_ok);
  status = napi_create_reference(env, args[1], 1, &refArg1);
  assert(status == napi_ok);

  //get pointers to start of buffers. 
  void * cypher;
  void * sk;
  size_t cypherLen;
  size_t skLen;
  status = napi_get_buffer_info(env, args[0], &cypher, &cypherLen);
  assert(status == napi_ok);

  status = napi_get_buffer_info(env, args[1], &sk, &skLen);
  assert(status == napi_ok);

  //make a new buffer the same size as the cypher text.
  napi_value resultBuffer;
  void * result;
  napi_create_buffer(env, cypherLen, &result, &resultBuffer);

  //do the decryption. If decrypt returns 0 then it decrypted something.
  size_t resultLen;
  intptr_t decrytErrorCode = decrypt((const uint8_t *)cypher, cypherLen, (const uint8_t *)sk, (uint8_t *)result, &resultLen);
  
  //Delete refs to input args.
  status = napi_delete_reference(env, refArg0);
  assert(status == napi_ok);
  status = napi_delete_reference(env, refArg1);
  assert(status == napi_ok);

  //All this just to slice the buffer...
  napi_value sliceFn;
  status = napi_get_named_property(env, resultBuffer, "slice", &sliceFn );
  assert(status == napi_ok);

  napi_value arg0, arg1;
  napi_create_int32(env, 0, &arg0);
  napi_create_int32(env, resultLen, &arg1);

  napi_value sliceArgs[2] = {arg0, arg1};

  napi_value resultSlice;
  status = napi_call_function(env, resultBuffer, sliceFn, 2, sliceArgs, &resultSlice);
  assert(status == napi_ok);

  return decrytErrorCode == 0 ? resultSlice : undefined;
}

#define DECLARE_NAPI_METHOD(name, func)                          \
  { name, 0, func, 0, 0, 0, napi_default, 0 }

napi_value Init(napi_env env, napi_value exports) {
  init(); //Init private box
  napi_status status;
  napi_property_descriptor descriptors[] = {DECLARE_NAPI_METHOD("decryptAsync", decrypt_async), DECLARE_NAPI_METHOD("decrypt", decrypt)};
  status = napi_define_properties(env, exports, 2, descriptors);
  assert(status == napi_ok);
  return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)
