# CUDA cuBLAS Bindings

This crate was auto-generated using [bindgen](https://github.com/rust-lang/rust-bindgen).

The crate will search for CUDA using the following steps:

1. First check for `CUBLAS_LIB_DIR` and `CUBLAS_INCLUDE_DIR`
2. Check for `CUDA_PATH` - if not defined then use `/usr/local/cuda`, then check:
   * `${CUDA_PATH}/lib64` or `${CUDA_PATH}/lib/x64` for libraries
   * `${CUDA_PATH}/include` for headers
3. Use the `pkg-config` library for `cuda`, `cudart` or `cublas` packages

List of libraries for linking can be customized via CUBLAS_LIBS environment
variable, by default taking `cublas.lib`

If `CUDA_STATIC` is set then the library will try to link statically.

## CUDA Toolkit Installation

The CUDA Toolkit from NVIDIA can be downloaded [here](https://developer.nvidia.com/cuda-downloads).

During the installation of CUDA, there may be versioning conflicts with previous
NVIDIA installations, which manifest themselves the following:

```
The following packages have unmet dependencies:
cuda : Depends: cuda-11-4 (>= 11.4.2) but it is not going to be installed
E: Unable to correct problems, you have held broken packages.
```

According to [this](https://bit.ly/3lBuF7O) you might be able to fix this by
purging all previous NVIDIA installations:

```bash
sudo apt purge "nvidia-*"
sudo apt purge "libnvidia-*"
```

On completion, CUDA will be installed in `/usr/local/cuda` on Linux.
