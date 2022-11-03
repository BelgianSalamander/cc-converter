use crate::convert::entry_location::RegionKey;
use crate::util::vec::vec2i::Vec2i;
use crate::util::vec::vec3i::Vec3i;
use crate::util::vec::CoordinateSpace;

pub struct BlockSpace {}
impl CoordinateSpace for BlockSpace {}
pub type BlockPos = Vec3i<BlockSpace>;

impl BlockPos {
    pub fn to_cube_pos(self) -> CubePos {
        CubePos::new(self.x >> 4, self.y >> 4, self.z >> 4)
    }

    pub fn to_3dr_pos(self) -> RegionPos3d {
        RegionPos3d::new(self.x >> 8, self.y >> 8, self.z >> 8)
    }
}

pub struct CubeSpace {}
impl CoordinateSpace for CubeSpace {}
pub type CubePos = Vec3i<CubeSpace>;

impl CubePos {
    pub fn to_block_pos(self) -> BlockPos {
        self.to_block_pos_offset(0, 0, 0)
    }
    pub fn to_block_pos_offset(self, local_x: i32, local_y: i32, local_z: i32) -> BlockPos {
        BlockPos::new((self.x << 4) + local_x, (self.y << 4) + local_y, (self.z << 4) + local_z)
    }

    pub fn to_3dr_pos(self) -> RegionPos3d {
        RegionPos3d::new(self.x >> 4, self.y >> 4, self.z >> 4)
    }
}

pub struct RegionSpace3d {}
impl CoordinateSpace for RegionSpace3d {}
pub type RegionPos3d = Vec3i<RegionSpace3d>;

impl RegionPos3d {
    pub const DIAMETER_IN_CUBES: usize = 16;
    pub const CUBES_COUNT: usize = Self::DIAMETER_IN_CUBES * Self::DIAMETER_IN_CUBES * Self::DIAMETER_IN_CUBES;

    pub fn to_block_pos(self) -> BlockPos {
        BlockPos::new(self.x << 8, self.y << 8, self.z << 8)
    }
    pub fn to_block_pos_offset(self, local_x: i32, local_y: i32, local_z: i32) -> BlockPos {
        BlockPos::new((self.x << 8) + local_x, (self.y << 8) + local_y, (self.z << 8) + local_z)
    }

    pub fn to_cube_pos(self) -> CubePos {
        self.to_cube_pos_offset(0, 0, 0)
    }
    pub fn to_cube_pos_offset(self, local_x: i32, local_y: i32, local_z: i32) -> CubePos {
        CubePos::new((self.x << 4) + local_x, (self.y << 4) + local_y, (self.z << 4) + local_z)
    }

    pub fn region_key(&self) -> RegionKey {
        format!("{}.{}.{}.3dr", self.x, self.y, self.z)
    }
}

pub struct ChunkSpace {}
impl CoordinateSpace for ChunkSpace {}
pub type ChunkPos = Vec2i<ChunkSpace>;

impl ChunkPos {
    pub fn to_2dr_pos(self) -> RegionPos2d {
        RegionPos2d::new(self.x >> 5, self.z >> 5)
    }
}

pub struct RegionSpace2d {}
impl CoordinateSpace for RegionSpace2d {}
pub type RegionPos2d = Vec2i<RegionSpace2d>;

impl RegionPos2d {
    pub const DIAMETER_IN_CHUNKS: usize = 32;
    pub const CHUNKS_COUNT: usize = Self::DIAMETER_IN_CHUNKS * Self::DIAMETER_IN_CHUNKS;

    pub fn to_chunk_pos(self) -> ChunkPos {
        self.to_chunk_pos_offset(0, 0)
    }
    pub fn to_chunk_pos_offset(self, local_x: i32, local_z: i32) -> ChunkPos {
        ChunkPos::new((self.x << 5) + local_x, (self.z << 5) + local_z)
    }

    pub fn region_key(&self) -> RegionKey {
        format!("r.{}.{}.mca", self.x, self.z)
    }
}
