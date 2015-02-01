#![allow(unstable)]

extern crate libc;
use std::os;

mod sndfile {
    use libc::{c_char, int32_t, c_float};
    use std::default::Default;
    use std::ffi;
    use std::str;
    use std::vec::Vec;

    type SfCount = i64;
    
    #[repr(C)]
    struct SndFile;

    #[allow(dead_code)]
    pub enum SFM {
        READ = 0x10,
        WRITE = 0x20,
        RDWR = 0x30,
    }

    #[allow(dead_code)]
	const SF_FORMAT_WAV: i32			= 0x010000;		/* Microsoft WAV format (little endian default). */
    #[allow(dead_code)]
	const SF_FORMAT_AIFF: i32			= 0x020000;		/* Apple/SGI AIFF format (big endian). */
	#[allow(dead_code)]
    const SF_FORMAT_AU: i32			= 0x030000;		/* Sun/NeXT AU format (big endian). */
	#[allow(dead_code)]
    const SF_FORMAT_RAW: i32			= 0x040000;		/* RAW PCM data. */
	#[allow(dead_code)]
    const SF_FORMAT_PAF: i32			= 0x050000;		/* Ensoniq PARIS file format. */
	#[allow(dead_code)]
    const SF_FORMAT_SVX: i32			= 0x060000;		/* Amiga IFF / SVX8 / SV16 format. */
	#[allow(dead_code)]
    const SF_FORMAT_NIST: i32			= 0x070000;		/* Sphere NIST format. */
	#[allow(dead_code)]
    const SF_FORMAT_VOC: i32			= 0x080000;		/* VOC files. */
	#[allow(dead_code)]
    const SF_FORMAT_IRCAM: i32			= 0x0A0000;		/* Berkeley/IRCAM/CARL */
	#[allow(dead_code)]
    const SF_FORMAT_W64: i32			= 0x0B0000;		/* Sonic Foundry's 64 bit RIFF/WAV */
	#[allow(dead_code)]
    const SF_FORMAT_MAT4: i32			= 0x0C0000;		/* Matlab (tm) V4.2 / GNU Octave 2.0 */
	#[allow(dead_code)]
    const SF_FORMAT_MAT5: i32			= 0x0D0000;		/* Matlab (tm) V5.0 / GNU Octave 2.1 */
	#[allow(dead_code)]
    const SF_FORMAT_PVF: i32			= 0x0E0000;		/* Portable Voice Format */
	#[allow(dead_code)]
    const SF_FORMAT_XI: i32			= 0x0F0000;		/* Fasttracker 2 Extended Instrument */
	#[allow(dead_code)]
    const SF_FORMAT_HTK: i32			= 0x100000;		/* HMM Tool Kit format */
	#[allow(dead_code)]
    const SF_FORMAT_SDS: i32			= 0x110000;		/* Midi Sample Dump Standard */
	#[allow(dead_code)]
    const SF_FORMAT_AVR: i32			= 0x120000;		/* Audio Visual Research */
    #[allow(dead_code)]	
    const SF_FORMAT_WAVEX: i32			= 0x130000;		/* MS WAVE with WAVEFORMATEX */
	#[allow(dead_code)]
    const SF_FORMAT_SD2: i32			= 0x160000;		/* Sound Designer 2 */
	#[allow(dead_code)]
    const SF_FORMAT_FLAC: i32			= 0x170000;		/* FLAC lossless file format */
	#[allow(dead_code)]
    const SF_FORMAT_CAF: i32			= 0x180000;		/* Core Audio File format */
	#[allow(dead_code)]
    const SF_FORMAT_WVE: i32			= 0x190000;		/* Psion WVE format */
	#[allow(dead_code)]
    const SF_FORMAT_OGG: i32			= 0x200000;		/* Xiph OGG container */
	#[allow(dead_code)]
    const SF_FORMAT_MPC2K: i32			= 0x210000;		/* Akai MPC 2000 sampler */
	#[allow(dead_code)]
    const SF_FORMAT_RF64: i32			= 0x220000;		/* RF64 WAV file */
    
	/* Subtypes from here on. */

	#[allow(dead_code)]
    const SF_FORMAT_PCM_S8: i32		= 0x0001;		/* Signed 8 bit data */
	#[allow(dead_code)]
    const SF_FORMAT_PCM_16: i32		= 0x0002;		/* Signed 16 bit data */
	#[allow(dead_code)]
    const SF_FORMAT_PCM_24: i32		= 0x0003;		/* Signed 24 bit data */
	#[allow(dead_code)]
    const SF_FORMAT_PCM_32: i32		= 0x0004;		/* Signed 32 bit data */
    
    #[allow(dead_code)]	
    const SF_FORMAT_PCM_U8: i32		= 0x0005;		/* Unsigned 8 bit data (WAV and RAW only) */
    
    #[allow(dead_code)]	
    const SF_FORMAT_FLOAT: i32			= 0x0006;		/* 32 bit float data */
	#[allow(dead_code)]
    const SF_FORMAT_DOUBLE: i32		= 0x0007;		/* 64 bit float data */
    
	#[allow(dead_code)]
    const SF_FORMAT_ULAW: i32			= 0x0010;		/* U-Law encoded. */
	#[allow(dead_code)]
    const SF_FORMAT_ALAW: i32			= 0x0011;		/* A-Law encoded. */
	#[allow(dead_code)]
    const SF_FORMAT_IMA_ADPCM: i32		= 0x0012;		/* IMA ADPCM. */
	#[allow(dead_code)]
    const SF_FORMAT_MS_ADPCM: i32		= 0x0013;		/* Microsoft ADPCM. */
    
