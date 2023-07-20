use std::fs;
use std::str::FromStr;
use anyhow::anyhow;
use crate::args::{
    EncodeArgs,
    DecodeArgs,
    RemoveArgs,
    PrintArgs
};
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::chunk::Chunk;
use crate::Result;

pub fn encode(EncodeArgs { 
    file_path, 
    chunk_type, 
    message 
}: EncodeArgs) -> Result<()> {
    let mut png = Png::try_from_path(file_path.as_path())?;

    let chunk_type = ChunkType::from_str(chunk_type.as_str())?;
    png.append_chunk(Chunk::new(chunk_type, message.as_bytes().to_vec()));

    Ok(fs::write(file_path, png.as_bytes())?)
}

pub fn decode(DecodeArgs {
    file_path,
    chunk_type
}: DecodeArgs) -> Result<()> {
    let png = Png::try_from_path(file_path.as_path())?;

    let chunk = match png.chunk_by_type(chunk_type.as_str()) {
        Some(chunk) => chunk,
        None => return Err(anyhow!("No such chunk_type found"))
    };

    println!("{}", chunk.data_as_string()?);
    Ok(())
}

pub fn remove(RemoveArgs {
    file_path,
    chunk_type
}: RemoveArgs) -> Result<()> {
    let mut png = Png::try_from_path(file_path.as_path())?;
    
    png.remove_chunk(chunk_type.as_str())?;

    Ok(fs::write(file_path, png.as_bytes())?)
}

pub fn print(PrintArgs {
    file_path
}: PrintArgs) -> Result<()> {
    let png = Png::try_from_path(file_path.as_path())?;

    println!("{:?}", png.header());

    for chunk in png.chunks() {
        println!("{}", chunk.data_as_string()?);
    }

    Ok(())
}