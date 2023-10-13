use std::{
    ffi::OsStr,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write},
    path::Path,
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

pub fn find_module_base_address(pid: u32, module_name: &str) -> io::Result<Option<usize>> {
    let path = format!("/proc/{}/maps", pid);
    let file = File::open(path)?;

    for line in BufReader::new(file).lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Line should contain all expected information
        if parts.len() >= 6 {
            let module_path = Path::new(parts[5]);
            if module_path.file_name() == Some(OsStr::new(module_name)) {
                let base_address = usize::from_str_radix(&parts[0][..8], 16).ok();
                return Ok(base_address);
            }
        }
    }
    Ok(None)
}

pub trait ByteConversion: Sized {
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
    fn to_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_byte_conversion {
    ($type:ty) => {
        impl ByteConversion for $type {
            fn from_bytes(bytes: &[u8]) -> Option<Self> {
                if bytes.len() == std::mem::size_of::<Self>() {
                    Some(Self::from_ne_bytes(
                        bytes.try_into().expect("Failed to convert bytes to type"),
                    ))
                } else {
                    None
                }
            }

            fn to_bytes(&self) -> Vec<u8> {
                self.to_ne_bytes().to_vec()
            }
        }
    };
}

impl_byte_conversion!(i32);
impl_byte_conversion!(u32);
impl_byte_conversion!(i64);
impl_byte_conversion!(u64);
