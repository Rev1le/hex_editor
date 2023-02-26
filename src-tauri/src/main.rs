#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod stream_file;
mod error;

use wfd::{DialogParams, FOS_PICKFOLDERS};

use tauri::{self, State};
use std::{
  fs,
  io::BufReader,
  path::Path,
  sync::Mutex
};
use stream_file::FileChunks;

// #[derive(Debug)]
// struct HexEditor(Option<OpenFile>);
//
// impl HexEditor {
//   pub fn new() -> Self {
//     HexEditor(None)
//   }
//
//   pub fn open_file(&mut self, path: &str) {
//     self.0 = Some(OpenFile::new(Path::new(path)));
//   }
//
//   pub fn get_open_file(&self) -> Option<&OpenFile> {
//     self.0.as_ref()
//   }
//
//   pub fn get_bytes(&self) -> Option<&Vec<u8>> {
//     return if let Some(open_file) = self.0.as_ref() {
//       Some(open_file.get_bytes())
//     } else {
//       None
//     }
//   }
// }

type HexEditor = Mutex<Option<OpenFile>>;

#[derive(Debug, Clone)]
struct OpenFile {
  pub file_name: String,
  file_extension: String,
  file_path: String,
  file_bytes: Vec<u8>,
  file_str: String
}

impl OpenFile {
  pub fn new(path: &Path) -> Self {
    OpenFile {
      file_name: path.file_name().unwrap().to_str().unwrap().to_owned(),
      file_extension: path.extension().unwrap().to_str().unwrap().to_owned(),
      file_path: path.to_str().unwrap().to_owned(),
      file_bytes: fs::read(path).unwrap(),
      file_str: String::from_utf8_lossy(&fs::read(path).unwrap()).into_owned(),
    }
  }

  pub fn filename(&self) -> &String {
    &self.file_name
  }

  pub fn file_path(&self) -> &String {
    &self.file_path
  }

  pub fn get_bytes(&self) -> &Vec<u8> {
    &self.file_bytes
  }

  pub fn content_as_str(&self) -> String{
    String::from_utf8_lossy(self.get_bytes()).into_owned()
  }

}

struct Chunk([u8;16]);

impl TryFrom<&[u8]> for Chunk {
  type Error = std::array::TryFromSliceError;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    Ok(Chunk(<[u8;16]>::try_from(&value[..16])?))
  }
}

struct Frame(Vec<Chunk>);

struct StreamOpenFile{
  open_file: OpenFile,
  frames: Vec<Frame>,
  current_frame: usize
}

impl StreamOpenFile {
  pub fn new(open_file: OpenFile) -> Self {


    StreamOpenFile {
      open_file,
      frames: Vec::new(),
      current_frame: 0,
    }
  }
}

#[tauri::command]
fn open_file(editor: State<HexEditor>) {

  let result = wfd::open_dialog(Default::default()).unwrap().selected_file_path;
  println!("{:?}", result);
  *editor.lock().unwrap() = Some(OpenFile::new(&result));
  println!("Открыт файл: {:?}", editor.lock().unwrap().as_ref().unwrap().file_path());
}

#[tauri::command]
fn get_file_bytes(editor: State<HexEditor>) -> Vec<u8> {

  return if let Some(open_file) = editor.lock().unwrap().as_ref() {
    open_file.get_bytes().clone()
  } else {
    vec![]
  }

}

#[tauri::command]
fn get_file_str(editor: State<HexEditor>) -> String{

  return if let Some(open_file) = editor.lock().unwrap().as_ref() {
    open_file.content_as_str()
  } else {
    String::new()
  }

}


#[tauri::command]
fn get_bytes_chunks(editor: State<HexEditor>) -> Vec<Vec<u8>> {

  return if let Some(open_file) = editor.lock().unwrap().as_ref() {
    open_file
        .get_bytes()
        .chunks(16)
        .map(|chunk| chunk.to_vec())
        .collect()
  } else {
    vec![]
  };
}



fn main() {

  // let mut file_chunks = FileChunks::new("C:\\Users\\nikiy\\Documents\\FPSMonitor.txt", 16).unwrap();
  // println!("{:?}", file_chunks.get_chunk_by_pos(0).unwrap());
  // println!("{:?}", file_chunks.get_chunk_by_pos(10).unwrap());
  //
  // println!("{:?}", file_chunks.stream_position());
  //
  // println!("{:?}", file_chunks.get_chunk_by_pos(0).unwrap());
  //
  // println!("{:?}", file_chunks.next_chunk().unwrap());
  // println!("{:?}", file_chunks.next_chunk().unwrap());
  //
  // println!("{:?}", file_chunks.stream_position().unwrap());
  //
  // println!("{:?}", file_chunks.pred_chunk().unwrap());
  //
  // println!("{:?}", file_chunks.stream_position());

  tauri::Builder::default()
      .manage(Mutex::new(None::<OpenFile>))
      .invoke_handler(tauri::generate_handler![open_file, get_file_bytes, get_file_str, get_bytes_chunks])
      .any_thread()
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
