#include <node_api.h>
#include "private_box.h"

#include <assert.h>
#include <stdio.h>

napi_value decrypt(napi_env env, napi_callback_info info) {
  napi_status status;
  napi_value undefined;
  napi_get_undefined(env, &undefined);

  size_t argc = 2;
  napi_value args[2];
  status = napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);
  assert(status == napi_ok);

  if (argc < 2) {
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
  status = napi_create_reference(env, args[0], 0, &refArg0);
  assert(status == napi_ok);
  status = napi_create_reference(env, args[1], 0, &refArg1);
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
  napi_property_descriptor addDescriptor = DECLARE_NAPI_METHOD("decrypt", decrypt);
  status = napi_define_properties(env, exports, 1, &addDescriptor);
  assert(status == napi_ok);
  return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)
