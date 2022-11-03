use byteorder::{BigEndian, WriteBytesExt};
use flate2::bufread::{GzDecoder, ZlibDecoder};
use quartz_nbt::io::Flavor::GzCompressed;
use quartz_nbt::io::{Flavor, NbtIoError};
use quartz_nbt::NbtCompound;
use Flavor::Uncompressed;

pub fn read_compressed(data: &[u8]) -> Result<NbtCompound, NbtIoError> {
    let format = data[0];
    let data = &data[1..];
    if data.is_empty() {
        return Err(NbtIoError::Custom(format!("Format: {}", format).into_boxed_str()));
    }

    let (compound, _) = match format {
        1 => quartz_nbt::io::read_nbt(&mut GzDecoder::new(data), Uncompressed),
        2 => quartz_nbt::io::read_nbt(&mut ZlibDecoder::new(data), Uncompressed),
        _ => {
            return Err(NbtIoError::Custom(
                format!("Unrecognised compression format {}. (Valid options are: 1, 2)", format).into_boxed_str(),
            ));
        }
    }?;
    Ok(compound)
}

pub fn read_compressed_cc(data: &[u8]) -> Result<NbtCompound, NbtIoError> {
    Ok(quartz_nbt::io::read_nbt(&mut GzDecoder::new(data), Uncompressed)?.0)
}

pub fn write_compressed(tag: &NbtCompound, prefix_format: bool) -> Result<Vec<u8>, NbtIoError> {
    let mut data = Vec::new();

    if prefix_format {
        data.write_i32::<BigEndian>(1)?;
    }
    quartz_nbt::io::write_nbt(&mut std::io::Cursor::new(&mut data), None, tag, GzCompressed)?;
    Ok(data)
}
