use std::env;
use lazy_static::lazy_static;
use fs_extra::dir;

mod utils;
use utils::{ from_archive };


fn main() -> anyhow::Result<()> {
    lazy_static! {
        static ref DEST: String = String::from("./dist");
    }
    dir::create_all("./dist/output", true)?;
    let args: Vec<String> = env::args().collect();
    if let Some(file) = args.get(1) {
        // test file: "./__test__/stardict-oald-cn-2.4.2.tar.bz2"
        from_archive(file, &DEST)?;
    }
    
    Ok(())
}
