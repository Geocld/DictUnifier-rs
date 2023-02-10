use std::fs::File;
use std::io::{BufReader, Read};
use byteorder::{BigEndian, ByteOrder};

use anyhow::Ok;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Dictionary {
  ifo_file: String,
  idx_file: String,
  dict_file: String,
  idx: Vec<Idx>
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Idx {
  id: i32,
  index: String,
  offset: u32,
  size: u32
}

impl Idx {
  fn new(id: i32, index: String, offset: u32, size: u32) -> Self {
    Idx { id, index, offset, size }
  }
}

impl Dictionary {

  pub fn new(ifo_file: String, idx_file: String, dict_file: String) -> Self {
    Dictionary { ifo_file, idx_file, dict_file, idx: Vec::new() }
  }

  pub fn load_idx(&mut self) -> anyhow::Result<()> {
    println!("idx_file: {}", self.idx_file);
    let f = File::open(&self.idx_file)?;

    let mut stream = BufReader::new(f);
    let mut buffer: Vec<u8> = vec![];

    // Get the full stream buffer of idx file.
    stream.read_to_end(&mut buffer)?;

    // println!("buffer: {:?}", buffer);

    let mut pos = 0;
    let mut offset: u32 = 0;
    for id in 0..39433 {
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

}