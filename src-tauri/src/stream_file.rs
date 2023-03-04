use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Error, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::ops::Neg;
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
    pub fn new(path: PathBuf, stream_file_capacity: Option<usize>) -> Result<Self, HexEditorError> {

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

        let f = File::open(&path)?;
        let file_len = f.metadata()?.len() as usize;
        let file_metadata = f.metadata()?;

        let stream_file_capacity = stream_file_capacity.unwrap_or(8_000);
        let stream_file = BufReader::with_capacity(stream_file_capacity,f);

        Ok( StreamOpenFile {
            stream_file,
            current_position: 0_usize,
            metadata: MetadataOpenFile {
                file_name,
                file_extension,
                absolute_file_path: PathBuf::from(&path),
                file_len,
                file_metadata,
            },
        })
    }

    pub fn buffer(&self) -> &[u8] {
        self.stream_file.buffer()
    }

    pub fn fill_buf(&mut self) -> Result<&[u8], HexEditorError> {
        Ok(self.stream_file.fill_buf()?)
    }

    pub fn consume(&mut self, amt: usize) {
        self.stream_file.consume(amt)
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

#[derive(Debug, Clone)]
pub struct Chunk(pub Vec<u8>);

impl Chunk {
    pub fn new(chunk_size: usize) -> Self {

        let mut chunk: Vec<u8> = Vec::with_capacity(chunk_size);
        unsafe { chunk.set_len(chunk_size); }

        Chunk(chunk)
    }

    pub fn get_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

impl From<Vec<u8>> for Chunk {
    fn from(value: Vec<u8>) -> Self {
        Chunk(value)
    }
}

#[derive(Debug)]
pub struct FileChunks {
    chunk_size: usize,
    count_chunks: usize,
    current_chunk: Chunk,
    stream_file: StreamOpenFile,
    current_pos: usize,
    buffer_chunks: VecDeque<Chunk>,
}

impl FileChunks {
    pub fn new(file_path: PathBuf, chunk_size: usize) -> Result<Self, HexEditorError> {

        let stream_file_capacity = Some(chunk_size * 5);

        let mut stream_open_file = StreamOpenFile::new(
            file_path,
            stream_file_capacity
        )?;

        let count_chunks = stream_open_file.get_metadata().file_len / chunk_size;

        let mut buffer_chunks = VecDeque::<Chunk>::from_iter(
            stream_open_file
                .fill_buf()?
                .chunks(chunk_size)
                //.inspect(|chunk| println!("Chunk: {chunk:?}"))
                .map(|chunk| Chunk::from(chunk.to_vec()))
        );

        Ok( FileChunks {
            chunk_size,
            count_chunks,
            current_chunk: Chunk::new(chunk_size),
            stream_file: stream_open_file,
            current_pos: 0,
            buffer_chunks,
        } )
    }

    pub fn get_chunk_from_buffer(&self, ind: usize) -> Result<Chunk, HexEditorError> {
        self.stream_file
            .buffer()
            .chunks(self.chunk_size)
            .nth(ind)
            .map(|chunk_bytes|Chunk::from(chunk_bytes.to_vec()))
            .ok_or(HexEditorError::NoneBufferChunk)
    }

    pub fn get_chunk_by_pos(&mut self, pos: usize) -> Result<&Chunk, HexEditorError> {
        println!("Old buffer {:?}", self.stream_file.fill_buf());

        let new_chunk_pos = self.chunk_size * pos;

        let seek_pos = SeekFrom::Start(new_chunk_pos as u64);
        self.stream_file.seek(seek_pos)?;
        println!("New buffer {:?}", self.stream_file.fill_buf());
        
        self.stream_file.read(self.current_chunk.get_mut())?;

        self.current_pos = new_chunk_pos + self.chunk_size;

        Ok(&self.current_chunk)
    }

    pub fn get_chunk_pos(&self) -> usize {
        self.current_pos/self.chunk_size
    }
	
	pub fn current_chunk(&mut self) -> &Chunk {
        &self.current_chunk
    }

    pub fn peek_next_chunk(&self) -> Result<Chunk, HexEditorError> {
        //self.buffer_chunks.get(3).ok_or(HexEditorError::NoneBufferChunk)
        Ok(Chunk::from(
            self.stream_file
                .buffer()
                .chunks(self.chunk_size)
                .nth(1)
                .unwrap()
                .to_vec()
        ) )
    }

    pub fn peek_prev_chunk(&self) -> Result<&Chunk, HexEditorError> {
        self.buffer_chunks.get(1).ok_or(HexEditorError::NoneBufferChunk)
    }

    pub fn next_chunk(&mut self) -> Result<&Chunk, HexEditorError> {
        self.stream_file.read(self.current_chunk.get_mut())?;
        self.current_pos += self.chunk_size;
        Ok(&self.current_chunk)
    }

    pub fn prev_chunk(&mut self) -> Result<&Chunk, HexEditorError> {

        // If seek file cursor stay in 0 bytes
        if self.current_pos == 0 {
            return Err(HexEditorError::StayFirstChunk)
        }

        let delta_cur_prev_pos= Neg::neg(self.chunk_size as i64 * 2);

        let prev_chunk_seek = dbg!(SeekFrom::Current(delta_cur_prev_pos));

        self.stream_file.seek(prev_chunk_seek)?;
        self.stream_file.read(&mut self.current_chunk.0)?;

        self.current_pos -= self.chunk_size;

        Ok(&self.current_chunk)
    }

    pub fn stream_position(&mut self) -> Result<u64, HexEditorError> {
        Ok(self.stream_file.stream_file.stream_position()?)
    }
}