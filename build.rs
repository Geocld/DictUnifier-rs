use std::env;
use std::path::Path;
use fs_extra::dir::{CopyOptions, copy};

macro_rules! console {
  ($($tokens: tt)*) => {
      println!("cargo:warning={}", format!($($tokens)*))
  }
}

const ASSETS_DIR: &str = "assets";

fn main() {
  let out_dir = env::var_os("OUT_DIR").unwrap_or_default();
  if out_dir.len() > 0 {
    let assets_path = Path::new(ASSETS_DIR);
    let target_path = Path::new(&out_dir).join("..").join("..").join("..");
    console!("Copying assets to {target_path:?}");

    let mut cp_options = CopyOptions::new();
    cp_options.overwrite = true;
    copy(assets_path, target_path, &cp_options).expect("Copy assets error.");
  }
}