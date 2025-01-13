use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};
use std::path::Path;

// use morse;
use morse::{decode, encode};
use std::env;

const HELP: &str = "
USAGE:
  finley [ARGS] [OPTIONS]

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --input PATH         Sets an input file path
  --output PATH        Sets an output file path

ARGS:
  <enc>        encode a file from path
  <dec>        decode a file from path
";

#[derive(Debug)]
struct AppArgs {
    cmd: String,
    input: std::path::PathBuf,
    output: std::path::PathBuf,
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    // https://github.com/RazrFalcon/pico-args/issues/10
    let args = AppArgs {
        // input: pargs.value_from_str("--input")?,
        // output: pargs.value_from_str("--output")?,
        // Parses a required free-standing/positional argument.
        cmd: pargs.free_from_str()?,
        input: pargs.value_from_os_str("--input", parse_input_file)?,
        output: pargs.value_from_os_str("--output", parse_output_file)?,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();

    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

// ==========================================
fn main() {
    println!("finley v.{}", env!("CARGO_PKG_VERSION"));
    match parse_args() {
        Ok(v) => {
            // println!("main entrando");
            // println!("{:#?}", v);
            // println!("cmd: {:?}", v.cmd);
            // println!("input_file: {:?}", v.input);
            // println!("output_file: {:?}", v.output);
            match v.cmd.as_str() {
                "enc" => {
                    // println!("cmd enc listo: {:?}", v.cmd);
                    enc_file(v.input, v.output);
                }
                "dec" => {
                    // println!("cmd dec listo: {:?}", v.cmd);
                    dec_file(v.input, v.output);
                }
                _ => {
                    eprintln!("unknown command '{}'", v.cmd);
                    println!();
                    println!("try 'finley --help' for more information");
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            println!();
            eprintln!("Error: {}.", e);
            println!();
            println!("try 'finley --help' for more information");
            std::process::exit(1);
        }
    };

    // println!("{:#?}", args);
}
// ==========================================

fn parse_input_file(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    match Path::new(s).exists() {
        true => Ok(s.into()),
        false => Err("input file does not exist"),
    }
}

fn parse_output_file(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    match Path::new(s).exists() {
        true => Err("there is already a file with that name"),
        false => Ok(s.into()),
    }
}

fn enc_file(input_file_path: std::path::PathBuf, output_file_path: std::path::PathBuf) {
    // Abre el archivo
    println!("encoding file: {:?}", input_file_path);
    println!();
    let file_in = File::open(input_file_path).expect("Unable to open file");
    let file_in = BufReader::new(file_in);
    // crea el archivo
    let file_out = File::create(output_file_path.clone()).expect("Unable to create file");
    let mut file_out = BufWriter::new(file_out);

    // Lee el archivo línea por línea
    for line in file_in.lines() {
        let line = line.expect("Unable to read line");
        // println!("{}", line);

        if line.is_empty() {
            println!("skipping blank line...");
            continue;
        }
        let result = encode::encode(&line);

        match result {
            Ok(x) => {
                let text = <String as Into<String>>::into(x).replace("_", "-").trim().to_string();
                // println!("{}", text);

                file_out.write_all(text.as_bytes()).expect("Unable to write data");
                // file_out.write(text.as_bytes()).expect("Unable to write data");
                file_out.write_all(b"\n").unwrap();
            }
            Err(e) => {
                println!(
                    "encode: The following chars were unsupported {:?}->{:?}",
                    e.unsupported_characters, e.result
                );
            }
        }
    }
    println!("finishing...");
    println!("encode: file created in {:?}", output_file_path);
}

fn dec_file(input_file_path: std::path::PathBuf, output_file_path: std::path::PathBuf) {
    println!("decoding file: {:?}", input_file_path);
    println!();
    // Abre el archivo
    let file_in = File::open(input_file_path).expect("Unable to open file");
    let file_in = BufReader::new(file_in);
    // crea el archivo
    let file_out = File::create(output_file_path.clone()).expect("Unable to create file");
    let mut file_out = BufWriter::new(file_out);

    // Lee el archivo línea por línea
    for line in file_in.lines() {
        let line = line.expect("Unable to read line");
        // println!("{}", line);

        let result = decode::decode(&line);

        match result {
            Ok(x) => {
                let text = <String as Into<String>>::into(x).replace("_", "-").trim().to_string();
                // println!("{}", text);

                file_out.write_all(text.as_bytes()).expect("Unable to write data");
                // file_out.write(text.as_bytes()).expect("Unable to write data");
                file_out.write_all(b"\n").unwrap();
            }
            Err(e) => {
                println!(
                    "decode: The following chars were unsupported {:?}->{:?}",
                    e.unsupported_characters, e.result
                );
            }
        }
    }
    println!("finishing...");
    println!("decode: file created in '{:?}'", output_file_path);
}
