use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    process
};
use ctrlc;

const VERSION: &str = "1.1.1";
const MAX_DATA_SIZE: usize = 32768; // 定义最大数据单元格大小
const PROGRAM_NAME: &str = if cfg!(windows) { "brainfuck.exe" } else { "brainfuck" };

static mut DATA: [u8; MAX_DATA_SIZE] = [0; MAX_DATA_SIZE];
static mut DATA_PTR: usize = 0;
fn main() {
    let args: Vec<String> = env::args().collect();

    // 设置 Ctrl+C 信号处理
    ctrlc::set_handler(|| {
        println!("^C");
        process::exit(-1);
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
    print_version();
    println!("Usage:");
    println!("  {}                     Enter interactive mode", PROGRAM_NAME);
    println!("  {} <file.bf>          Run a Brainfuck program from a file", PROGRAM_NAME);
    println!("Options:");
    println!("  --help, -h            Show this help message");
    println!("  --version, -v         Show version information");
}

fn print_version() {
    println!("Brainfuck Interpreter version {}", VERSION);
}

fn interactive_mode() {
    println!("Brainfuck Interpreter (interactive mode) version {}", VERSION);
    println!("Type your Brainfuck code here. Type Ctrl+C to quit.");

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
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error: {}", e)
            },
        }
    }
}

fn run_file(file_path: &str) -> Result<(), String> {
    let file: File = File::open(file_path).map_err(|e: io::Error| format!("Failed to open file: {}", e))?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut code: String = String::new();
    for line in reader.lines() {
        code.push_str(&line.map_err(|e| format!("Failed to read line: {}", e))?);
    }

    run_code(&code)
}

fn run_code(code: &str) -> Result<(), String> {
    let mut code_ptr: usize = 0;
    let code: Vec<char> = code.chars().collect();

    unsafe {
        while code_ptr < code.len() {
            match code[code_ptr] {
                '>' => {
                    if DATA_PTR >= MAX_DATA_SIZE {
                        return Err("Data pointer out of bounds (>)".to_string());
                    }
                    DATA_PTR += 1;
                },
                '<' => {
                    if DATA_PTR == 0 {
                        return Err("Data pointer out of bounds (<)".to_string());
                    }
                    DATA_PTR -= 1;
                },
                '+' => {
                    DATA[DATA_PTR] = DATA[DATA_PTR].wrapping_add(1)
                },
                '-' => {
                    DATA[DATA_PTR] = DATA[DATA_PTR].wrapping_sub(1)
                },
                '.' => {
                    print!("{}", DATA[DATA_PTR] as char);
                    io::stdout().flush().unwrap();
                },
                ',' => {
                    let mut input: String = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    DATA[DATA_PTR] = input.as_bytes()[0];
                },
                '[' => {
                    if DATA[DATA_PTR] == 0 {
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
                },
                ']' => {
                    if DATA[DATA_PTR] != 0 {
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
                },
                _ => {},
            }
            code_ptr += 1;
        }
    }
    Ok(())
}