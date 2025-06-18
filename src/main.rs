use clap::{self, Parser};
use std::io::{Read, Write};
use std::path::PathBuf;
mod codec;

#[derive(clap::Parser)]
#[command(
    name = "vinegar",
    version = "0.1.0",
    author = "XTZ206",
    about = "A simple command line byte stream codec tool"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, help = "input file path (stdin if not provided)")]
    input: Option<PathBuf>,
    #[arg(short, long, help = "output file path (stdout if not provided)")]
    output: Option<PathBuf>,

    #[arg(short, long, help = "enable debug output", default_value_t = false)]
    debug: bool,
}

#[derive(clap::Subcommand)]
enum Commands {
    #[command(about = "encode data using the codec")]
    Encode,
    #[command(about = "decode data using the codec")]
    Decode,
}

fn main() {
    let cli = Cli::parse();
    let mut buffer: Vec<u8> = Vec::new();
    match &cli.command {
        Commands::Encode => {
            eprintln!("reading data");
            if let Some(path) = &cli.input {
                eprintln!("reading from file: {}", path.display());
                let mut file = std::fs::File::open(path).expect("failed to open input file");
                file.read_to_end(&mut buffer)
                    .expect("failed to read from input file");
            } else {
                eprintln!("reading from stdin");
                std::io::stdin()
                    .read_to_end(&mut buffer)
                    .expect("failed to read from stdin");
            }
            eprintln!("reading completed, {} bytes in buffer", buffer.len());
            if cli.debug {
                eprintln!("debug mode enabled, buffer content:");
                eprint!("{}", codec::display(&buffer));
            }

            let encoded = codec::encode(&buffer);
            eprintln!("encoding completed, {} bytes in buffer", encoded.len());
            if cli.debug {
                eprintln!("debug mode enabled, buffer content:");
                eprint!("{}", codec::display(&encoded));
            }

            if let Some(path) = &cli.output {
                eprintln!("writing to file: {}", path.display());
                let mut file = std::fs::File::create(path).expect("failed to create output file");
                file.write_all(&encoded)
                    .expect("failed to write to output file");
            } else {
                eprintln!("writing to stdout");
                std::io::stdout()
                    .write_all(&encoded)
                    .expect("failed to write to stdout");
            }
        }
        Commands::Decode => {
            eprintln!("reading data");
            if let Some(path) = &cli.input {
                eprintln!("reading from file: {}", path.display());
                let mut file = std::fs::File::open(path).expect("failed to open input file");
                file.read_to_end(&mut buffer)
                    .expect("failed to read from input file");
            } else {
                eprintln!("reading from stdin");
                std::io::stdin()
                    .read_to_end(&mut buffer)
                    .expect("failed to read from stdin");
            }
            eprintln!("reading completed, {} bytes in buffer", buffer.len());
            if cli.debug {
                eprintln!("debug mode enabled, buffer content:");
                eprint!("{}", codec::display(&buffer));
            }

            let decoded = codec::decode(&buffer);
            eprintln!("decoding completed, {} bytes in buffer", decoded.len());
            if cli.debug {
                eprintln!("debug mode enabled, buffer content:");
                eprint!("{}", codec::display(&decoded));
            }

            if let Some(path) = &cli.output {
                eprintln!("writing to file: {}", path.display());
                let mut file = std::fs::File::create(path).expect("failed to create output file");
                file.write_all(&decoded)
                    .expect("failed to write to output file");
            } else {
                eprintln!("writing to stdout");
                std::io::stdout()
                    .write_all(&decoded)
                    .expect("failed to write to stdout");
            }
        }
    }
}
