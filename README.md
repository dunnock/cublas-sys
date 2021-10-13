# CUBLAS Bindings

This crate was auto-generated using [bindgen](https://github.com/crabtw/rust-bindgen).

The library will search for dependencies in the following order:

1. `CUBLAS_LIB_DIR` and `CUBLAS_INCLUDE_DIR`
2. `${CUDA_PATH}/lib64` or `${CUDA_PATH}/lib/x64` for libraries, and `${CUDA_PATH}/include` for headers
3. `/usr/local/cuda/lib64` or `/usr/local/cuda/x64` for libraries, and `/usr/local/cuda/include` for headers
4. Using the `pkg-config` library for `cuda`, `cudart` or `cublas` packages

List of libraries for linking can be customized via CUBLAS_LIBS environment variable,
by default taking `cublas.lib`

Include file is defined in wrapper.h and is `<cublas_v2.h>`.

## Static linking

If `CUDA_STATIC` is set then the library will try to link statically.
