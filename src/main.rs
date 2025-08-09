use std::io;
use std::io::Write;
use std::io::Error;
use std::process;
use ctrlc;
// use std::env;

const VERSION: &str = "250809a";
const AUTHOR: &str = "Kjulam";

fn main() -> Result<(), Error> {
    println!("Brainfuck interpreter version {} by {}", VERSION, AUTHOR);

    // 设置信号处理函数
    ctrlc::set_handler(move || {
        println!("^C");
        process::exit(0);
    }).expect("Error: Failed to set Ctrl-C handler");

    println!("Press Ctrl-C to stop the program.");

    loop {
        let mut brainfuck_code: String = String::new();
        print!("Brainfuck: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut brainfuck_code).expect("Error: Failed to read line");
        let brainfuck_code: &str = brainfuck_code.trim();
        // println!("Your Brainfuck code is: {}", brainfuck_code);
        for char in brainfuck_code.chars() {
            println!("{}", char);
        }
    }
    Ok(())
}
