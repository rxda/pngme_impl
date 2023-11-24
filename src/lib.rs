#![allow(dead_code)]
#![allow(unused_variables)]


mod chunk;
mod chunk_type;
mod commands;
mod png;
pub mod command;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
