use crate::util::positions::{RegionPos2d, RegionPos3d};
use crate::util::vec::vec2i::Vec2i;
use crate::util::vec::vec3i::Vec3i;
use crate::util::vec::CoordinateSpace;

pub type RegionKey = String;

pub trait Key<R> {
    fn to_region_pos(self) -> R;
    fn region_key(&self) -> RegionKey;
    fn id(&self) -> usize;
}

pub struct EntryLocation3dSpace {}
impl CoordinateSpace for EntryLocation3dSpace {}

pub type EntryLocation3d = Vec3i<EntryLocation3dSpace>;
impl EntryLocation3d {
    const LOC_BITS: usize = 4;
    const LOC_BITMASK: usize = (1 << Self::LOC_BITS) - 1;
    pub const ENTRIES_PER_REGION: usize = (1 << Self::LOC_BITS) * (1 << Self::LOC_BITS) * (1 << Self::LOC_BITS);
}

impl Key<RegionPos3d> for EntryLocation3d {
    fn to_region_pos(self) -> RegionPos3d {
        RegionPos3d::new(self.x >> Self::LOC_BITS, self.y >> Self::LOC_BITS, self.z >> Self::LOC_BITS)
    }

    fn region_key(&self) -> RegionKey {
        let reg_x = self.x >> Self::LOC_BITS;
        let reg_y = self.y >> Self::LOC_BITS;
        let reg_z = self.z >> Self::LOC_BITS;

        format!("{}.{}.{}.3dr", reg_x, reg_y, reg_z)
    }

    fn id(&self) -> usize {
        ((self.x as usize & Self::LOC_BITMASK) << Self::LOC_BITS * 2)
            | ((self.y as usize & Self::LOC_BITMASK) << Self::LOC_BITS)
            | (self.z as usize & Self::LOC_BITMASK)
    }
}

pub struct EntryLocation2dSpace {}
impl CoordinateSpace for EntryLocation2dSpace {}

pub type EntryLocation2d = Vec2i<EntryLocation2dSpace>;
impl EntryLocation2d {
    const LOC_BITS: usize = 5;
    const LOC_BITMASK: usize = (1 << Self::LOC_BITS) - 1;
    const ENTRIES_PER_REGION: usize = (1 << Self::LOC_BITS) * (1 << Self::LOC_BITS);
}

impl Key<RegionPos2d> for EntryLocation2d {
    fn to_region_pos(self) -> RegionPos2d {
        RegionPos2d::new(self.x >> Self::LOC_BITS, self.z >> Self::LOC_BITS)
    }

    fn region_key(&self) -> RegionKey {
        format!("{}.{}.2dr", self.x, self.z)
    }

    fn id(&self) -> usize {
        ((self.x as usize & Self::LOC_BITMASK) << Self::LOC_BITS) | (self.z as usize & Self::LOC_BITMASK)
    }
}

pub struct MinecraftChunkSpace {}
impl CoordinateSpace for MinecraftChunkSpace {}

pub type MinecraftChunkLocation = Vec2i<MinecraftChunkSpace>;
impl MinecraftChunkLocation {
    const LOC_BITS: usize = 5;
    const LOC_BITMASK: usize = (1 << Self::LOC_BITS) - 1;
    const ENTRIES_PER_REGION: usize = (1 << Self::LOC_BITS) * (1 << Self::LOC_BITS);

    pub fn to_entry_location_2d(self) -> EntryLocation2d {
        EntryLocation2d {
            x: self.x,
            z: self.z,
            _phantom: Default::default(),
        }
    }
}

impl RegionPos2d {
    pub fn to_minecraft_chunk_location(self) -> MinecraftChunkLocation {
        self.to_minecraft_chunk_location_offset(0, 0)
    }
    pub fn to_minecraft_chunk_location_offset(self, local_x: i32, local_z: i32) -> MinecraftChunkLocation {
        MinecraftChunkLocation::new((self.x << 5) + local_x, (self.z << 5) + local_z)
    }
}

impl Key<RegionPos2d> for MinecraftChunkLocation {
    fn to_region_pos(self) -> RegionPos2d {
        RegionPos2d::new(self.x >> Self::LOC_BITS, self.z >> Self::LOC_BITS)
    }

    fn region_key(&self) -> RegionKey {
        format!("r.{}.{}.mca", self.x, self.z)
    }

    fn id(&self) -> usize {
        ((self.x as usize & Self::LOC_BITMASK) << Self::LOC_BITS) | (self.z as usize & Self::LOC_BITMASK)
    }
}
