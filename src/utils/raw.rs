use std::ops::Add;
use std::path::{PathBuf, Path};
use std::{io, fs};
use std::process::Command;

use anyhow::Ok;

#[path = "./dict.rs"] mod dict;

use dict::Dictionary;

fn path_buf_to_str(path_buf: &PathBuf) -> String {
  path_buf.display().to_string()
}

fn convert(barename: &PathBuf) -> anyhow::Result<()> {
  Ok(())
}

pub fn from_ifo(ifo_path: &str, dest: &str) -> anyhow::Result<()> {
  let file_id = Path::new(ifo_path).file_stem();
  if let Some(file_name) = file_id {
    let barename = Path::new(dest).join(file_name).to_owned();
    println!("barename: {:?}", barename);
    
    // Unarchived dict.dz
    // Command::new("gunzip")
    //   .args(["-f", "-S", ".dz", path_buf_to_str(&barename).add(".dict.dz").as_str()])
    //   .output()?;

    let mut dic = Dictionary::new(
      path_buf_to_str(&barename).add(".ifo"), 
      path_buf_to_str(&barename).add(".idx"), 
      path_buf_to_str(&barename).add(".dict")
    );

    dic.load_info()?;
    dic.load_idx()?;
    dic.load_dict()?;

  }
  Ok(())
}

// Get files from .tar.bz2
pub fn from_archive(finename: &str, dest: &str) -> anyhow::Result<()> {
  println!("fromArchive");
  Command::new("tar")
    .args(["-xjf", finename, "-C", dest])
    .output()?;
  
  let entries = fs::read_dir(dest)?.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>()?;
  let archive_dir = &entries[0];

  let file_contents = fs::read_dir(archive_dir)?
                      .map(|res| res.map(|e| e.path()))
                      .collect::<Result<Vec<_>, io::Error>>()?;

  let file_contents: Vec<_> = file_contents.iter().filter(|f| {
    let filename = f.display().to_string();
    let filename = Path::new(&filename);
    let mut has_ifo = false;
    if let Some(ext) = filename.extension() {
      if ext.eq("ifo") {
        has_ifo = true
      }
    }
    has_ifo
  }).collect();

  from_ifo(path_buf_to_str(file_contents[0]).as_str(), path_buf_to_str(archive_dir).as_str())?;
  
  Ok(())
}