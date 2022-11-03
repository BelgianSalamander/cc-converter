use crate::convert::entry_location::EntryLocation2d;

pub struct CubicChunks112Data {
    pub position: EntryLocation2d,
    pub column_data: Vec<u8>,
    pub cube_data: Vec<(i32, Vec<u8>)>,
}

impl CubicChunks112Data {
    pub fn from_data(position: EntryLocation2d, column_data: Vec<u8>, cube_data: Vec<(i32, Vec<u8>)>) -> Self {
        Self {
            position,
            column_data,
            cube_data,
        }
    }
}
