{
  "targets": [
  {
    "target_name": "binding",
      "sources": [ "private-box-native.cc" ],
      "copies": [{
        "destination": "<(module_root_dir)/build/Release/",
        "files": ["<(module_root_dir)/native/target/release/libprivate_box_native.so"]
      }],
      "link_settings": {
        "ldflags": ["-Wl,-R,'$$ORIGIN/libprivate_box_native.so'", "-Wl,-z,origin"],
        "libraries": ["<(module_root_dir)/build/Release/libprivate_box_native.so"]
      },
      "defines": [
        "NAPI_VERSION=<(napi_build_version)",
      ],
  }
  ]
}
