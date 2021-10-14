# CUDA cuBLAS Bindings

This crate was auto-generated using [bindgen](https://github.com/rust-lang/rust-bindgen).

The library will search for dependencies in the following order:

1. `CUBLAS_LIB_DIR` and `CUBLAS_INCLUDE_DIR`
2. `${CUDA_PATH}/lib64` or `${CUDA_PATH}/lib/x64` for libraries, and `${CUDA_PATH}/include` for headers
3. `/usr/local/cuda/lib64` or `/usr/local/cuda/x64` for libraries, and `/usr/local/cuda/include` for headers
4. Using the `pkg-config` library for `cuda`, `cudart` or `cublas` packages

List of libraries for linking can be customized via CUBLAS_LIBS environment variable,
by default taking `cublas.lib`

Include file is defined in wrapper.h and is `<cublas_v2.h>`.

## CUDA Toolkit Installation

The CUDA Toolkit from NVIDIA can be downloaded [here](https://developer.nvidia.com/cuda-downloads).

During the installation of CUDA, there may be versioning conflicts with previous NVIDIA installations, which manifest themselves with errors such as:

```
The following packages have unmet dependencies:
cuda : Depends: cuda-11-4 (>= 11.4.2) but it is not going to be installed
E: Unable to correct problems, you have held broken packages.
```

According to [this](https://bit.ly/3lBuF7O) you might be able to fix this by purging all previous NVIDIA installations:

```bash
sudo apt purge "nvidia-*"
sudo apt purge "libnvidia-*"
```

On completion, CUDA will be installed in `/usr/local/cuda` on Linux.

## Static linking

If `CUDA_STATIC` is set then the library will try to link statically.
