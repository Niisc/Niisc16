mod lexer;
mod parser;
mod emitter;
use std::{arch::x86_64, env, fs::{self, File}, io::ErrorKind, path::PathBuf};
use crate::emitter::*;
use crate::lexer::*;
use crate::parser::*;
use std::io::prelude::*;

const HELP: &str = "\
App

USAGE:
  app <ARGS> <OPTIONS>

FLAGS:
  -h, --help            Prints help information

ARGS:
  <INPUT FILE>

OPTIONS:
  -e                    Sets the ouput endianess of the compiled code
  -f                    Sets the output format of the compiled code (can be real or fake binary)
  -o PATH               Sets the output path

EXAMPLES:
  app some_code.nasm -o some_output.out -e littleendian -f fakebinary
  app some_code.nasm -o some_output.out -e bigendian -f realbinary
  app some_code.nasm -e b -f r
";

#[derive(Debug, PartialEq, Clone)]
enum OutputLineFormat {
    NotSeparated,
    Separated8Bit,
    Separated16Bit,

}

#[derive(Debug, PartialEq)]
enum OutputNumberFormat {
    FakeBinary,
    RealBinary,
    Hex,
    Decimal
}

#[derive(Debug, PartialEq)]
enum Endianess{
    BigEndian,
    LittleEndian
}

#[derive(Debug)]
struct AppArgs {
    input: std::path::PathBuf,
    output: std::path::PathBuf,
    number_format: OutputNumberFormat,
    endianess: Endianess,
    line_format: OutputLineFormat,
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}. Use -h or --help for help.", e);
            std::process::exit(1);
        }
    };
    println!("{:?}, {}", args, env::current_dir().unwrap().display());
    //problems may arise with little / big endian

    let out_filename: String = String::from("");

    let code = fs::read_to_string(&args.input).expect("error with opening / finding the file");

    if code.is_empty() {
        panic!("Empty file was provided");
    }

    let mut iter = code.chars().enumerate().peekable();
    
    let mut parser = Parser::init(&code, &mut iter, args.line_format.clone());

    /*
    match parser.program() {
        Ok((current_tokens, all_labels)) => {
            for x in current_tokens {
                println!("{} {}", x.token, x.data);
            }
            println!();
            for x in all_labels {
                println!("Labels found: {:?}", x);
            }
            //println!("Tokens found: {:?}", all_tokens);
            //println!("Labels found: {:?}", all_labels);
        },
        Err(x) => panic!("{}",x),
    }
    */
    let mut emitter_code: &mut Vec<u16>;

    match parser.program() {

        Ok((all_tokens, all_labels)) => {

            let mut emitter = Emitter::init(&out_filename, all_labels, all_tokens);
            let mut file = File::create(&args.output).unwrap();
            
            emitter_code  = emitter.emit_all();

            if args.endianess == Endianess::LittleEndian {
                for x in emitter_code.iter_mut() {
                    *x = x.swap_bytes();
                }
            }
            let mut final_code = String::new();
            match args.number_format {
                OutputNumberFormat::FakeBinary =>{
                    match args.line_format {
                        OutputLineFormat::NotSeparated => {
                            for x in emitter_code {
                                final_code.push_str(format!("{:#b}",x).as_str());
                            }
                        }
                        OutputLineFormat::Separated8Bit => {
                            for x in emitter_code {
                                final_code.push_str(format!("{:#b}\n{:#b}\n", ((*x & 0b1111111100000000)>>8)as u8, (*x & 0b11111111) as u8).as_str());
                            }
                            if !final_code.is_empty() {
                                final_code.pop(); // Removes the last newline character if the string is not empty
                            }
                        }
                        OutputLineFormat::Separated16Bit => {
                            for x in emitter_code {
                                final_code.push_str(format!("{:#b}\n",x).as_str());
                            }
                            if !final_code.is_empty() {
                                final_code.pop(); // Removes the last newline character if the string is not empty
                            }
                            //maybe pop last character?
                        }
                    }
                    
                }
                OutputNumberFormat::RealBinary=>{
                    unimplemented!("RealBinary isnt implemented")
                }
                OutputNumberFormat::Hex=> {
                    match args.line_format {
                        OutputLineFormat::NotSeparated => {
                            for x in emitter_code {
                                final_code.push_str(format!("{:#x}",x).as_str());
                            }
                        }
                        OutputLineFormat::Separated8Bit => {
                            for x in emitter_code {
                                final_code.push_str(format!("{:#x}\n{:#x}\n", ((*x & 0b1111111100000000)>>8)as u8, (*x & 0b11111111) as u8).as_str());
                            }
                            if !final_code.is_empty() {
                                final_code.pop(); // Removes the last newline character if the string is not empty
                            }
                        }
                        OutputLineFormat::Separated16Bit => {
                            for x in emitter_code {
                                final_code.push_str(format!("{:#x}\n",x).as_str());
                            }
                            if !final_code.is_empty() {
                                final_code.pop(); // Removes the last newline character if the string is not empty
                            }
                        }
                    }
                }
                OutputNumberFormat::Decimal=>{
                    match args.line_format {
                        OutputLineFormat::NotSeparated => {
                            for x in emitter_code {
                                final_code.push_str(format!("{}",x).as_str());
                            }
                        }
                        OutputLineFormat::Separated8Bit => {
                            for x in emitter_code {
                                final_code.push_str(format!("{}\n{}\n", ((*x & 0b1111111100000000)>>8)as u8, (*x & 0b11111111) as u8).as_str());
                            }
                            if !final_code.is_empty() {
                                final_code.pop(); // Removes the last newline character if the string is not empty
                            }
                        }
                        OutputLineFormat::Separated16Bit => {
                            for x in emitter_code {
                                final_code.push_str(format!("{}\n",x).as_str());
                            }
                            if !final_code.is_empty() {
                                final_code.pop(); // Removes the last newline character if the string is not empty
                            }
                        }
                    }
                }
            }
            file.write_all(final_code.as_bytes()).unwrap();
        },
        Err(x) => panic!("{}",x),
    }

        
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = AppArgs {

        output: pargs.opt_value_from_os_str("-o", parse_path)?.unwrap_or(PathBuf::from("a.out")),

        input: pargs.free_from_os_str(parse_path)?,

        endianess: pargs.opt_value_from_fn("-e",parse_endianess)?.unwrap_or(Endianess::BigEndian),

        number_format: pargs.opt_value_from_fn("-f", parse_output_number_format)?.unwrap_or(OutputNumberFormat::FakeBinary),

        line_format: pargs.opt_value_from_fn("-l", parse_output_line_format)?.unwrap_or(OutputLineFormat::NotSeparated),

    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}

