use std::io::Command;

fn main() {
    // let sndfileflags = match Command::new("pkg-config").arg("--libs").arg("sndfile").output() {
    //     Ok(output) => output,
    //     Err(e) => panic!("failed to get lib flags for sndfile. {}", e),
    // };
    // println!("cargo:rustc-flags={}", String::from_utf8_lossy(sndfileflags.output.as_slice()));
}
