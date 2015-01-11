
use std::default::Default;
mod sndfile {
    extern crate libc;
    type SfCount = i64;
    type SfHandle = libc::size_t;

    #[allow(dead_code)]
    pub static SFM_READ: i32 = 0x10;
    #[allow(dead_code)]
    pub static SFM_WRITE: i32 = 0x20;
    #[allow(dead_code)]
    pub static SFM_RDWR: i32 = 0x30;

    #[repr(C)]
    #[derive(Default)]
    pub struct SfInfo {
        frames: SfCount,
        samplerate: i32,
        channels: i32,
        format: i32,
        sections: i32,
        seekable: i32,
    }

    #[link(name = "sndfile")]
    extern {
        pub fn sf_open(path: *const u8, mode: i32, sfinfo: *mut SfInfo) -> SfHandle;
        pub fn sf_close(sndfile: SfHandle) -> i32;
    }
}

fn main() {
    let mut info: sndfile::SfInfo = Default::default();
    let handle = unsafe {
        sndfile::sf_open("test.wav".as_ptr(), sndfile::SFM_READ, &mut info)
    };
    println!("{}", handle);
    unsafe {
        sndfile::sf_close(handle);
    }
}
