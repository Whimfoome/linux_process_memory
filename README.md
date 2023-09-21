# Linux Process Memory Reader/Writer

A Rust program for reading and writing process memory in Linux. This library enables legitimate and ethical use cases for interacting with a running process's memory space. It provides a secure and controlled way to access process memory for various applications, such as debugging, analysis, and system monitoring.

It's only 2 files to showcase how it works and to learn from it: **main.rs** and **memory.rs**

## Use

You need to have **Rust** installed, clone this repo and cd into it and run `cargo run`.

I strongly suggest to look at the source code to learn how it works, if you want to use other value types/structures, you have to implement them for `ByteConversion`, and convert them into bytes that can be used for read and write in the process memory, there is an example with the **i32** type.
