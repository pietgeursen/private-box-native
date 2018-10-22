#include <node_api.h>

#include <assert.h>
#include <stdio.h>

extern "C" {
  extern napi_value decrypt(napi_env, napi_callback_info);
  extern napi_value decrypt_async(napi_env, napi_callback_info);
  extern void init();
}

#define DECLARE_NAPI_METHOD(name, func)                          \
  { name, 0, func, 0, 0, 0, napi_default, 0 }

napi_value Init(napi_env env, napi_value exports) {
  init(); //Init private box
  napi_status status;
  napi_property_descriptor addDescriptor[] = {DECLARE_NAPI_METHOD("decrypt", decrypt), DECLARE_NAPI_METHOD("decryptAsync", decrypt_async)};
  status = napi_define_properties(env, exports, 2, addDescriptor);
  assert(status == napi_ok);
  return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)
