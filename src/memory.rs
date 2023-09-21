use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
};

pub struct ProcessMemory {
    file: File,
}

impl ProcessMemory {
    pub fn open(pid: u32) -> io::Result<ProcessMemory> {
        let path = format!("/proc/{}/mem", pid);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(path)?;

        Ok(ProcessMemory { file })
    }

    pub fn read<T: ByteConversion>(&mut self, offset: u64) -> io::Result<T> {
        self.file.seek(SeekFrom::Start(offset))?;
        let size = std::mem::size_of::<T>();
        let mut buf = vec![0; size];
        self.file.read_exact(&mut buf)?;

        match T::from_bytes(&buf) {
            Some(value) => Ok(value),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid data at the specified offset",
            )),
        }
    }

    pub fn write<T: ByteConversion>(&mut self, offset: u64, value: T) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(offset))?;
        let buf = value.to_bytes();
        self.file.write_all(&buf)?;

        Ok(())
    }
}

pub trait ByteConversion: Sized {
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
    fn to_bytes(&self) -> Vec<u8>;
}

impl ByteConversion for i32 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() == std::mem::size_of::<i32>() {
            Some(i32::from_ne_bytes(
                bytes.try_into().expect("Failed to convert bytes to i32"),
            ))
        } else {
            None
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
