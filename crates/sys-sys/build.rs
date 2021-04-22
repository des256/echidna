fn main() {    
    println!("cargo:rustc-link-lib=xcb");
    println!("cargo:rustc-link-lib=vulkan");
}
