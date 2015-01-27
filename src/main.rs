#![allow(unstable)]

extern crate libc;

mod sndfile {
    use libc::{c_int, c_char, uint8_t, uint32_t, int32_t};
    use std::default::Default;
    use std::ffi;
    use std::str;

    type SfCount = i64;
    
    #[repr(C)]
    struct SndFile;

    #[allow(dead_code)]
    pub enum SFM {
        READ = 0x10,
        WRITE = 0x20,
        RDWR = 0x30,
    }

    #[repr(C)]
    #[derive(Default)]
    struct SfInfo {
        frames: SfCount,
        samplerate: i32,
        channels: i32,
        format: i32,
        sections: i32,
        seekable: i32,
    }

    #[link(name = "sndfile")]
    extern {
        fn sf_open(path: *const c_char, mode: int32_t, sfinfo: *mut SfInfo) -> *mut SndFile;
        fn sf_close(sndfile: *mut SndFile) -> int32_t;
        fn sf_strerror(sndfie: *mut SndFile) -> *const c_char;
    }

    #[allow(raw_pointer_derive)]
    #[derive(Show)]
    pub struct File {
        handle: *mut SndFile,
        path: String,
    }
    
    impl Drop for File {
        fn drop(&mut self) {
            unsafe {
                sf_close(self.handle);
            }
        }
    }

    impl File {
        pub fn open(path: &str, mode: SFM) -> File {
            let mut info: SfInfo = Default::default();
            let mut handle: *mut SndFile;
            unsafe {
                handle = sf_open(ffi::CString::from_slice(path.as_bytes()).as_ptr(), mode as i32, &mut info);
                if handle.is_null() {
                    let errstr = sf_strerror(handle);
                    panic!("error: {}", str::from_utf8(ffi::c_str_to_bytes(&errstr)).unwrap());
                }
            }
            File { handle: handle,
                   path: String::from_str(path),
            }
        }
    }
}

fn main() {
    let sound = sndfile::File::open("test.wav", sndfile::SFM::READ);
    println!("{}", sound);
}
