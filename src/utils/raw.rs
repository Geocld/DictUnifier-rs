use std::ops::Add;
use std::path::{PathBuf, Path};
use std::{io, fs};
use std::process::Command;

use anyhow::Ok;

#[path = "./dict.rs"] 
mod dict;
#[path = "./html.rs"] 
mod html;

#[path = "./template.rs"] 
mod template;

use html::{ clean_xml };
use template::{generate_plist, generate_css};

use dict::Dictionary;

fn path_buf_to_str(path_buf: &PathBuf) -> String {
  path_buf.display().to_string()
}

pub fn from_ifo(ifo_path: &str, archive_path: &str, dest: &str) -> anyhow::Result<()> {
  let file_id = Path::new(ifo_path).file_stem();
  if let Some(file_name) = file_id {
    let barename = Path::new(archive_path).join(file_name).to_owned();
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

    let output_path = Path::new(dest).join(file_name);
    let is_exists = output_path.try_exists()?;
    if !is_exists {
      fs::create_dir(output_path.clone())?;
    }
    
    let str_entries: Vec<_> = dic.data.iter().map(|d| {
      let clean_index = clean_xml(&*d.index);
      format!("<d:entry id=\"{}\" d:title=\"{}\">
<d:index d:value=\"{}\"/>
<h1>{}</h1>
<div>
{}
</div>
</d:entry>", d.id, clean_index, clean_index, clean_index, d.xml)
    }).collect();
    
    let dictionary_xml_path = Path::new(&output_path).join("Dictionary.xml");
    let dictionary_plist_path = Path::new(&output_path).join("DictInfo.plist");
    let dictionary_css_path = Path::new(&output_path).join("Dictionary.css");

    fs::write(dictionary_xml_path, format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<d:dictionary xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:d=\"http://www.apple.com/DTDs/DictionaryService-1.0.rng\">
{}
</d:dictionary>", str_entries.join("\n")))?;

    let plist_contents = generate_plist(dic.info.bookname.as_str(), file_name.to_str().unwrap_or(""));
    fs::write(dictionary_plist_path, plist_contents)?;

    let css_contents = generate_css();
    fs::write(dictionary_css_path, css_contents)?;
  }


  Ok(())
}

// Get files from .tar.bz2
pub fn from_archive(finename: &str, dest: &str) -> anyhow::Result<()> {
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

  from_ifo(
    path_buf_to_str(file_contents[0]).as_str(), 
    path_buf_to_str(archive_dir).as_str(),
    dest
  )?;
  
  Ok(())
}