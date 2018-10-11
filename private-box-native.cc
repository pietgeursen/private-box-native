#include <nan.h>
#include "private-box.h"

namespace demo {

NAN_METHOD(hello) {
  const uint8_t cypher[64] = {};
  const uint8_t key[64] = {};
  uint8_t result[64] = {};
  decrypt(cypher, 64, key, result, 64);
  info.GetReturnValue().Set(Nan::New("world").ToLocalChecked());
}

NAN_MODULE_INIT(init) {
    NAN_EXPORT(target, hello);
}

NODE_MODULE(MODULE_NAME, init)

} // namespace demo
