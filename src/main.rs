use memory::{open_process, read_mem, write_mem};

mod memory;

fn main() {
    print("PID: ");

    let pid = read_input().parse::<u32>().unwrap();

    let mut file_process = open_process(pid);

    print("Option: Read/Write (R/W): ");
    let option = read_input();
    match option.as_str() {
        "R" | "r" => {
            let mut result: i32 = 0;

            print("Offset: ");
            let mut offset_input = read_input();
            offset_input = offset_input.trim_start_matches("0x").to_owned();
            let offset = u64::from_str_radix(&offset_input, 16).unwrap();

            read_mem::<i32>(&mut file_process, offset, &mut result);

            println!("Decoded: {}", result);
        }
        "W" | "w" => {
            print("Offset: ");
            let mut offset_input = read_input();
            offset_input = offset_input.trim_start_matches("0x").to_owned();
            let offset = u64::from_str_radix(&offset_input, 16).unwrap();

            print("Value: ");
            let value = read_input().parse::<i32>().unwrap();

            write_mem(&mut file_process, offset, value);
        }
        _ => (),
    }
}

fn print(buffer: &str) {
    use std::io::Write;

    print!("{}", buffer);
    std::io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let mut user_input = String::new();
    match std::io::stdin().read_line(&mut user_input) {
        Ok(_) => (),
        Err(_) => panic!("Couldn't read line"),
    };

    return user_input.trim().to_owned();
}
