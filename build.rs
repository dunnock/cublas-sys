use std::path::PathBuf;

fn read_env_path(path: &str) -> Option<PathBuf> {
    std::env::var(path).ok().map(|p| PathBuf::from(&p))
}

fn main() {
    let mut lib_dir = read_env_path("CUBLAS_LIB_DIR");
    let mut inc_dir = read_env_path("CUBLAS_INCLUDE_DIR");
    let cuda_path = read_env_path("CUDA_PATH").or_else(|| Some(PathBuf::from("/usr/local/cuda")));

    if let Some(path) = cuda_path {
        if path.is_dir() {
            let possible_lib_dir = if path.join("lib64").is_dir() {
                path.join("lib64")
            } else {
                path.join("lib").join("x64")
            };
            let possible_inc_dir = path.join("include");

            if possible_lib_dir.is_dir() && possible_inc_dir.is_dir() {
                lib_dir = lib_dir.or(Some(possible_lib_dir));
                inc_dir = inc_dir.or(Some(possible_inc_dir));
            }
        }
    }

    // If all else fails, try looking through `pkg-config`
    if inc_dir.is_none() {
        for package in ["cuda", "cudart", "cublas"] {
            if let Ok(pkg) = pkg_config::probe_library(package) {
                assert!(pkg.link_paths.len() == 1);
                assert!(pkg.include_paths.len() == 1);
                lib_dir = Some(pkg.link_paths[0].clone());
                inc_dir = Some(pkg.include_paths[0].clone());
                break;
            }
        }
    }

    // Hopefully by this point we have it all figured out...
    if let (Some(inc_dir), Some(lib_dir)) = (&inc_dir, lib_dir) {
        println!("cargo:include={}", inc_dir.to_str().unwrap());
        println!(
            "cargo:rustc-link-search=native={}",
            lib_dir.to_str().unwrap()
        );
    } else {
        panic!("Unable to find CUDA libraries");
    }

    //-------------------------------------------------------------------------

    let libs_env = std::env::var("CUBLAS_LIBS");
    let libs = match libs_env {
        Ok(ref v) => v.split(':').collect(),
        Err(_) => vec!["cublas"],
    };

    let mode = if std::env::var_os("CUDA_STATIC").is_some() {
        "static"
    } else {
        "dylib"
    };

    for lib in libs {
        println!("cargo:rustc-link-lib={}={}", mode, lib);
    }

    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "generate")]
    {
        let bindings = bindgen::Builder::default()
            .rust_target(bindgen::RustTarget::Stable_1_40)
            .raw_line(
                r"
//! Defines the FFI for CUDA cuBLAS.
//!
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
            ",
            )
            .ctypes_prefix("::libc")
            .size_t_is_usize(true)
            .clang_arg("-I")
            .clang_arg(inc_dir.unwrap().to_str().unwrap())
            .header("wrapper.h")
            .rustified_non_exhaustive_enum("cublas[A-Za-z]+_t")
            .rustified_non_exhaustive_enum("cuda.*")
            .whitelist_function("cu.*")
            .whitelist_var("CUBLAS.*")
            .whitelist_type("[Cc][Uu].*")
            .default_alias_style(bindgen::AliasVariation::TypeAlias)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .rustfmt_bindings(true)
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from("src").join("generated.rs");
        bindings
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");
    }
}
