use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use byteorder::{LittleEndian, WriteBytesExt};

use lazy_static::lazy_static;

use crate::convert::anvil2cc::Anvil2CCConversionError::{InvalidData, NbtIo, NbtRepr, NbtStructure, Other};
use quartz_nbt::io::NbtIoError;
use quartz_nbt::NbtTag::{Byte, ByteArray, Compound, Int, IntArray, String};
use quartz_nbt::{NbtCompound, NbtList, NbtReprError, NbtStructureError, NbtTag};

use crate::convert::data::anvil_chunk_data::AnvilChunkData;
use crate::convert::data::cubic_chunks_1_12::CubicChunks112Data;
use crate::util::compress::{read_compressed, write_compressed};
use crate::util::reinterpret::vec_u8_into_i8;

lazy_static! {
    static ref TE_REGISTRY: HashMap<i32, &'static str> = create_te_registry();
}

fn create_te_registry() -> HashMap<i32, &'static str> {
    let mut registry = HashMap::new();
    registry.insert(61, "furnace");
    registry.insert(62, "furnace");
    registry.insert(54, "chest");
    registry.insert(146, "chest");
    registry.insert(130, "ender_chest");
    registry.insert(84, "jukebox");
    registry.insert(23, "dispenser");
    registry.insert(158, "dropper");
    registry.insert(63, "sign");
    registry.insert(68, "sign");
    registry.insert(52, "mob_spawner");
    registry.insert(25, "noteblock");
    // registry.insert(, "piston");
    registry.insert(117, "brewing_stand");
    registry.insert(116, "enchanting_table");
    registry.insert(119, "end_portal");
    registry.insert(138, "beacon");
    registry.insert(144, "skull");
    registry.insert(151, "daylight_detector");
    registry.insert(178, "daylight_detector");
    registry.insert(154, "hopper");
    registry.insert(149, "comparator");
    registry.insert(150, "comparator");
    registry.insert(140, "flower_pot");
    registry.insert(176, "banner");
    registry.insert(177, "banner");
    registry.insert(255, "structure_block");
    registry.insert(209, "end_gateway");
    registry.insert(137, "command_block");
    registry.insert(210, "command_block");
    registry.insert(211, "command_block");
    registry.insert(219, "shulker_box");
    registry.insert(220, "shulker_box");
    registry.insert(221, "shulker_box");
    registry.insert(222, "shulker_box");
    registry.insert(223, "shulker_box");
    registry.insert(224, "shulker_box");
    registry.insert(225, "shulker_box");
    registry.insert(226, "shulker_box");
    registry.insert(227, "shulker_box");
    registry.insert(228, "shulker_box");
    registry.insert(229, "shulker_box");
    registry.insert(230, "shulker_box");
    registry.insert(231, "shulker_box");
    registry.insert(232, "shulker_box");
    registry.insert(233, "shulker_box");
    registry.insert(234, "shulker_box");
    registry.insert(26, "bed");
    registry
}

pub struct InvalidChunkTagError {
    message: std::string::String,
}

impl Debug for InvalidChunkTagError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Display for InvalidChunkTagError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

pub enum Anvil2CCConversionError {
    InvalidData(InvalidChunkTagError),
    NbtRepr(NbtReprError),
    NbtStructure(NbtStructureError),
    NbtIo(NbtIoError),
    Other(Box<dyn Error>),
}

impl Debug for Anvil2CCConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidData(err) => f.write_str(&*format!("{}", err)),
            NbtRepr(err) => f.write_str(&*format!("{}", err)),
            NbtStructure(err) => f.write_str(&*format!("{}", err)),
            NbtIo(err) => f.write_str(&*format!("{}", err)),
            Other(err) => f.write_str(&*format!("{}", err)),
        }
    }
}

impl Display for Anvil2CCConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidData(err) => f.write_str(&*format!("{}", err)),
            NbtRepr(err) => f.write_str(&*format!("{}", err)),
            NbtStructure(err) => f.write_str(&*format!("{}", err)),
            NbtIo(err) => f.write_str(&*format!("{}", err)),
            Other(err) => f.write_str(&*format!("{}", err)),
        }
    }
}

