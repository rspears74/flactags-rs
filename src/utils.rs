use std::fs::{DirEntry, read_dir};
use std::path::PathBuf;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::io::{self, Write};
use std::error;

pub struct TagEditor {
  tags: Vec<Tag>
}

impl TagEditor {
  pub fn new(dir: &PathBuf) -> Result<TagEditor, Box<dyn std::error::Error>> {
    let mut flacs = Self::get_flacs(dir)?;
    flacs.sort_by_key(|f| f.path());
    Ok(TagEditor {
      tags: flacs.into_iter()
        .map(|f| Self::get_tag(f).expect("Failed to get tag"))
        .collect::<Vec<Tag>>()
    })
  }

  fn get_tag<'a>(file: DirEntry) -> metaflac::Result<Tag> {
    let meta_tag = metaflac::Tag::read_from_path(file.path())?;
    Ok(Tag {
      meta_tag,
      file_info: file
    })
  }

  fn get_flacs(dir: &PathBuf) -> io::Result<Vec<DirEntry>> {
    let all_files: std::fs::ReadDir = read_dir(dir)?;
    all_files.filter(
        |path| match path.as_ref().ok()
            .and_then(|p| p.path().extension().map(|e| e.to_owned()))
            .map(|ext| ext == "flac") {
                Some(result) => result,
                None => false
            }
        ).collect()
  }
  
  pub fn set_tags(&mut self, key: &str, value: &str) {
    self.tags
      .iter_mut()
      .for_each(|tag| tag.set(key, value))
  }

  pub fn print_tags(&self) {
    self.tags
      .iter()
      .for_each(|tag| println!("{}", tag))
  }

  pub fn save(&mut self) -> metaflac::Result<()> {
    self.tags
      .iter_mut()
      .map(|tag| tag.save())
      .collect()
  }
}


pub struct InteractiveTagEditor<'a> {
  editor: &'a mut TagEditor
}

impl<'a> InteractiveTagEditor<'a> {
  pub fn new(tag_editor: &'a mut TagEditor) -> Self {
    InteractiveTagEditor { editor: tag_editor }
  }

  pub fn edit_tags(&mut self, key: &str) -> Result<(), Box<dyn error::Error>> {
    for tag in self.editor.tags.iter_mut() {
      tag.change_interactive(key)?;
    }
    Ok(())
  }
}


pub struct Tag {
  pub meta_tag: metaflac::Tag,
  pub file_info: DirEntry
}

impl Tag {
  pub fn set(&mut self, key: &str, value: &str) {
    self.meta_tag.set_vorbis(key, vec![value])
  }

  pub fn save(&mut self) -> metaflac::Result<()> {
    self.meta_tag.save()
  }

  pub fn write_key(&self, key: &str, f: &mut Formatter) -> fmt::Result {
    if let Some(items) = self.meta_tag.get_vorbis(key) {
      write!(f, "{}: ", key)?;
      write!(f, "{}", items.collect::<Vec<_>>().join("; "))?;
      write!(f, "\n")
    } else {
      write!(f, "No items found for key {}\n", key)
    }
  }

  pub fn print_key(&self, key: &str) {
    if let Some(items) = self.meta_tag.get_vorbis(key) {
      print!("{}: ", key);
      print!("{}", items.collect::<Vec<_>>().join("; "));
      print!("\n");
    } else {
      print!("No items found for key {}\n", key);
    }
  }

  fn change_interactive(&mut self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", self);
    print!("Current ");
    self.print_key(key);
    print!("Enter new {} or press ENTER to keep existing: ", key);
    io::stdout().flush()?;
    let mut new_value = String::new();
    io::stdin().read_line(&mut new_value)?;
    new_value.pop();
    if new_value != "" {
      self.set(key, &new_value);
    }
    Ok(())
  }

}

impl Display for Tag {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let keys = self.meta_tag.vorbis_comments().unwrap().comments.keys();
    write!(f, "file: {:?}\n", self.file_info.file_name())?;
    keys
      .map(|s| self.write_key(s, f))
      .collect::<Result<(), _>>()
  }
}
