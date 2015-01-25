
mod sndfile {
    extern crate libc;
    use std::default::Default;
    type SfCount = i64;
    struct SndFile;

    enum SFM {
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
        fn sf_open(path: *const u8, mode: i32, sfinfo: *mut SfInfo) -> *mut SndFile;
        fn sf_close(sndfile: *mut SndFile) -> i32;
    }

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
        pub fn OpenRead(path: &str) -> File {
            let mut info: SfInfo = Default::default();
            File { handle: unsafe { sf_open(path.as_ptr(), SFM::READ as i32, &mut info) },
                   path: String::from_str(path),
            }
        }

        pub fn OpenWrite(path: &str) -> File {
            let mut info: SfInfo = Default::default();
            File { handle: unsafe { sf_open(path.as_ptr(), SFM::WRITE as i32, &mut info) },
                   path: String::from_str(path),
            }
        }

    }
}

fn main() {
    let sound = sndfile::File::OpenRead("test.wav");
    println!("{}", sound);
}
