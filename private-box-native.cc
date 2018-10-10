#include <nan.h>
#include "private_box.h"

namespace demo {

NAN_METHOD(hello) {
  const uint8_t cypher[32] = {};
  const uint8_t key[32] = {};
  uint8_t result[32] = {};
  decrypt(cypher, 32, key, result, 32);
  info.GetReturnValue().Set(Nan::New("world").ToLocalChecked());
}

NAN_MODULE_INIT(init) {
    NAN_EXPORT(target, hello);
}

NODE_MODULE(MODULE_NAME, init)

} // namespace demo
