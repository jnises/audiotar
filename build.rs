extern crate pkg_config;

fn main() {
    // so we can find libsndfile
    pkg_config::find_library("sndfile").unwrap();
}
