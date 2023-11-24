use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::{png::Png, Result};
use std::fs::OpenOptions;
use std::future::IntoFuture;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// encode message into png
    Encode {
        file_path: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    /// decode message from png
    Decode {
        file_path: PathBuf,
        chunk_type: String,
    },
    /// remove message from png
    Remove {
        file_path: PathBuf,
        chunk_type: String,
    },
    /// print message from png
    Print { file_path: PathBuf },
}

pub fn handle() -> Result<()> {
    let cli = Cli::parse();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(c) => match c {
            Commands::Encode {
                file_path,
                chunk_type,
                message,
                output_file,
            } => encode(file_path, chunk_type, message, output_file)?,
            Commands::Decode {
                file_path,
                chunk_type,
            } => decode(file_path, chunk_type)?,
            Commands::Remove {
                file_path,
                chunk_type,
            } => remove(file_path, chunk_type)?,
            Commands::Print { file_path } => println!("000"),
        },
        None => todo!(),
    }
    Ok(())
}

fn encode(
    file_path: &PathBuf,
    chunk_type: &str,
    message: &str,
    output_file: &Option<PathBuf>,
) -> Result<()> {
    let input = OpenOptions::new().read(true).open(file_path)?;
    let mut buf_reader = BufReader::new(&input);
    let mut buffer = Vec::new();

    // Read file into vector.
    buf_reader.read_to_end(&mut buffer)?;
    let mut png = Png::try_from(buffer.as_slice())?;
    let chunk = Chunk::new(
        ChunkType::from_str(&chunk_type)?,
        message.as_bytes().to_vec(),
    );
    png.append_chunk(chunk);
    match output_file {
        Some(out_path) => {
            let mut out_file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(out_path)?;
            out_file.write_all(png.as_bytes().as_slice())?;
        }
        None => {
            let mut input = OpenOptions::new().write(true).truncate(true).open(file_path)?;
            input.write_all(png.as_bytes().as_slice())?;
        }
    }
    Ok(())
}

fn decode(file_path: &PathBuf, chunk_type: &str) -> Result<()> {
    let input = OpenOptions::new()
        .read(true) // 如果要追加写入，请设置为 true；否则，如果要覆盖文件内容，请设置为 false
        .open(file_path)?;
    let mut buf_reader = BufReader::new(&input);
    let mut buffer = Vec::new();

    // Read file into vector.
    buf_reader.read_to_end(&mut buffer)?;
    let png = Png::try_from(buffer.as_slice())?;
    let chunk_by_type = png.chunk_by_type(chunk_type);
    match chunk_by_type {
        Some(chunk) => println!("{}", String::from_utf8(chunk.data().to_vec())?),
        None => Err("no message")?,
    }
    Ok(())
}

fn remove(file_path: &PathBuf, chunk_type: &str) -> Result<()> {
    let input = OpenOptions::new()
        .read(true) // 如果要追加写入，请设置为 true；否则，如果要覆盖文件内容，请设置为 false
        .open(file_path)?;
    let mut buf_reader = BufReader::new(&input);
    let mut buffer = Vec::new();

    // Read file into vector.
    buf_reader.read_to_end(&mut buffer)?;
    let mut png = Png::try_from(buffer.as_slice())?;
    let remove_chunk = png.remove_chunk(chunk_type)?;

    let mut input = OpenOptions::new()
        .write(true)
        .truncate(true) // 如果要追加写入，请设置为 true；否则，如果要覆盖文件内容，请设置为 false
        .open(file_path)?;

    input.write_all(png.as_bytes().as_slice())?;
    Ok(())
}
