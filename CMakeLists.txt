cmake_minimum_required(VERSION 3.1)

# Enable ExternalProject CMake module
include(ExternalProject)

# Set default ExternalProject root directory
set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/Rust)

# Add rust_example as a CMake target
ExternalProject_Add(
    private_box_rs
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND cargo build COMMAND cargo build --release
    BINARY_DIR "${CMAKE_SOURCE_DIR}/private-box-rs"
    INSTALL_COMMAND ""
    LOG_BUILD ON)

project(private_box_native VERSION 1.0.0)

include(NodeJS.cmake)
nodejs_init()


add_nodejs_module(${PROJECT_NAME} private-box-native.cc)
add_dependencies(${PROJECT_NAME} private_box_rs)

# Specify Example's link libraries
target_link_libraries(${PROJECT_NAME}
    debug "${CMAKE_SOURCE_DIR}/private-box-rs/target/debug/libprivate_box_native.a"
    optimized "${CMAKE_SOURCE_DIR}/private-box-rs/target/release/libprivate_box_native.a"
    )
