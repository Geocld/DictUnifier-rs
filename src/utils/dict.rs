use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use byteorder::{BigEndian, ByteOrder};
use lazy_regex::*;

use anyhow::{Result, Ok};

#[path = "./html.rs"] mod html;

use html::{html2text};

#[derive(Debug, Default, Clone, PartialEq)]
struct Idx {
  id: i32,
  index: String,
  offset: u32,
  size: u32
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Info {
  version: String,
  wordcount: i32,
  idxfilesize: u64,
  bookname: String,
  sametypesequence: String
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Dict {
  id: i32,
  index: String,
  xml: String
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Dictionary {
  ifo_file: String,
  idx_file: String,
  dict_file: String,
  idx: Vec<Idx>,
  info: Info,
  data: Vec<Dict>
}

impl Idx {
  fn new(id: i32, index: String, offset: u32, size: u32) -> Self {
    Idx { id, index, offset, size }
  }
}

impl Dict {
  fn new(id: i32, index: String, xml: String) -> Self {
    Dict { id, index, xml }
  }
}

fn cover_dict_data_to_xml(text: String, t: char) {
  match t {
    'h' => { // TODO: HTML
      // let txt = html2text(&text);
      // let txt_arr: Vec<&str> = txt.split("\n").collect();
      // let result = txt_arr.into_iter().map(|t| {
      // });
    }
    'g' => {

    }
    'x' => {}
    _ => {}
  }
}

fn parse_dict_data(id: i32, buffer: Vec<u8>, types: &str) -> Result<()> {
  let tps: Vec<char> = types.to_string().clone().chars().collect();
  if tps.len() == 0 {
     // TODO
  } else {
    let mut pos = 0;
    let mut res: Vec<Dict> = Vec::new();

    // TODO: use map
    for i in 0..tps.len() {
      while pos < buffer.len() {
        let tp = tps.get(i);
        let mut l = 0;

        if let Some(t) = tp {
          match t {
            'W'|'P'|'X' => { // Media file
              if i < tps.len() - 1 {
                l = BigEndian::read_u16(&buffer[pos - 2..pos]) as usize;
                pos = pos + l + 4;
              } else {
                pos = buffer.len();
              }
              res.push(Dict::new(id, String::from(""), String::from("<p class=\"error\"> DictUnifier: Media file is not supported. </p>")));
            },
            'r' => { // Resource file
              while pos + l < buffer.len() && buffer[pos + l] != 0 {
                l = l + 1;
              }
              res.push(Dict::new(id, String::from(""), String::from("<p class=\"error\"> DictUnifier: Resource file is not supported. </p>")));
            },
            _ => {
              while pos + l < buffer.len() && buffer[pos + l] != 0 {
                l = l + 1;
              }
              let text_buf = buffer[pos..pos + l].to_owned();
              let text = String::from_utf8(text_buf)?;
              println!("text: {}", text);
            }
          }
        }

        let next = pos + l + 1;
  
        pos = next;
      }
    }

    // println!("res: {:?}", res);
  }

  Ok(())
}

impl Dictionary {

  pub fn new(ifo_file: String, idx_file: String, dict_file: String) -> Self {
    Dictionary { 
      ifo_file, 
      idx_file, 
      dict_file, 
      idx: Vec::new(), 
      info: Info{ ..Default::default() },
      data: Vec::new()
    }
  }

  pub fn load_info(&mut self) -> Result<()> {
    let input = File::open(&self.ifo_file)?;
    let buffered = BufReader::new(input);
    let mut info = Info{ ..Default::default() };

    for line in buffered.lines() {
      let text = line?;
      let match_words = regex_captures!(r#"^(.+)=(.+)$"#, &text);
      if let Some(captures) = match_words {
        let (key, value) = (
          captures.1.to_string(),
          captures.2.to_string(),
        );
        // TODO: how to generate dynamic struct
        if key.eq("version") {
          info.version = value.clone();
        }

        if key.eq("wordcount") {
          let v = value.clone();
          info.wordcount = v.parse::<i32>()?;
        }

        if key.eq("idxfilesize") {
          let v = value.clone();
          info.idxfilesize = v.parse::<u64>()?;
        }

        if key.eq("bookname") {
          info.bookname = value.clone();
        }

        if key.eq("sametypesequence") {
          info.sametypesequence = value.clone();
        }
      }
    }

    self.info = info;

    Ok(())
  }

  pub fn load_idx(&mut self) -> Result<()> {
    let f = File::open(&self.idx_file)?;

    let mut stream = BufReader::new(f);
    let mut buffer: Vec<u8> = vec![];

    // Get the full stream buffer of idx file.
    stream.read_to_end(&mut buffer)?;

    // println!("buffer: {:?}", buffer);

    let mut pos = 0;
    let mut offset: u32 = 0;
    for id in 0..self.info.wordcount {
      let mut l = 1;
      while buffer[pos + l] != 0 && pos + l < buffer.len() {
        l = l + 1;
      }
      let next = pos + l + 9;
      let index_buf = buffer[pos..pos + l].to_owned();
      let index = String::from_utf8(index_buf)?;
      let size = BigEndian::read_u16(&buffer[next - 2..next]);

      // println!("index: {}", index);
      // println!("size: {}", size);
      // println!("offset: {}", offset);

      self.idx.push(Idx::new(id, index, offset as u32, size as u32));

      pos = next; // move to next word
      offset = offset + size as u32; // record offset
    }

    // println!("idx: {:?}", self.idx);

    Ok(())
  }

  pub fn load_dict(&mut self) -> Result<()> {
    println!("Total length of entries: {}", self.idx.len());

    let f = File::open(&self.dict_file)?;

    let mut stream = BufReader::new(f);
    let mut buffer: Vec<u8> = vec![];

    // Get the full stream buffer of idx file.
    stream.read_to_end(&mut buffer)?;

    // println!("dict file buffer: {:?}", buffer);
    for idx in 0..2 {
      // idx -> dict
      let idx_data = self.idx.get(idx);
      println!("idx_data: {:?}", idx_data);

      if let Some(data) = idx_data {
        let offset = data.offset as usize;
        let size = data.size as usize;
        let buf = buffer[offset..offset + size].to_owned();

        parse_dict_data(idx as i32, buf, &self.info.sametypesequence)?;
        
      }
      
    }


    Ok(())
  }

}