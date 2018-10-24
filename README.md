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
 
   - [x] how to handle incorrect args to the function
 - [x] set up [cmakejs](https://stackoverflow.com/questions/31162438/how-can-i-build-rust-code-with-a-c-qt-cmake-project)
 - [ ] prebuild --all 
  - [ ] are "flavours" how you set the target triple?
 - [ ] could we use cross somehow? 

Get cross building everything

- use cargo make? Doesn't really solve how to do cross compile
  - need to choose a target triple, say arm64
  - sodium build happens first. gets called with the dockcross (linux-arm64)
  - rust build happens with cross (aarch64-unknown-linux-gnu)
  - binding build happens with dock cross (linux-arm64)

- Hypothesis: That cross can't use absolute paths at all. This would match maybe with dockercross.
  - How to test? 
    - see if dockercross can ls an absolute path => It can't.
    - make a hello world cross lib and see if ls works in the build rs.
  - How to get around?
    - make my own libsodium-sys, which would give me control of the build.rs and in there I could try and pass a relative path to the rustc-link-path. If that even works.
  