impl From<InvalidChunkTagError> for Anvil2CCConversionError {
    fn from(error: InvalidChunkTagError) -> Self {
        InvalidData(error)
    }
}
impl From<NbtReprError> for Anvil2CCConversionError {
    fn from(error: NbtReprError) -> Self {
        NbtRepr(error)
    }
}
impl From<NbtStructureError> for Anvil2CCConversionError {
    fn from(error: NbtStructureError) -> Self {
        NbtStructure(error)
    }
}
impl From<NbtIoError> for Anvil2CCConversionError {
    fn from(error: NbtIoError) -> Self {
        NbtIo(error)
    }
}
impl From<std::io::Error> for Anvil2CCConversionError {
    fn from(error: std::io::Error) -> Self {
        Other(Box::new(error))
    }
}

pub struct Anvil2CCConverter {
    fix_missing_tile_entities: bool,
}

impl Anvil2CCConverter {
    pub fn new(fix_missing_tile_entities: bool) -> Self {
        Self { fix_missing_tile_entities }
    }

    pub fn convert(&self, src: AnvilChunkData) -> Result<Box<[CubicChunks112Data]>, Anvil2CCConversionError> {
        let data = CubicChunks112Data::from_data(
            src.position.to_entry_location_2d(),
            self.extract_column_data(src.data)?,
            self.extract_cube_data(src.data)?,
        );

        Ok(Box::new([data]))
    }

    fn extract_column_data(&self, data: &[u8]) -> Result<Vec<u8>, Anvil2CCConversionError> {
        let tag = read_compressed(data)?;
        let output = self.extract_column_data_from_tag(tag)?;
        Ok(write_compressed(&output, true)?)
    }

    fn extract_column_data_from_tag(&self, tag: NbtCompound) -> Result<NbtCompound, Anvil2CCConversionError> {
        /*
         * Vanilla Chunk NBT structure:
         *
         * ROOT
         * |- DataVersion
         * |- Level
         *  |- v
         *  |- xPos
         *  |- zPos
         *  |- LastUpdate
         *  |- TerrainPopulated
         *  |- LightPopulated
         *  |- InhabitedTime
         *  |- Biomes
         *  |- HeightMap
         *  |- Sections
         *  ||* Section list:
         *  | |- Y
         *  | |- Blocks
         *  | |- Data
         *  | |- Add
         *  | |- BlockLight
         *  | |- SkyLight
         *  |- Entities
         *  |- TileEntities
         *  |- TileTicks
         *
         * CubicChunks Column format:
         *
         * ROOT
         * |- DataVersion
         * |- Level
         *  |- v
         *  |- x
         *  |- z
         *  |- InhabitedTime
         *  |- Biomes
         *  |- OpacityIndex
         */

        let mut level = NbtCompound::new();
        let src_level = tag.get::<_, &NbtCompound>("Level")?;

        let src_height_map = Self::fix_heightmap(src_level.get::<_, &Vec<i32>>("HeightMap")?);

        level.insert("v", Int(1));

        if src_level.get::<_, i32>("xPos").is_err() || src_level.get::<_, i32>("zPos").is_err() {
            return Err(Anvil2CCConversionError::from(InvalidChunkTagError {
                message: "Anvil chunk does not contain xPos or zPos!".to_string(),
            }));
        }

        level.insert("x", Int(src_level.get::<_, i32>("xPos")?));
        level.insert("z", Int(src_level.get::<_, i32>("zPos")?));
        level.insert("InhabitedTime", Int(src_level.get::<_, i32>("InhabitedTime").unwrap_or(0)));
        if let Ok(biomes) = src_level.get::<_, &Vec<i32>>("Biomes") {
            level.insert("Biomes", biomes.clone());
        }
        level.insert("OpacityIndex", ByteArray(Self::make_dummy_opacity_index(&src_height_map)?));

        let mut root = NbtCompound::new();
        root.insert("Level", level);
        if let Ok(data_version) = tag.get::<_, &i32>("DataVersion") {
            root.insert("DataVersion", *data_version);
        }

        Ok(root)
    }

