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

type HexEditor = Mutex<Option<FileChunks>>;

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

#[tauri::command(rename_all = "snake_case")]
fn open_file(editor: State<HexEditor>, chunk_size: usize) -> bool {

  if let Ok(open_dialog) = wfd::open_dialog(Default::default()) {
    let path = open_dialog.selected_file_path;
    *editor.lock().unwrap() = Some( FileChunks::new(path.clone(), chunk_size).unwrap());

    println!("Открыт файл по пути: {:?}", path);

    return true
  }

  false
}

#[tauri::command]
fn get_chunk_by_pos(editor: State<HexEditor>, pos: usize) -> Vec<u8> {
	if let Some(editor) = editor.lock().unwrap().as_mut() {
		return editor.get_chunk_by_pos(pos).unwrap().0.clone()
	}
	println!("Файл ещё не открыт");
	vec![]
}

#[tauri::command]
fn get_chunk_pos(editor: State<HexEditor>) -> usize {
	if let Some(editor) = editor.lock().unwrap().as_mut() {
		return editor.get_chunk_pos()
	}
	println!("Файл ещё не открыт");
	0
}

#[tauri::command]
fn next_chunk(editor: State<HexEditor>) -> Vec<u8> {
	if let Some(editor) = editor.lock().unwrap().as_mut() {
		return editor.next_chunk().unwrap().0.clone()
	}
	println!("Файл ещё не открыт");
	vec![]
}

//size
#[tauri::command]
fn next_chunk_split(editor: State<HexEditor>) -> Vec<Vec<u8>> {
    if let Some(editor) = editor.lock().unwrap().as_mut() {
        let next_chunk = editor.next_chunk().unwrap().0.clone();
        return next_chunk.chunks(16).map(|chunk| chunk.to_vec()).collect::<Vec<Vec<u8>>>()
    }
    println!("Файл ещё не открыт");
    vec![]
}

#[tauri::command]
fn prev_chunk(editor: State<HexEditor>) -> Vec<u8> {
	if let Some(editor) = editor.lock().unwrap().as_mut() {
		return editor.prev_chunk().unwrap().0.clone()
	}
	println!("Файл ещё не открыт");
	vec![]
}

fn main() {

    // let mut file_chunks = FileChunks::new("C:\\Users\\nikiy\\Documents\\FPSMonitor.txt".into(), 4).unwrap();
    // println!("Чанк по позиции 0: {:?}", file_chunks.get_chunk_by_pos(0).unwrap());
    // println!("Чанк по позиции 10: {:?}", file_chunks.get_chunk_by_pos(10).unwrap());
    //
    // println!("Позиция стрима: {:?}", file_chunks.stream_position());
    //
    // println!("Чанк по позиции 0 {:?}", file_chunks.get_chunk_by_pos(0).unwrap());
    //
    // println!("Следующий чанк: {:?}", file_chunks.next_chunk().unwrap());
    // println!("Следующий чанк: {:?}", file_chunks.next_chunk().unwrap());
    //
    // println!("Позиция стрима: {:?}", file_chunks.stream_position().unwrap());
    //
    // println!("Предыдущий чанк: {:?}", file_chunks.prev_chunk().unwrap());
    //
    // println!("Позиция стрима: {:?}", file_chunks.stream_position());

    tauri::Builder::default()
        .manage(Mutex::new(None::<FileChunks>))
        .invoke_handler(tauri::generate_handler![open_file, get_chunk_by_pos, get_chunk_pos, next_chunk, prev_chunk, next_chunk_split])
        .any_thread()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
