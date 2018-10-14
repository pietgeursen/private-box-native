cmake_minimum_required(VERSION 3.1)

# Enable ExternalProject CMake module
include(ExternalProject)

# Set default ExternalProject root directory
set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/Rust)

set(RUST_DIR ${CMAKE_BINARY_DIR}/Rust)
set(SODIUM_INSTALL_DIR ${RUST_DIR}/local)
set(SODIUM_LIB_DIR ${SODIUM_INSTALL_DIR}/lib)
set(SODIUM_STATIC 1)

configure_file("build-native.sh.in" "build-native.sh")

# Add rust_example as a CMake target
ExternalProject_Add(
  native
  BUILD_COMMAND sh "${CMAKE_CURRENT_BINARY_DIR}/build-native.sh"
  BINARY_DIR "${CMAKE_SOURCE_DIR}/native"
  LOG_BUILD ON
  DOWNLOAD_COMMAND ""
  CONFIGURE_COMMAND ""
  INSTALL_COMMAND ""
  )

project(private_box_native VERSION 1.0.0)
add_definitions(-DNAPI_VERSION=3)


include(NodeJS.cmake)
nodejs_init()

add_nodejs_module(${PROJECT_NAME} private-box-native.cc)
add_dependencies(${PROJECT_NAME} native)
target_include_directories(${PROJECT_NAME} PRIVATE "${CMAKE_SOURCE_DIR}/node_modules/node-addon-api")

# Specify Example's link libraries
target_link_libraries(${PROJECT_NAME}
  debug "${CMAKE_SOURCE_DIR}/native/target/debug/libprivate_box_native.so"
  optimized "${CMAKE_SOURCE_DIR}/native/target/release/libprivate_box_native.so"
  )