fn parse_endianess(s: &str) -> Result<Endianess, &'static str> {
    match s.to_lowercase().as_str() {
        "bigendian"     | "b" | "big" => {Ok(Endianess::BigEndian)},
        "littleendian"  | "l" | "little" => {Ok(Endianess::LittleEndian)},
        _ => Err("Could not understand what was provided after \"-e\"")
    }
}

fn parse_output_number_format(s: &str) -> Result<OutputNumberFormat, &'static str> {
    match s.to_lowercase().as_str() {
        "fakebinary" | "f" | "fake" => {Ok(OutputNumberFormat::FakeBinary)},
        "realbinary" | "r" | "real" => {Ok(OutputNumberFormat::RealBinary)},
        "decimal"    | "d" | "10" => {Ok(OutputNumberFormat::Decimal)},
        "hex"        | "h" | "16" => {Ok(OutputNumberFormat::Hex)},
        _ => Err("Could not understand what was provided after \"-f\"")
    }
}

fn parse_output_line_format(s: &str) -> Result<OutputLineFormat, &'static str> {
    match s.to_lowercase().as_str() {
        "notseparated"  | "n" | "not" => {Ok(OutputLineFormat::NotSeparated)},
        "separated8bit" | "8" | "8bit" => {Ok(OutputLineFormat::Separated8Bit)},
        "separated16bit"| "16"| "16bit" => {Ok(OutputLineFormat::Separated16Bit)},

        _ => Err("Could not understand what was provided after \"-f\"")
    }
}
