use std::io::{self, Write};

mod memory;
use memory::ProcessMemory;

fn main() -> io::Result<()> {
    print("PID: ");
    let pid = match read_input()?.parse::<u32>() {
        Ok(pid) => pid,
        Err(err) => {
            eprintln!("Error parsing PID: {}", err);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, err));
        }
    };
    let mut file_process = ProcessMemory::open(pid)?;

    print("Option: Read/Write (R/W): ");
    let option = read_input()?;
    match option.as_str() {
        "R" | "r" => {
            print("Offset: ");
            let offset_input = read_input()?;
            let offset = match u64::from_str_radix(&offset_input.trim_start_matches("0x"), 16) {
                Ok(offset) => offset,
                Err(err) => {
                    eprintln!("Error parsing offset: {}", err);
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, err));
                }
            };

            let result: i32 = file_process.read(offset)?;
            println!("Decoded: {}", result);
        }
        "W" | "w" => {
            print("Offset: ");
            let offset_input = read_input()?;
            let offset = match u64::from_str_radix(&offset_input.trim_start_matches("0x"), 16) {
                Ok(offset) => offset,
                Err(err) => {
                    eprintln!("Error parsing offset: {}", err);
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, err));
                }
            };

            print("Value: ");
            let value_input = read_input()?;
            let value = match value_input.parse::<i32>() {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Error parsing value: {}", err);
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, err));
                }
            };
            file_process.write(offset, value)?;
        }
        _ => (),
    }

    Ok(())
}

fn print(buffer: &str) {
    print!("{}", buffer);
    std::io::stdout().flush().unwrap();
}

fn read_input() -> io::Result<String> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;

    Ok(user_input.trim().to_owned())
}