	#[allow(dead_code)]
    const SF_FORMAT_GSM610: i32		= 0x0020;		/* GSM 6.10 encoding. */
	#[allow(dead_code)]
    const SF_FORMAT_VOX_ADPCM: i32		= 0x0021;		/* OKI / Dialogix ADPCM */
    
	#[allow(dead_code)]
    const SF_FORMAT_G721_32: i32		= 0x0030;		/* 32kbs G721 ADPCM encoding. */
	#[allow(dead_code)]
    const SF_FORMAT_G723_24: i32		= 0x0031;		/* 24kbs G723 ADPCM encoding. */
	#[allow(dead_code)]
    const SF_FORMAT_G723_40: i32		= 0x0032;		/* 40kbs G723 ADPCM encoding. */

	#[allow(dead_code)]
    const SF_FORMAT_DWVW_12: i32		= 0x0040; 		/* 12 bit Delta Width Variable Word encoding. */
    #[allow(dead_code)]	
    const SF_FORMAT_DWVW_16: i32		= 0x0041; 		/* 16 bit Delta Width Variable Word encoding. */
	#[allow(dead_code)]
    const SF_FORMAT_DWVW_24: i32		= 0x0042; 		/* 24 bit Delta Width Variable Word encoding. */
	#[allow(dead_code)]
    const SF_FORMAT_DWVW_N: i32		= 0x0043; 		/* N bit Delta Width Variable Word encoding. */
    
	#[allow(dead_code)]
    const SF_FORMAT_DPCM_8: i32		= 0x0050;		/* 8 bit differential PCM (XI only) */
	#[allow(dead_code)]
    const SF_FORMAT_DPCM_16: i32		= 0x0051;		/* 16 bit differential PCM (XI only) */
    
	#[allow(dead_code)]
    const SF_FORMAT_VORBIS: i32		= 0x0060;		/* Xiph Vorbis encoding. */
    
	/* Endian-ness options. */

	#[allow(dead_code)]
    const SF_ENDIAN_FILE: i32			= 0x00000000;	/* Default file endian-ness. */
	#[allow(dead_code)]
    const SF_ENDIAN_LITTLE: i32		= 0x10000000;	/* Force little endian-ness. */
	#[allow(dead_code)]
    const SF_ENDIAN_BIG: i32			= 0x20000000;	/* Force big endian-ness. */
	#[allow(dead_code)]
    const SF_ENDIAN_CPU: i32			= 0x30000000;	/* Force CPU endian-ness. */

    #[allow(dead_code)]	
    const SF_FORMAT_SUBMASK: i32		= 0x0000FFFF;
    #[allow(dead_code)]	
    const SF_FORMAT_TYPEMASK: i32		= 0x0FFF0000;
	#[allow(dead_code)]
    const SF_FORMAT_ENDMASK: i32		= 0x30000000;

    #[repr(C)]
    #[derive(Default)]
    #[derive(Show)]
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
        fn sf_strerror(sndfile: *mut SndFile) -> *const c_char;
        fn sf_read_float(sndfile: *mut SndFile, ptr: *mut c_float, samples: SfCount) -> SfCount;
        fn sf_write_float(sndfile: *mut SndFile, ptr: *const c_float, samples: SfCount) -> SfCount;
    }

    #[allow(raw_pointer_derive)]
    #[derive(Show)]
    pub struct File {
        handle: *mut SndFile,
        path: String,
        info: SfInfo,
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
            let mut info: SfInfo = match mode {
                SFM::READ => { Default::default() }
                SFM::WRITE | SFM::RDWR => SfInfo {
                    frames: 0,
                    samplerate: 44100,
                    channels: 1,
                    format: SF_FORMAT_WAV | SF_FORMAT_PCM_16,
                    sections: 1, // ?
                    seekable: 1,
                }
            };
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
                   info: info,
            }
        }

        pub fn channels(& self) -> i32 {
            return self.info.channels;
        }

        pub fn read_everything(&mut self) -> Vec<f32> {
            let mut outvec: Vec<f32> = Vec::new();
            loop {
                let oldsize = outvec.len();
                outvec.resize(oldsize + 64, 0f32);
                let read;
                unsafe {
                    let buffer = outvec.as_mut_ptr().offset(oldsize as int);
                    read = sf_read_float(self.handle, buffer, 64);
                }
                if read < 64 {
                    outvec.truncate(oldsize + read as uint);
                    break;
                }
            }
            outvec
        }

        pub fn write(&mut self, data: &[f32]) {
            let datalen = data.len() as i64;
            let written = unsafe { sf_write_float(self.handle, data.as_ptr(), datalen) };
            if written != datalen {
                panic!("error writing to file {}", self.path);
            }
        }
    }
}

fn audiotar(bigdata: &[f32], smalldata: &[f32], levels: i32) -> Vec<f32> {
    let mut out = Vec::new();
    out.push_all(bigdata);
    out
}

fn main() {
    if os::args().len() != 4 {
        panic!("usage: {} bigfile smallfile outfile", os::args()[0]);
    }
    let mut bigsound = sndfile::File::open(os::args()[1].as_slice(), sndfile::SFM::READ);
    if bigsound.channels() != 1 {
        panic!("bad file, only mono is supported");
    }
    let mut smallsound = sndfile::File::open(os::args()[2].as_slice(), sndfile::SFM::READ);
    if smallsound.channels() != 1 {
        panic!("bad file, only mono is supported");
    }
    let outdata = audiotar(bigsound.read_everything().as_slice(), smallsound.read_everything().as_slice(), 4);
    let mut outsound = sndfile::File::open(os::args()[3].as_slice(), sndfile::SFM::WRITE);
    outsound.write(outdata.as_slice());
}