    fn fix_heightmap(heights: &[i32]) -> Vec<i32> {
        let mut heights = heights.to_vec();
        for i in 0..heights.len() {
            heights[i] -= 1; // vanilla = 1 above top, data = top block
        }
        heights
    }

    fn make_dummy_opacity_index(height_map: &[i32]) -> Result<Vec<i8>, std::io::Error> {
        let mut out = Vec::new();

        for i in 0..256 {
            // 256 segment arrays
            out.write_i32::<LittleEndian>(0)?; // minY
            out.write_i32::<LittleEndian>(height_map[i])?; // maxY
            out.write_i16::<LittleEndian>(0)?; // no segments - write zero
        }
        Ok(vec_u8_into_i8(out))
    }

    fn extract_cube_data(&self, data: &[u8]) -> Result<Vec<(i32, Vec<u8>)>, Anvil2CCConversionError> {
        let tags = self.extract_cube_data_from_tag(read_compressed(data)?)?;
        let mut bytes_by_cube_y = Vec::new();
        for (y, tag) in tags {
            bytes_by_cube_y.push((y, write_compressed(&tag, false)?));
        }
        Ok(bytes_by_cube_y)
    }

    fn extract_cube_data_from_tag(&self, src_root: NbtCompound) -> Result<Vec<(i32, NbtCompound)>, Anvil2CCConversionError> {
        /*
         * Vanilla Chunk NBT structure:
         *
         * ROOT
         * |- DataVersion
         * |- Level
         *  |- v
         *  |- xPos
         *  |- zPos
         *  |- LastUpdate
         *  |- TerrainPopulated
         *  |- LightPopulated
         *  |- InhabitedTime
         *  |- Biomes
         *  |- HeightMap
         *  |- Sections
         *  ||* Section list:
         *  | |- Y
         *  | |- Blocks
         *  | |- Data
         *  | |- Add
         *  | |- BlockLight
         *  | |- SkyLight
         *  |- Entities
         *  |- TileEntities
         *  |- TileTicks
         *
         * CubicChunks Cube NBT structure:
         *
         * ROOT
         * |- DataVersion
         * |- Level
         *  |- v
         *  |- x
         *  |- y
         *  |- z
         *  |- populated
         *  |- fullyPopulated
         *  |- initLightDone
         *  |- isSurfaceTracked
         *  |- Sections
         *  ||* A single section
         *  | |- Blocks
         *  | |- Data
         *  | |- Add
         *  | |- BlockLight
         *  | |- SkyLight
         *  |- Entities
         *  |- TileEntities
         *  |- TileTicks
         *  |- LightingInfo
         *   |- LastHeightMap
         */
        let mut tags = HashMap::new();
        let data_version = match src_root.get::<_, &NbtTag>("DataVersion") {
            Ok(version) => Some(version.clone()),
            Err(_) => None,
        };
        let src_level = src_root.get::<_, &NbtCompound>("Level")?;

        let x = src_level.get::<_, i32>("xPos")?;
        let z = src_level.get::<_, i32>("zPos")?;

        let src_sections = src_level.get::<_, &NbtList>("Sections")?;
        for src_section in src_sections {
            let y = match src_section {
                Compound(section) => section.get::<_, u8>("Y"),
                _ => continue,
            }? as i32;

            let src_section: &NbtCompound = src_section.try_into()?;
            let mut root = NbtCompound::new();
            {
                if let Some(data_version) = &data_version {
                    root.insert("DataVersion", data_version.clone());
                }

                let mut level = NbtCompound::new();
                {
                    level.insert("v", Byte(1));
                    level.insert("x", Int(x));
                    level.insert("y", Int(y));
                    level.insert("z", Int(z));

                    let populated = src_level.get::<_, i8>("TerrainPopulated").unwrap_or(0);
                    level.insert("populated", Byte(populated));
                    level.insert("fullyPopulated", Byte(populated)); // TODO: handle this properly
                    level.insert("isSurfaceTracked", Byte(0)); // so that cubic chunks can re-make surface tracking data on it's own

                    let light_populated = src_level.get::<_, i8>("LightPopulated").unwrap_or(0);
                    level.insert("initLightDone", Byte(light_populated));

                    // the vanilla section has additional Y tag, it will be ignored by cubic chunks
                    let mut sections_tag = NbtList::new();

                    let mut sec = src_section.clone();
                    Self::fix_section(&mut sec)?;
                    sections_tag.push(sec);
                    level.insert("Sections", sections_tag);

                    level.insert("Entities", Self::filter_entities(src_level.get::<_, &NbtList>("Entities")?, y)?);
                    let mut tile_entities = Self::filter_tile_entities(src_level.get::<_, &NbtList>("TileEntities")?, y)?;
                    if self.fix_missing_tile_entities {
                        tile_entities = Self::add_missing_tile_entities(x, y, z, tile_entities, src_section)?;
                    }
                    level.insert("TileEntities", tile_entities);
                    if let Ok(tile_ticks) = src_level.get::<_, &NbtList>("TileTicks") {
                        level.insert("TileTicks", Self::filter_tile_ticks(tile_ticks, y)?);
                    }
                    level.insert("LightingInfo", Self::make_lighting_info(src_level)?);
                }
                root.insert("Level", level);
            }
            tags.insert(y, root);
        }

        // make sure the 0-15 range is there because it's using vanilla generator which expects it to be the case
        for y in 0..16 {
            tags.entry(y).or_insert_with(|| Self::empty_cube(x, y, z));
        }
        let tags: Vec<_> = tags.drain().map(|(y, tag)| (y, tag)).collect();
        Ok(tags)
    }

