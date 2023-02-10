mod utils;

use utils::{ from_archive };

fn main() -> anyhow::Result<()> {
    from_archive("./__test__/stardict-oald-cn-2.4.2.tar.bz2", "./__test2__")?;
    Ok(())
}
