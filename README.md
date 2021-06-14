The file `src/generated.rs` was created with [bindgen](https://github.com/crabtw/rust-bindgen) using `build.rs`

CUDA BLAS bindings

# Cublas include files and library files located by following proirities:

1. CUBLAS_LIB_DIR and CUBLAS_INCLUDE_DIR
2. CUDA_PATH with suffixes:
- `lib64` or `lib/x64` suffix for library 
- `include` suffix for headers 
3. pkg_config for `cuda`, `cudart` or `cublas` packages

List of libraries for linking can be customized via CUBLAS_LIBS env var, 
by default taking `cublas.lib`

Include file is defined in wrapper.h and is `<cublas_v2.h>`.

## Static linking

When CUDA_STATIC parameter is set bindgen will try to link statically.