    fn add_missing_tile_entities(
        cube_x: i32,
        cube_y: i32,
        cube_z: i32,
        tile_entities: NbtList,
        section: &NbtCompound,
    ) -> Result<NbtList, Anvil2CCConversionError> {
        if !section.contains_key("Blocks") {
            return Ok(tile_entities);
        }

        let blocks = section.get::<_, &[i8]>("Blocks")?;
        let add = section.get::<_, &[i8]>("Add").ok();
        let add2neid = section.get::<_, &[i8]>("Add2").ok();

        let mut te_map: HashMap<i32, NbtCompound> = HashMap::new();
        for tag in tile_entities {
            let te: NbtCompound = tag.try_into()?;
            let x = te.get::<_, i32>("x").unwrap_or(0);
            let y = te.get::<_, i32>("y").unwrap_or(0);
            let z = te.get::<_, i32>("z").unwrap_or(0);
            let idx = y & 0xF << 8 | z & 0xF << 4 | x & 0xF;
            te_map.insert(idx, te);
        }
        for i in 0..4096 {
            let x = i & 15;
            let y = i >> 8 & 15;
            let z = i >> 4 & 15;

            let mut to_add = if let Some(add) = add { Self::get_nibble(add, i) } else { 0 };

            let asd = if let Some(add2neid) = add2neid {
                Self::get_nibble(add2neid, i) << 4
            } else {
                0
            };
            to_add = (to_add & 0xF) | asd;

            let id = (to_add << 8) | blocks[i as usize] as i32;
            let te_id = TE_REGISTRY.get(&(id as i32));
            if let Some(te_id) = te_id {
                te_map.entry(i).or_insert_with(|| {
                    let mut tag = NbtCompound::new();
                    tag.insert("id", String(te_id.to_string()));
                    tag.insert("x", Int(cube_x * 16 + x));
                    tag.insert("y", Int(cube_y * 16 + y));
                    tag.insert("z", Int(cube_z * 16 + z));
                    tag
                });
            }
        }
        let mut tile_entities_list = NbtList::new();
        te_map.drain().for_each(|(_, value)| {
            tile_entities_list.push(Compound(value));
        });
        Ok(tile_entities_list)
    }

