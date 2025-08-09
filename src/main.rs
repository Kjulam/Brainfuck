use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;
use ctrlc;

const VERSION: &str = "1.0.2";
const MAX_DATA_SIZE: usize = 32768; // 定义最大数据单元格大小
const PROGRAM_NAME: &str = if cfg!(windows) { "brainfuck.exe" } else { "brainfuck" };

fn main() {
    let args: Vec<String> = env::args().collect();

    // 设置 Ctrl+C 信号处理
    ctrlc::set_handler(|| {
        println!("^C");
        process::exit(0);
    }).expect("Error setting Ctrl+C handler");

    match args.len() {
        1 => interactive_mode(),
        2 => {
            if args[1] == "--help" || args[1] == "-h" {
                print_help();
            } else if args[1] == "--version" || args[1] == "-v" {
                print_version();
            } else {
                match run_file(&args[1]) {
                    Ok(_) => {}
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        _ => {
            eprintln!("Invalid arguments. Use --help for more information.");
            process::exit(1);
        }
    }
}

fn print_help() {
    println!("Brainfuck Interpreter version {}", VERSION);
    println!("Usage:");
    println!("  {}                    Enter interactive mode", PROGRAM_NAME);
    println!("  {} <file.bf>          Run a Brainfuck program from a file", PROGRAM_NAME);
    println!("Options:");
    println!("  --help, -h            Show this help message");
    println!("  --version, -v         Show version information");
}

fn print_version() {
    println!("Brainfuck Interpreter version {}", VERSION);
}

fn interactive_mode() {
    println!("Brainfuck Interpreter (interactive mode)");
    println!("Type your Brainfuck code here. Enter an empty line to execute.");

    let mut input: String = String::new();
    loop {
        print!("Brainfuck: ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            continue;
        }

        match run_code(&input.trim()) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn run_file(file_path: &str) -> Result<(), String> {
    let file: File = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut code: String = String::new();
    for line in reader.lines() {
        code.push_str(&line.map_err(|e| format!("Failed to read line: {}", e))?);
    }

    run_code(&code)
}

fn run_code(code: &str) -> Result<(), String> {
    let mut data: [u8; MAX_DATA_SIZE] = [0; MAX_DATA_SIZE];
    let mut data_ptr: usize = 0;
    let mut code_ptr: usize = 0;

    let code: Vec<char> = code.chars().collect();

    while code_ptr < code.len() {
        match code[code_ptr] {
            '>' => {
                data_ptr += 1;
                if data_ptr >= MAX_DATA_SIZE {
                    return Err("Data pointer out of bounds".to_string());
                }
            }
            '<' => {
                if data_ptr == 0 {
                    return Err("Data pointer out of bounds".to_string());
                }
                data_ptr -= 1;
            }
            '+' => data[data_ptr] = data[data_ptr].wrapping_add(1),
            '-' => data[data_ptr] = data[data_ptr].wrapping_sub(1),
            '.' => {
                print!("{}", data[data_ptr] as char);
                io::stdout().flush().unwrap();
            },
            ',' => {
                let mut input: String = String::new();
                io::stdin().read_line(&mut input).unwrap();
                data[data_ptr] = input.as_bytes()[0];
            }
            '[' => {
                if data[data_ptr] == 0 {
                    let mut loop_count: isize = 1;
                    while loop_count > 0 {
                        code_ptr += 1;
                        if code_ptr >= code.len() {
                            return Err("Unmatched '['".to_string());
                        }
                        match code[code_ptr] {
                            '[' => loop_count += 1,
                            ']' => loop_count -= 1,
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if data[data_ptr] != 0 {
                    let mut loop_count: isize = 1;
                    while loop_count > 0 {
                        code_ptr -= 1;
                        if code_ptr == 0 {
                            return Err("Unmatched ']'".to_string());
                        }
                        match code[code_ptr] {
                            '[' => loop_count -= 1,
                            ']' => loop_count += 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
        code_ptr += 1;
    }

    Ok(())
}