use std::ops::Add;
use std::path::{PathBuf, Path};
use std::{io, fs};
use std::process::Command;
use std::collections::HashMap;
use anyhow::Ok;
use fs_extra::file::{CopyOptions, copy};

#[path = "./dict.rs"] 
mod dict;
#[path = "./html.rs"] 
mod html;

use html::{ clean_xml };
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
    Command::new("gunzip")
      .args(["-f", "-S", ".dz", path_buf_to_str(&barename).add(".dict.dz").as_str()])
      .output()?;

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

    fs::write(&dictionary_xml_path, format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<d:dictionary xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:d=\"http://www.apple.com/DTDs/DictionaryService-1.0.rng\">
{}
</d:dictionary>", str_entries.join("\n")))?;

    let mut cp_options = CopyOptions::new();
    cp_options.overwrite = true;

    let css_template_path = Path::new("assets").join("templates").join("Dictionary.css");
    copy(css_template_path, &dictionary_css_path, &cp_options)?;

    let plist_template_path = Path::new("assets").join("templates").join("DictInfo.plist");
    let plist_contents = fs::read_to_string(plist_template_path)?;
    let plist = plist_contents
                .replace("{{dictName}}", dic.info.bookname.as_str())
                .replace("{{dictID}}", file_name.to_str().unwrap_or(""));
    fs::write(&dictionary_plist_path, plist)?;

    // build dictionary
    let build_dict_shell_path = Path::new("assets").join("bin").join("build_dict.sh");
    let xml = dictionary_xml_path.clone();
    let css = dictionary_css_path.clone();
    let plist = dictionary_plist_path.clone();
    let output = Path::new(dest).join("output");
    let output_str  = path_buf_to_str(&output);
    let envs = HashMap::from([
      ("DICT_DEV_KIT_OBJ_DIR", output_str.as_str()),
      ("LANG", "en_US.UTF-8")
    ]);

    let dic_name = file_name.to_str().unwrap_or("default");
    let mut build_progress = Command::new(build_dict_shell_path)
                                    .arg(dic_name)
                                    .arg(&(path_buf_to_str(&xml)))
                                    .arg(&(path_buf_to_str(&css)))
                                    .arg(&(path_buf_to_str(&plist)))
                                    .envs(envs)
                                    .spawn()
                                    .expect("Fail to build dictionary");
    build_progress.wait()?;

    let output_dictionary = output.join(String::from(dic_name) + ".dictionary");

    match home::home_dir() {
      Some(home_path) => {
        let library_dictionaries = home_path.join("Library").join("Dictionaries");
        let options = fs_extra::dir::CopyOptions::new(); 
        fs_extra::dir::copy(output_dictionary, library_dictionaries, &options)?;
        println!("Installed to ~/Library/Dictionaries/");
        fs_extra::dir::remove(dest)?;
      },
      None => println!("Impossible to get your home dir!"),
    }
  }


  Ok(())
}

// Get files from .tar.bz2
pub fn from_archive(file: &str, dest: &str) -> anyhow::Result<()> {
  Command::new("tar")
    .args(["-xjf", file, "-C", dest])
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