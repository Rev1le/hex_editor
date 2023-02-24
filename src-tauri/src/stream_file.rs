use std::fs::{self, File};
use std::io::{self, BufReader, Error, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use super::error::HexEditorError;

#[derive(Debug)]
pub struct MetadataOpenFile {
    pub file_name: String,
    pub file_extension: String,
    pub absolute_file_path: PathBuf,
    pub file_len: usize,
    pub file_metadata: fs::Metadata
}

#[derive(Debug)]
pub struct StreamOpenFile {
    stream_file: BufReader<File>,
    current_position: usize,
    metadata: MetadataOpenFile
}

impl StreamOpenFile {
    pub fn new(path: &str) -> Result<Self, HexEditorError> {
        let path = Path::new(path);
        let file_name = path
            .file_name()
            .ok_or(HexEditorError::PathError)?
            .to_os_string()
            .into_string()?;
        let file_extension = path
            .extension()
            .ok_or(HexEditorError::PathError)?
            .to_os_string()
            .into_string()?;

        let absolute_file_path = PathBuf::from(path);
        let f = File::open(path)?;
        let file_len = f.metadata()?.len() as usize;
        let file_metadata = f.metadata()?;


        Ok( StreamOpenFile {
            stream_file: BufReader::new(f),
            current_position: 0_usize,
            metadata: MetadataOpenFile {
                file_name,
                file_extension,
                absolute_file_path,
                file_len,
                file_metadata,
            },
        })
    }

    pub fn buffer(&self) -> &[u8] {
        self.stream_file.buffer()
    }

    pub fn get_metadata(&self) -> &MetadataOpenFile {
        &self.metadata
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, HexEditorError> {
        Ok(self.stream_file.read(buf)?)
    }

    fn seek(&mut self, pos: SeekFrom) -> Result<u64, HexEditorError> {
        Ok(self.stream_file.seek(pos)?)
    }
}

#[derive(Debug)]
pub struct Chunk(pub Vec<u8>);

#[derive(Debug)]
pub struct FileChunks {
    chunk_size: usize,
    count_chunks: usize,
    current_chunk: Chunk,
    file: StreamOpenFile,
    current_pos: usize,
}

impl FileChunks {
    pub fn new(file_path: &str, chunk_size: usize) -> Result<Self, HexEditorError> {

        let mut stream_open_file = StreamOpenFile::new(file_path)?;
        let mut chunk: Vec<u8> = Vec::with_capacity(chunk_size);
        unsafe {
            chunk.set_len(chunk_size);
        }
        let count_chunks = stream_open_file.get_metadata().file_len/chunk_size;

        Ok(FileChunks {
            chunk_size,
            count_chunks,
            current_chunk: Chunk(chunk),
            file: stream_open_file,
            current_pos: 0
        })
    }

    pub fn get_file_metadata(&self) -> &MetadataOpenFile {
        self.file.get_metadata()
    }

    pub fn get_chunk_by_pos(&mut self, pos: usize) -> Result<&Chunk, HexEditorError> {

        let new_chunk_pos = self.chunk_size * pos;

        self.file.stream_file.seek(
            SeekFrom::Start(new_chunk_pos as u64)
        )?;
        self.file.read(&mut self.current_chunk.0)?;

        self.current_pos = new_chunk_pos + 16;
        Ok(&self.current_chunk)
    }

    pub fn get_chunk_pos(&self) -> usize {
        self.current_pos/16
    }

    pub fn next_chunk(&mut self) -> Result<&Chunk, HexEditorError> {
        self.file.read(&mut self.current_chunk.0)?;
        self.current_pos += 16;
        Ok(&self.current_chunk)
    }

    pub fn pred_chunk(&mut self) -> Result<&Chunk, HexEditorError> {
        let pred_chunk_pos = self.current_pos - 32;

        self.file.seek(SeekFrom::Start(pred_chunk_pos as u64))?;
        self.file.read(&mut self.current_chunk.0)?;

        self.current_pos -= 16;
        Ok(&self.current_chunk)
    }

    pub fn stream_position(&mut self) -> Result<u64, HexEditorError> {
        Ok(self.file.stream_file.stream_position()?)
    }
}