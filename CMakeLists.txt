cmake_minimum_required(VERSION 3.1)

# Enable ExternalProject CMake module
include(ExternalProject)

# Set default ExternalProject root directory
set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/Rust)

set(RUST_DIR ${CMAKE_BINARY_DIR}/Rust)
set(SODIUM_INSTALL_DIR ${RUST_DIR}/local)
set(SODIUM_LIB_DIR ${SODIUM_INSTALL_DIR}/lib)
set(SODIUM_STATIC 1)

set(ENV{SODIUM_INSTALL_DIR} ${SODIUM_INSTALL_DIR})
set(ENV{SODIUM_LIB_DIR} ${SODIUM_LIB_DIR})
set(ENV{SODIUM_STATIC} ${SODIUM_STATIC})

message( STATUS ${RUST_DIR} )

message( STATUS $ENV{SODIUM_INSTALL_DIR} )
message( STATUS $ENV{SODIUM_LIB_DIR} )
message( STATUS $ENV{SODIUM_STATIC} )

configure_file("build-native.sh.in" "build-native.sh")

# Add rust_example as a CMake target
ExternalProject_Add(
    native
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    #BUILD_COMMAND SODIUM_INSTALL_DIR=${CMAKE_BINARY_DIR}/Rust/local SODIUM_LIB_DIR=${CMAKE_BINARY_DIR}/Rust/local SODIUM_STATIC=1 cargo build COMMAND SODIUM_INSTALL_DIR=${CMAKE_BINARY_DIR}/Rust/local SODIUM_LIB_DIR=${CMAKE_BINARY_DIR}/Rust/local SODIUM_STATIC=1 cargo build --release
    BUILD_COMMAND sh "${CMAKE_CURRENT_BINARY_DIR}/build-native.sh"
    BINARY_DIR "${CMAKE_SOURCE_DIR}/native"
    INSTALL_COMMAND ""
    LOG_BUILD ON)

project(private_box_native VERSION 1.0.0)

include(NodeJS.cmake)
nodejs_init()


add_nodejs_module(${PROJECT_NAME} private-box-native.cc)
add_dependencies(${PROJECT_NAME} native)

# Specify Example's link libraries
target_link_libraries(${PROJECT_NAME}
    debug "${CMAKE_SOURCE_DIR}/native/target/debug/libprivate_box_native.a"
    optimized "${CMAKE_SOURCE_DIR}/native/target/release/libprivate_box_native.a"
    )
