mod lexer;
mod parser;
mod emitter;
use std::{env, fs::{self, File}, io::ErrorKind, path::PathBuf};
use crate::emitter::*;
use crate::lexer::*;
use crate::parser::*;

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

#[derive(Debug)]
enum OutputFormat {
    FakeBinary,
    RealBinary,
}

#[derive(Debug)]
enum Endianess{
    BigEndian,
    LittleEndian
}

#[derive(Debug)]
struct AppArgs {
    input: std::path::PathBuf,
    output: std::path::PathBuf,
    format: OutputFormat,
    endianess: Endianess,
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

    let out_filename: String = String::from("/Users/nico/Documents/Code stuff/Niisc16/assembler/main.out");

    let code = fs::read_to_string(&args.input).expect("error with opening / finding the file");

    if code.is_empty() {
        panic!("Empty file was provided");
    }

    let mut iter = code.chars().enumerate().peekable();
    
    let mut parser = Parser::init(&code, &mut iter);

    match parser.program() {
        Ok((all_tokens, all_labels)) => {
            println!("Tokens found: {:?}", all_tokens);
            println!("Labels found: {:?}", all_labels);

            
        },
        Err(x) => panic!("{}",x),
    }
    /*
    match parser.program() {
        Ok((all_tokens, all_labels)) => {
            let mut emitter = Emitter::init(&out_filename, all_labels, all_tokens);
            emitter.emit_all();
            
        },
        Err(x) => panic!("{}",x),
    }
    */
        
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

        format: pargs.opt_value_from_fn("-f", parse_output_format)?.unwrap_or(OutputFormat::FakeBinary),
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
        "bigendian" | "b" | "big" => {Ok(Endianess::BigEndian)},
        "littleendian" | "l" | "little" => {Ok(Endianess::LittleEndian)},
        _ => Err("Could not understand what was provided after \"-e\"")
    }
}

fn parse_output_format(s: &str) -> Result<OutputFormat, &'static str> {
    match s.to_lowercase().as_str() {
        "fakebinary" | "f" | "fake" => {Ok(OutputFormat::FakeBinary)},
        "realbinary" | "r" | "real" => {Ok(OutputFormat::RealBinary)},
        _ => Err("Could not understand what was provided after \"-f\"")
    }
}

/*
    loop {
        let a = get_token(&code, &mut iter);
        println!("{:?}", a);
        if a.token == TokenType::EOF {
            break;
        }
    }
    */