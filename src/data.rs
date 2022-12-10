use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

#[derive(Clone)]
pub struct Note {
    path: PathBuf,
    content: String,
}

#[derive(Clone)]
pub struct NoteTree {
    dir_path: PathBuf,
    current_note: Option<Note>,
}

impl NoteTree {
    pub fn new(dir_path: PathBuf) -> Self {
        Self {
            dir_path,
            current_note: None,
        }
    }
    pub fn walk(&self) -> WalkDir {
        WalkDir::new(&self.dir_path)
    }

    pub fn open(&mut self, path: &Path) {
        self.current_note = Note::open(path).ok();
    }
    pub fn render_current(&self) -> Option<String> {
        self.current_note.as_ref().map(|note| &note.content).map(|content| markdown::to_html(content))
    }
}

impl Note {
    pub fn open(path: &Path) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            path: path.into(),
            content: String::from_utf8(fs::read(path)?)?,
        })
    }
}
