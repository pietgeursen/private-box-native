# private-box-native

Node bindings to private-box-rs

napi todos:
 - [x] expose a c interface to private-box-rs
 - [x] use cbindgen
 - [x] sort out naming, dashes, low dashes
 - [x] get private box rs able to statically link. There's a way to check linked libs ldd?
   - [x] how to build c deps form cargo. Ideally also supporting use with cross.
   - [x] need to work out how to pass down project specific vars hopefully in the cargo.toml
   - [x] it would be good if -native doesn't require sodiumoxide. It should just use re-exported constants from private box rs

 - [x] write some c
 - [x] get a test passing
 
   - [ ] how to handle incorrect args to the function
 - [x] set up [cmakejs](https://stackoverflow.com/questions/31162438/how-can-i-build-rust-code-with-a-c-qt-cmake-project)
 - [ ] prebuild --all 
  - [ ] are "flavours" how you set the target triple?
 - [ ] could we use cross somehow? 

 What did I learn?
 - cross isn't _that_ easy to understand. Or rather, I need to study it for longer.
  - It's not clear to me how / when qemu gets loaded.

