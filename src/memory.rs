use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
};

pub fn open_process(pid: u32) -> File {
    let path = format!("/proc/{}/mem", pid);

    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(path)
    {
        Ok(value) => value,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    file
}

pub fn read_mem<T: ByteConversion>(process_file: &mut File, offset: u64, result: &mut T) {
    process_file
        .seek(SeekFrom::Start(offset))
        .expect("Couldn't seek offset");

    let size = std::mem::size_of::<T>();

    let mut buf: Vec<u8> = Vec::with_capacity(size);
    buf.resize(size, 0);

    process_file
        .read(&mut buf[..])
        .expect("Failed to read address");

    *result = result.from_bytes(buf);
}

pub fn write_mem<T: ByteConversion>(process_file: &mut File, offset: u64, value: T) {
    process_file
        .seek(SeekFrom::Start(offset))
        .expect("Couldn't seek offset");

    let buf = value.to_bytes();

    process_file
        .write(&buf[..])
        .expect("Failed to write address");
}

pub trait ByteConversion {
    fn from_bytes(&self, bytes: Vec<u8>) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
}

impl ByteConversion for i32 {
    fn from_bytes(&self, bytes: Vec<u8>) -> Self {
        let converted = i32::from_ne_bytes(bytes.try_into().expect("Couldn't convert bytes"));
        converted
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
