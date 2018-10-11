# private-box-native

Node bindings to private-box-rs

napi todos:
 - [x] expose a c interface to private-box-rs
 - [x] use cbindgen
 - [x] sort out naming, dashes, low dashes
 - [x] get private box rs able to statically link. There's a way to check linked libs ldd?
   - [ ] how to build c deps form cargo. Ideally also supporting use with cross.
   - [ ] need to work out how to pass down project specific vars hopefully in the cargo.toml
   - [ ] it would be good if -native doesn't require sodiumoxide. It should just use re-exported constants from private box rs
 
 - [ ] 
 - [ ] write some c
   - [ ] how to handle incorrect args to the function
 - [ ] set up [cmakejs](https://stackoverflow.com/questions/31162438/how-can-i-build-rust-code-with-a-c-qt-cmake-project)
 - [ ] prebuild --all 
  - [ ] are "flavours" how you set the target triple?
 - [ ] could we use cross somehow? 

WTF is happening?
- pretty confident that statically building private-box-rs is working ok. ldd of the test executable looks good.
- maybe native is not using the generated static lib file?
  - I think it's not. But I don't _think_ that should matter. As long as sodium is statically linked. => not true!
    - private-box-rs needs to be compiled with the env vars set.
- maybe the env vars need to be set everywhere?


libs

node.so -> native.a -> private-box.a -> sodium.a

as soon as one lib is dynamic, all the sub libs need to have PIC because otherwise they'll overlap the address space.


options:

- [ ] node.so -> native.cdylib -> private-box.rlib -> sodium.a
- [ ] node.so -> native.a -> private-box.rlib -> sodium.a
- [ ] node.so -> native.cdylib -> private-box.dylib -> sodium.a
- [ ] node.so -> native.a -> private-box.dylib -> sodium.a

