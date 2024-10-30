use std::env;

fn main()
{
    if cfg!(target_os = "macos")
    {
        // Link against the Accelerate framework on macOS
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }
    else if cfg!(target_os = "linux")
    {
        // Link against OpenBLAS on Linux
        println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
        println!("cargo:rustc-link-lib=dylib=openblas");
    }

    // Print a message for debugging purposes
    let target = env::var("TARGET").unwrap();
    println!("cargo:warning=Building for target: {}", target);
}