    fn get_nibble(array: &[i8], i: i32) -> i32 {
        let v = array[(i >> 1) as usize];
        let shifted_value = if (i & 1) == 0 { v } else { v >> 4 };
        (shifted_value & 0xF) as i32
    }

    fn empty_cube(x: i32, y: i32, z: i32) -> NbtCompound {
        let mut root = NbtCompound::new();
        {
            let mut level = NbtCompound::new();
            {
                level.insert("v", Byte(1));
                level.insert("x", Int(x));
                level.insert("y", Int(y));
                level.insert("z", Int(z));

                level.insert("populated", Byte(1));
                level.insert("fullyPopulated", Byte(1));
                level.insert("isSurfaceTracked", Byte(1)); // it's empty, no need to re-track

                // no need for Sections, CC has isEmpty check for that

                level.insert("initLightDone", Byte(0));

                level.insert("Entities", NbtList::new());
                level.insert("TileEntities", NbtList::new());

                level.insert("LightingInfo", Self::make_empty_lighting_info());
            }
            root.insert("Level", level);
        }
        root
    }

    fn make_empty_lighting_info() -> NbtCompound {
        let heightmap = IntArray(Vec::with_capacity(256));
        let mut lighting_info = NbtCompound::new();
        lighting_info.insert("LastHeightMap", heightmap);
        lighting_info
    }

    fn fix_section(src_section: &mut NbtCompound) -> Result<(), NbtReprError> {
        let ids = src_section.get_mut::<_, &mut Vec<i8>>("Blocks")?;
        // TODO: handle it the forge way
        for i in 0..ids.len() {
            if ids[i] == 7 {
                // bedrock
                ids[i] = 1; // stone
            }
        }
        Ok(())
    }

    fn make_lighting_info(src_level: &NbtCompound) -> Result<NbtCompound, NbtReprError> {
        let heightmap = src_level.get::<_, &Vec<i32>>("HeightMap")?;
        let mut lighting_info_map = NbtCompound::new();
        lighting_info_map.insert("LastHeightMap", heightmap.clone());
        Ok(lighting_info_map)
    }

    fn filter_entities(entities: &NbtList, cube_y: i32) -> Result<NbtList, Anvil2CCConversionError> {
        let y_min = cube_y * 16;
        let y_max = y_min + 16;
        let mut cube_entities = NbtList::new();
        for entity in entities {
            let entity: &NbtCompound = entity.try_into()?;
            let pos = entity.get::<_, &NbtList>("Pos")?;
            let y: f64 = pos[1].clone().try_into()?;
            if y >= y_min as f64 && y < y_max as f64 {
                cube_entities.push(Compound(entity.clone()));
            }
        }
        Ok(cube_entities)
    }

    fn filter_tile_entities(tile_entities: &NbtList, cube_y: i32) -> Result<NbtList, Anvil2CCConversionError> {
        // empty list is list of EndTags
        if tile_entities.is_empty() {
            return Ok(tile_entities.clone());
        }
        let y_min = cube_y * 16;
        let y_max = y_min + 16;
        let mut cube_tes = NbtList::new();
        for tile_entity_tag in tile_entities {
            let tile_entity_tag: &NbtCompound = tile_entity_tag.try_into()?;
            let y = tile_entity_tag.get::<_, i32>("y")?;
            if y >= y_min && y < y_max {
                cube_tes.push(tile_entity_tag.clone());
            }
        }
        Ok(cube_tes)
    }

    fn filter_tile_ticks(tile_ticks: &NbtList, cube_y: i32) -> Result<NbtList, Anvil2CCConversionError> {
        let y_min = cube_y * 16;
        let y_max = y_min + 16;
        let mut cube_ticks = NbtList::new();
        for tile_tick_tag in tile_ticks {
            let tile_tick_tag: &NbtCompound = tile_tick_tag.try_into()?;
            let y = tile_tick_tag.get::<_, i32>("y")?;
            if y >= y_min && y < y_max {
                cube_ticks.push(tile_tick_tag.clone());
            }
        }
        Ok(cube_ticks)
    }
}
