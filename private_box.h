#ifndef private_box_native
#define private_box_native

#include <cstdint>
#include <cstdlib>

extern "C" {

intptr_t decrypt(const uint8_t *p_cypher_text,
                 uintptr_t cypher_text_len,
                 const uint8_t *p_key,
                 uint8_t *p_result_buf,
                 uintptr_t *result_buf_len);

void init();

} // extern "C"

#endif // private_box_native
