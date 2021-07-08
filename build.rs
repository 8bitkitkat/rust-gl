extern crate gl_generator;

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::fs::File;
use std::path::Path;

#[cfg(feature = "core")]
const PROFILE: Profile = Profile::Core;
#[cfg(not(feature = "core"))]
const PROFILE: Profile = Profile::Compatibility;

#[cfg(feature = "fallbacks")]
const FALLBACKS: Fallbacks = Fallbacks::ALl;
#[cfg(not(feature = "fallbacks"))]
const FALLBACKS: Fallbacks = Fallbacks::None;

#[cfg(all(feature = "gl_3_3", feature = "gl_4_5"))]
compile_error!("Please only select one version feature flag");
#[cfg(feature = "gl_3_3")]
const VERSION: (u8, u8) = (3, 3);
#[cfg(feature = "gl_4_5")]
const VERSION: (u8, u8) = (4, 5);

const API: Api = Api::Gl;

fn main() {
    println!("cargo:rerun-if-env-changed=REGEN_GL");

    let src_path = Path::new("./src");
    if !src_path.exists() {
        std::fs::create_dir(src_path).unwrap();
    }
    let mut out_file = File::create("./src/lib.rs").unwrap();

    Registry::new(API, VERSION, PROFILE, FALLBACKS, [])
        .write_bindings(GlobalGenerator, &mut out_file)
        .unwrap();
}
