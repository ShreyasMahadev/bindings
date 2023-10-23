use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use bindgen;

fn generate_bindings(headers: &[&str], clang_args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = bindgen::Builder::default();
    
    for header in headers {
        builder = builder.header(*header);
    }

    let bindings = builder
        .clang_args(clang_args)        
        .layout_tests(false)
        .use_core()
        .ctypes_prefix("cty")
        .generate()?;

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}

fn main() {
    // Specify the STM32G431CBUx_FLASH.ld linker script
    //println!("cargo:rustc-link-arg=-Tmemory.x");

    // Specify the path to the directory containing libstm32.a
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-search=native={}", env::current_dir().unwrap().display());       
    println!("cargo:rustc-link-lib=static=stm32");
   
    let headers = vec![
        "c_code/Core/Inc/mainc.h",
        // ... any other headers you want directly included
    ];

    let clang_paths = vec![
        "-Ic_code/Core/Inc",
        "-Ic_code/Drivers/CMSIS/Core/Include",
        "-Ic_code/Drivers/CMSIS/Device/ST/STM32G4xx/Include",
        "-Ic_code/Drivers/CMSIS/Include",
        "-Ic_code/Drivers/CMSIS/DSP/DSP_Lib_TestSuite/Common/inc",
        "-Ic_code/Drivers/CMSIS/DSP/Include",
        "-Ic_code/Drivers/CMSIS/NN/Include",
        "-Ic_code/Drivers/CMSIS/RTOS/Template",
        "-Ic_code/Drivers/CMSIS/RTOS2/Include",
        "-Ic_code/Drivers/STM32G4xx_HAL_Driver/Inc",
        "-Ic_code/Drivers/STM32G4xx_HAL_Driver/Inc/Legacy",        
    ];
    
    generate_bindings(&headers, &clang_paths).unwrap();
}

